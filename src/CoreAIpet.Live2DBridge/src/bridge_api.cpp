// ============================================================
// bridge_api.cpp — Exported C functions (connects all modules)
// ============================================================

#include "bridge_api.h"
#include "bridge_internal.h"
#include <chrono>

static bool g_initialized = false;
static DWORD g_lastTick = 0;

// ============================================================
// FPS counter
// ============================================================
namespace FpsCounter {
static DWORD g_fpsLastTick = 0;
static int   g_frames = 0;
static float g_fps = 0.0f;

void Tick()
{
    DWORD now = ::GetTickCount();
    ++g_frames;
    if (g_fpsLastTick == 0) g_fpsLastTick = now;
    DWORD dt = now - g_fpsLastTick;
    if (dt >= 500) {
        g_fps = (float)g_frames * 1000.0f / (float)dt;
        g_frames = 0;
        g_fpsLastTick = now;
    }
}

float Value() { return g_fps; }
}

// ============================================================
// Initialize / Shutdown
// ============================================================

bool Bridge_Initialize(const char* /*sdkKey*/)
{
    if (g_initialized) return true;
    if (!Model::Initialize()) return false;
    Animation::Initialize();
    g_initialized = true;
    g_lastTick = ::GetTickCount();
    return true;
}

void Bridge_Shutdown()
{
    if (!g_initialized) return;
    Bridge_UnloadModel();
    Renderer::Shutdown();
    Model::Shutdown();
    g_initialized = false;
}

// ============================================================
// Model management
// ============================================================

bool Bridge_LoadModel(const char* modelPath)
{
    if (!g_initialized) return false;
    if (!modelPath) return false;
    if (!Model::Load(modelPath)) return false;
    Animation::SetMotion("idle", "idle_00");
    return true;
}

void Bridge_UnloadModel()
{
    Model::Unload();
}

// ============================================================
// Rendering
// ============================================================

bool Bridge_InitializeRenderer(HWND hwnd, int width, int height)
{
    // hwnd is ignored — offscreen rendering doesn't need a window
    if (width <= 0)  width  = 150;
    if (height <= 0) height = 150;

    if (!Renderer::Initialize(hwnd, width, height)) return false;

#ifdef LIVE2D_HAS_SDK
    // If model is already loaded, create renderer for it
    auto* userModel = Model::GetUserModel();
    if (userModel && userModel->GetModel())
    {
        // Set static device for CubismRenderer_D3D11
        Live2D::Cubism::Framework::Rendering::CubismRenderer_D3D11::SetConstantSettings(
            1, Renderer::GetDevice());

        userModel->CreateRenderer(width, height);
    }
#endif

    return true;
}

void Bridge_Render()
{
    if (!Model::IsLoaded()) return;

    // Calculate delta time
    DWORD now = ::GetTickCount();
    float dt = (now - g_lastTick) / 1000.0f;
    if (dt > 0.1f) dt = 0.1f; // clamp to prevent huge jumps
    g_lastTick = now;

    // 1. Update model parameters (motions, breath, physics, etc.)
    Model::Update(dt);

    // 2. Begin frame (clear + set render targets)
    Renderer::BeginFrame();

#ifdef LIVE2D_HAS_SDK
    // 3. Draw model
    auto* userModel = Model::GetUserModel();
    if (userModel && userModel->GetModel())
    {
        auto* renderer = userModel->GetRenderer<
            Live2D::Cubism::Framework::Rendering::CubismRenderer_D3D11>();
        if (renderer)
        {
            // Setup projection matrix
            int w = Renderer::GetWidth();
            int h = Renderer::GetHeight();
            if (w > 0 && h > 0)
            {
                Live2D::Cubism::Framework::CubismMatrix44 projection;
                // Identity projection — model matrix handles coordinate mapping
                // (Cubism standard: model coords map directly to NDC)

                // Apply model matrix (contains layout from model3.json)
                auto* modelMatrix = userModel->GetModelMatrix();
                if (modelMatrix)
                {
                    // Ensure model fills vertical space and is centered (model coords are ±1 = 2 units)
                    modelMatrix->SetHeight(2.0f);
                    modelMatrix->CenterX(0.0f);
                    modelMatrix->CenterY(0.0f);
                    projection.MultiplyByMatrix(modelMatrix);
                }

                renderer->SetMvpMatrix(&projection);
                renderer->DrawModel();
            }
        }
    }
#endif

    // 4. End frame (present)
    Renderer::EndFrame();

    // 5. FPS
    FpsCounter::Tick();
}

void Bridge_Resize(int width, int height)
{
    Renderer::Resize(width, height);

#ifdef LIVE2D_HAS_SDK
    // Notify model's renderer of new size
    auto* userModel = Model::GetUserModel();
    if (userModel)
    {
        userModel->SetRenderTargetSize(width, height);
    }
#endif
}

// ============================================================
// Animation / State
// ============================================================

void Bridge_SetMotionGroup(const char* group, const char* name)
{
    if (!g_initialized) return;
    Animation::SetMotion(group, name);
}

void Bridge_SetParameter(const char* paramId, float value)
{
    if (!g_initialized) return;
    Model::SetParameter(paramId, value);
}

// ============================================================
// Eye tracking
// ============================================================

void Bridge_SetEyeTarget(float x, float y)
{
    if (!g_initialized) return;
    EyeTracking::Apply(x, y);
}

// ============================================================
// Info
// ============================================================

float Bridge_GetFPS()
{
    return FpsCounter::Value();
}

// ============================================================
// Pixel readback (for WPF WriteableBitmap display)
// ============================================================

extern "C" __declspec(dllexport)
const void* Bridge_ReadPixels()
{
    return Renderer::ReadPixels();
}

extern "C" __declspec(dllexport)
void Bridge_UnlockPixels()
{
    Renderer::UnlockPixels();
}

extern "C" __declspec(dllexport)
int Bridge_GetPixelStride()
{
    return Renderer::GetPixelStride();
}
