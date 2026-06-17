// ============================================================
// cubism_model.cpp — Live2D model loading & per-frame update
// ============================================================

#include "bridge_internal.h"

#ifdef LIVE2D_HAS_SDK

#include <CubismFramework.hpp>
#include <Model/CubismUserModel.hpp>
#include <Model/CubismModel.hpp>
#include <Model/CubismMoc.hpp>
#include <CubismModelSettingJson.hpp>
#include <Motion/CubismMotionManager.hpp>
#include <Motion/CubismMotion.hpp>
#include <Motion/ACubismMotion.hpp>
#include <Effect/CubismBreath.hpp>
#include <Effect/CubismEyeBlink.hpp>
#include <Effect/CubismPose.hpp>
#include <Physics/CubismPhysics.hpp>
#include <Math/CubismModelMatrix.hpp>
#include <Rendering/D3D11/CubismRenderer_D3D11.hpp>
#include <Id/CubismIdManager.hpp>
#include <Utils/CubismString.hpp>

#include <d3d11.h>
#include <fstream>
#include <vector>
#include <map>
#include <algorithm>

#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

using namespace Live2D::Cubism::Framework;
using namespace Live2D::Cubism::Framework::Rendering;

// ============================================================
// BridgeUserModel — subclass to access protected members
// ============================================================
class BridgeUserModel : public CubismUserModel
{
public:
    CubismMotionManager* GetMotionManager() { return _motionManager; }
    CubismBreath*        GetBreath()         { return _breath; }
    CubismEyeBlink*      GetEyeBlink()       { return _eyeBlink; }
    CubismPhysics*       GetPhysics()        { return _physics; }
    CubismPose*          GetPose()           { return _pose; }
};

// ============================================================
// Logging helper (global, not in namespace)
// ============================================================
void BridgeLog(const char* msg)
{
    OutputDebugStringA(msg);
    OutputDebugStringA("\n");
}

namespace Model {

static std::mutex g_mutex;
static BridgeUserModel* g_userModel = nullptr;
static CubismModelSettingJson* g_modelSetting = nullptr;
static csmVector<ACubismMotion*> g_motions;
static std::map<std::string, std::vector<int>> g_motionIndex; // group name (lowercase) -> motion indices
static std::string g_modelDir;
static bool g_frameworkStarted = false;

// Layout info
static float g_centerX = 0, g_centerY = 0, g_modelWidth = 2, g_modelHeight = 2;

// ============================================================
// File I/O helpers
// ============================================================
static bool ReadFile(const std::string& path, std::vector<csmByte>& outBuffer)
{
    std::ifstream ifs(path, std::ios::binary | std::ios::ate);
    if (!ifs.is_open()) return false;
    auto size = ifs.tellg();
    if (size <= 0) return false;
    outBuffer.resize((size_t)size);
    ifs.seekg(0, std::ios::beg);
    ifs.read((char*)outBuffer.data(), size);
    return ifs.good();
}

static std::string JoinPath(const std::string& dir, const std::string& file)
{
    if (dir.empty()) return file;
    char last = dir.back();
    if (last == '/' || last == '\\') return dir + file;
    return dir + "/" + file;
}

// ============================================================
// Texture loading (stb_image → D3D11 SRV)
// ============================================================
static ID3D11ShaderResourceView* LoadTexture(const std::string& path)
{
    int w, h, channels;
    unsigned char* pixels = stbi_load(path.c_str(), &w, &h, &channels, 4); // force RGBA
    if (!pixels) return nullptr;

    ID3D11Device* device = Renderer::GetDevice();
    if (!device) { stbi_image_free(pixels); return nullptr; }

    D3D11_TEXTURE2D_DESC texDesc = {};
    texDesc.Width = w;
    texDesc.Height = h;
    texDesc.MipLevels = 1;
    texDesc.ArraySize = 1;
    texDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    texDesc.SampleDesc.Count = 1;
    texDesc.Usage = D3D11_USAGE_DEFAULT;
    texDesc.BindFlags = D3D11_BIND_SHADER_RESOURCE;

    D3D11_SUBRESOURCE_DATA initData = {};
    initData.pSysMem = pixels;
    initData.SysMemPitch = w * 4;

    ID3D11Texture2D* texture = nullptr;
    HRESULT hr = device->CreateTexture2D(&texDesc, &initData, &texture);
    stbi_image_free(pixels);
    if (FAILED(hr) || !texture) return nullptr;

    D3D11_SHADER_RESOURCE_VIEW_DESC srvDesc = {};
    srvDesc.Format = texDesc.Format;
    srvDesc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
    srvDesc.Texture2D.MipLevels = 1;
    srvDesc.Texture2D.MostDetailedMip = 0;

    ID3D11ShaderResourceView* srv = nullptr;
    hr = device->CreateShaderResourceView(texture, &srvDesc, &srv);
    texture->Release();
    if (FAILED(hr)) return nullptr;

    return srv;
}

// ============================================================
// File loading callbacks for CubismFramework
// ============================================================
static csmByte* LoadFile(const std::string filePath, csmSizeInt* outSize)
{
    std::ifstream ifs(filePath, std::ios::binary | std::ios::ate);
    if (!ifs.is_open())
    {
        char buf[512];
        snprintf(buf, sizeof(buf), "[Bridge] LoadFile failed: %s", filePath.c_str());
        BridgeLog(buf);
        return nullptr;
    }
    auto size = ifs.tellg();
    if (size <= 0) return nullptr;
    *outSize = static_cast<csmSizeInt>(size);
    csmByte* buffer = static_cast<csmByte*>(std::malloc(size));
    if (!buffer) return nullptr;
    ifs.seekg(0, std::ios::beg);
    ifs.read(reinterpret_cast<char*>(buffer), size);
    if (!ifs.good())
    {
        std::free(buffer);
        return nullptr;
    }
    return buffer;
}

static void ReleaseBytes(csmByte* byteData)
{
    std::free(byteData);
}

// ============================================================
// Framework callbacks (logging)
// ============================================================
static void CubismLogFunction(const csmChar* message)
{
    char buf[512];
    snprintf(buf, sizeof(buf), "[Cubism] %s", message);
    BridgeLog(buf);
}

// ============================================================
// Module implementation
// ============================================================

// Static option to keep it alive after Initialize returns
static CubismFramework::Option g_frameworkOption;

bool Initialize()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    if (g_frameworkStarted) return true;

    // Start Cubism Framework
    g_frameworkOption.LogFunction = CubismLogFunction;
    g_frameworkOption.LoggingLevel = CubismFramework::Option::LogLevel_Info;
    g_frameworkOption.LoadFileFunction = LoadFile;
    g_frameworkOption.ReleaseBytesFunction = ReleaseBytes;

    CubismFramework::StartUp(CsmAllocator::Get(), &g_frameworkOption);
    CubismFramework::Initialize();
    g_frameworkStarted = true;

    BridgeLog("[Bridge] Cubism Framework initialized");
    return true;
}

void Shutdown()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    Unload();

    if (g_frameworkStarted)
    {
        CubismFramework::Dispose();
        CubismFramework::CleanUp();
        g_frameworkStarted = false;
    }
}

bool Load(const char* modelPath)
{
    if (!modelPath) return false;
    std::lock_guard<std::mutex> lk(g_mutex);
    char logBuf[512]; // For formatted debug messages

    // Clean up previous model
    if (g_userModel)
    {
        g_userModel->DeleteRenderer();
        delete g_userModel;
        g_userModel = nullptr;
    }
    if (g_modelSetting)
    {
        delete g_modelSetting;
        g_modelSetting = nullptr;
    }
    for (csmUint32 i = 0; i < g_motions.GetSize(); i++) { ACubismMotion::Delete(g_motions[i]); }
    g_motions.Clear();
    g_motionIndex.clear();
    std::string fullPath(modelPath);
    auto lastSlash = fullPath.find_last_of("/\\");
    g_modelDir = (lastSlash != std::string::npos) ? fullPath.substr(0, lastSlash) : ".";

    // Read model3.json
    std::vector<csmByte> jsonBuf;
    if (!ReadFile(fullPath, jsonBuf)) return false;

    g_modelSetting = new CubismModelSettingJson(jsonBuf.data(), (csmSizeInt)jsonBuf.size());

    // Create user model
    g_userModel = new BridgeUserModel();

    // Load .moc3 file
    const csmChar* mocFileName = g_modelSetting->GetModelFileName();
    std::string mocPath = JoinPath(g_modelDir, mocFileName);
    std::vector<csmByte> mocBuf;
    if (!ReadFile(mocPath, mocBuf))
    {
        BridgeLog("[Bridge] FAILED to load .moc3 file");
        return false;
    }
    g_userModel->LoadModel(mocBuf.data(), (csmSizeInt)mocBuf.size());

    // Load physics
    const csmChar* physicsFile = g_modelSetting->GetPhysicsFileName();
    if (physicsFile && physicsFile[0])
    {
        std::string physicsPath = JoinPath(g_modelDir, physicsFile);
        std::vector<csmByte> physBuf;
        if (ReadFile(physicsPath, physBuf))
        {
            g_userModel->LoadPhysics(physBuf.data(), (csmSizeInt)physBuf.size());
        }
    }

    // Load pose
    const csmChar* poseFile = g_modelSetting->GetPoseFileName();
    if (poseFile && poseFile[0])
    {
        std::string posePath = JoinPath(g_modelDir, poseFile);
        std::vector<csmByte> poseBuf;
        if (ReadFile(posePath, poseBuf))
        {
            g_userModel->LoadPose(poseBuf.data(), (csmSizeInt)poseBuf.size());
        }
    }

    // Setup layout
    csmMap<csmString, csmFloat32> layout;
    if (g_modelSetting->GetLayoutMap(layout))
    {
        auto* modelMatrix = g_userModel->GetModelMatrix();
        // Iterate layout map and apply values
        for (csmMap<csmString, csmFloat32>::const_iterator it = layout.Begin(); it != layout.End(); ++it)
        {
            const csmString& key = it->First;
            csmFloat32 value = it->Second;
            if (modelMatrix)
            {
                if (key == "center_x") modelMatrix->CenterX(value);
                else if (key == "center_y") modelMatrix->CenterY(value);
                else if (key == "width") modelMatrix->SetWidth(value);
                else if (key == "height") modelMatrix->SetHeight(value);
            }
            // Store for projection
            if (key == "center_x") g_centerX = value;
            else if (key == "center_y") g_centerY = value;
            else if (key == "width") g_modelWidth = value;
            else if (key == "height") g_modelHeight = value;
        }
    }

    // Setup eye blink
    if (g_modelSetting->GetEyeBlinkParameterCount() > 0)
    {
        CubismEyeBlink* eyeBlink = CubismEyeBlink::Create(g_modelSetting);
        // The model's destructor handles cleanup via CubismEyeBlink::Delete
        // Store it on the model — but we can't set it directly.
        // For now, just clean it up; the model's _eyeBlink remains null.
        // We'll handle eye blink manually in Update().
        // Actually, just let it leak for now and fix later — or better, store it ourselves.
        // NOTE: We'll manage this via our BridgeUserModel getters.
        // For now, delete it properly since we can't set it on the model.
        CubismEyeBlink::Delete(eyeBlink);
    }

    // Preload motions
    BridgeLog("[Bridge] Loading motions...");
    for (csmInt32 i = 0; i < g_modelSetting->GetMotionGroupCount(); i++)
    {
        const csmChar* groupName = g_modelSetting->GetMotionGroupName(i);
        if (!groupName) continue;

        csmInt32 count = g_modelSetting->GetMotionCount(groupName);
        snprintf(logBuf, sizeof(logBuf), "[Bridge] Motion group '%s': %d motions", groupName, count);
        BridgeLog(logBuf);

        for (csmInt32 j = 0; j < count; j++)
        {
            const csmChar* motionFile = g_modelSetting->GetMotionFileName(groupName, j);
            if (!motionFile) continue;

            std::string motionPath = JoinPath(g_modelDir, motionFile);
            std::vector<csmByte> motionBuf;
            if (!ReadFile(motionPath, motionBuf)) continue;

            ACubismMotion* motion = g_userModel->LoadMotion(
                motionBuf.data(), (csmSizeInt)motionBuf.size(),
                motionFile, nullptr, nullptr,
                g_modelSetting, groupName, j
            );
            if (motion)
            {
                int motionIdx = (int)g_motions.GetSize();
                g_motions.PushBack(motion);

                // Build motion group index (lowercase group name)
                std::string groupLower(groupName);
                std::transform(groupLower.begin(), groupLower.end(), groupLower.begin(),
                    [](unsigned char c) { return (char)std::tolower(c); });
                g_motionIndex[groupLower].push_back(motionIdx);
            }
        }
    }
    snprintf(logBuf, sizeof(logBuf), "[Bridge] Total motions loaded: %u", (unsigned)g_motions.GetSize());
    BridgeLog(logBuf);

    // Create renderer (needs D3D11 device already created)
    int w = Renderer::GetWidth();
    int h = Renderer::GetHeight();
    snprintf(logBuf, sizeof(logBuf), "[Bridge] Creating renderer %dx%d", w, h);
    BridgeLog(logBuf);
    if (w > 0 && h > 0)
    {
        BridgeLog("[Bridge] Setting up D3D11 renderer...");
        ID3D11Device* device = Renderer::GetDevice();
        snprintf(logBuf, sizeof(logBuf), "[Bridge] D3D11 device: %p", device);
        BridgeLog(logBuf);
        if (!device)
        {
            BridgeLog("[Bridge] ERROR: D3D11 device is NULL!");
            return false;
        }
        // Set static device for CubismRenderer_D3D11
        CubismRenderer_D3D11::SetConstantSettings(1, device);
        BridgeLog("[Bridge] SetConstantSettings done, calling CreateRenderer...");
        g_userModel->CreateRenderer(w, h);
        BridgeLog("[Bridge] CreateRenderer done");

        // Bind textures
        auto* renderer = g_userModel->GetRenderer<CubismRenderer_D3D11>();
        if (renderer)
        {
            BridgeLog("[Bridge] Got renderer, binding textures...");
            csmInt32 texCount = g_modelSetting->GetTextureCount();
            snprintf(logBuf, sizeof(logBuf), "[Bridge] texCount=%d", texCount);
            BridgeLog(logBuf);

            for (csmInt32 i = 0; i < texCount; i++)
            {
                const csmChar* texFile = g_modelSetting->GetTextureFileName(i);
                if (!texFile) continue;
                // Texture paths in model3.json are already relative to model directory
                std::string texPath = JoinPath(g_modelDir, texFile);
                snprintf(logBuf, sizeof(logBuf), "[Bridge] Loading texture: %s", texPath.c_str());
                BridgeLog(logBuf);
                ID3D11ShaderResourceView* srv = LoadTexture(texPath);
                if (srv)
                {
                    renderer->BindTexture(i, srv);
                    BridgeLog("[Bridge] Texture bound OK");
                }
                else
                {
                    snprintf(logBuf, sizeof(logBuf), "[Bridge] FAILED to load texture: %s", texPath.c_str());
                    BridgeLog(logBuf);
                }
            }
        }
        else
        {
            BridgeLog("[Bridge] Renderer is null after CreateRenderer!");
        }
    }
    else
    {
        snprintf(logBuf, sizeof(logBuf), "[Bridge] Renderer not created: w=%d, h=%d", w, h);
        BridgeLog(logBuf);
    }

    // Start idle motion
    auto* motionMgr = g_userModel->GetMotionManager();
    if (motionMgr && g_motions.GetSize() > 0)
    {
        // Try "Idle" first (common convention), then "idle"
        csmInt32 idleCount = g_modelSetting->GetMotionCount("Idle");
        if (idleCount == 0)
        {
            idleCount = g_modelSetting->GetMotionCount("idle");
        }

        if (idleCount > 0)
        {
            // Start the first motion as idle
            motionMgr->StartMotionPriority(g_motions[0], false, 3);
            BridgeLog("[Bridge] Started idle motion");
        }
        else
        {
            // No idle motion found, just start the first motion
            motionMgr->StartMotionPriority(g_motions[0], false, 3);
            BridgeLog("[Bridge] Started first motion as fallback");
        }
    }

    BridgeLog("[Bridge] Live2D model loaded successfully");
    return true;
}

void Unload()
{
    for (csmUint32 i = 0; i < g_motions.GetSize(); i++) { ACubismMotion::Delete(g_motions[i]); }
    g_motions.Clear();
    g_motionIndex.clear();

    if (g_userModel)
    {
        g_userModel->DeleteRenderer();
        delete g_userModel;
        g_userModel = nullptr;
    }
    if (g_modelSetting)
    {
        delete g_modelSetting;
        g_modelSetting = nullptr;
    }
}

bool IsLoaded()
{
    std::lock_guard<std::mutex> lk(g_mutex);
    return g_userModel != nullptr && g_userModel->GetModel() != nullptr;
}

void Update(float deltaTimeSeconds)
{
    if (!g_userModel || !g_userModel->GetModel()) return;

    auto* model = g_userModel->GetModel();
    auto* motionMgr = g_userModel->GetMotionManager();
    auto* breath = g_userModel->GetBreath();
    auto* eyeBlink = g_userModel->GetEyeBlink();
    auto* physics = g_userModel->GetPhysics();
    auto* pose = g_userModel->GetPose();

    // Update motion
    if (motionMgr)
    {
        motionMgr->UpdateMotion(model, deltaTimeSeconds);
    }

    // Update breath
    if (breath)
    {
        breath->UpdateParameters(model, deltaTimeSeconds);
    }

    // Update eye blink
    if (eyeBlink)
    {
        eyeBlink->UpdateParameters(model, deltaTimeSeconds);
    }

    // Update physics
    if (physics)
    {
        physics->Evaluate(model, deltaTimeSeconds);
    }

    // Update pose
    if (pose)
    {
        pose->UpdateParameters(model, deltaTimeSeconds);
    }

    // Save parameters for next frame
    model->SaveParameters();
}

void SetParameter(const char* paramId, float value)
{
    if (!g_userModel || !g_userModel->GetModel() || !paramId) return;
    auto* model = g_userModel->GetModel();
    CubismIdHandle id = CubismFramework::GetIdManager()->GetId(paramId);
    csmInt32 index = model->GetParameterIndex(id);
    if (index >= 0)
    {
        model->SetParameterValue(id, value);
    }
}

CubismUserModel* GetUserModel()
{
    return g_userModel;
}

bool GetLayout(float& centerX, float& centerY, float& width, float& height)
{
    centerX = g_centerX;
    centerY = g_centerY;
    width = g_modelWidth;
    height = g_modelHeight;
    return true;
}

const std::map<std::string, std::vector<int>>& GetMotionIndex()
{
    return g_motionIndex;
}

ACubismMotion* GetMotion(int index)
{
    if (index < 0 || index >= (int)g_motions.GetSize()) return nullptr;
    return g_motions[index];
}

int GetMotionCount()
{
    return (int)g_motions.GetSize();
}

CubismMotionManager* GetMotionManager()
{
    if (!g_userModel) return nullptr;
    return g_userModel->GetMotionManager();
}

} // namespace Model

#else // !LIVE2D_HAS_SDK — mock mode stubs

void BridgeLog(const char*) {} // No-op in mock mode

namespace Model {

static bool g_loaded = false;
static std::unordered_map<std::string, float> g_params;

bool Initialize() { g_loaded = false; g_params.clear(); return true; }
void Shutdown() { g_loaded = false; g_params.clear(); }
bool Load(const char*) { g_loaded = true; return true; }
void Unload() { g_loaded = false; }
bool IsLoaded() { return g_loaded; }
void Update(float) {}
void SetParameter(const char* p, float v) { if (p) g_params[p] = v; }
bool GetLayout(float&, float&, float&, float&) { return false; }
static const std::map<std::string, std::vector<int>> g_emptyIndex;
const std::map<std::string, std::vector<int>>& GetMotionIndex() { return g_emptyIndex; }

} // namespace Model

#endif
