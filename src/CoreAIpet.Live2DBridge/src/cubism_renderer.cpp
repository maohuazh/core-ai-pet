// ============================================================
// cubism_renderer.cpp — D3D11 offscreen rendering + pixel readback
// ============================================================
// Instead of rendering to a swap chain (which requires a visible HWND),
// we render to an offscreen texture, then copy pixels to a staging
// texture for CPU readback. C# WPF displays pixels via WriteableBitmap.
// This is needed because WPF AllowsTransparency + HwndHost is broken.
// ============================================================

#include "bridge_internal.h"
#include <d3d11.h>
#include <dxgi.h>
#include <cstring>

namespace Renderer {

static int               g_width      = 0;
static int               g_height     = 0;

static ID3D11Device*           g_device       = nullptr;
static ID3D11DeviceContext*    g_context      = nullptr;
static ID3D11Texture2D*        g_renderTex    = nullptr;
static ID3D11RenderTargetView* g_rtv          = nullptr;
static ID3D11Texture2D*        g_depthTex     = nullptr;
static ID3D11DepthStencilView* g_dsv          = nullptr;
static ID3D11DepthStencilState* g_depthState  = nullptr;
static ID3D11BlendState*        g_blendState  = nullptr;
static ID3D11Texture2D*        g_stagingTex   = nullptr;

static void ReleaseAll()
{
    if (g_dsv)        { g_dsv->Release();        g_dsv = nullptr; }
    if (g_depthTex)   { g_depthTex->Release();   g_depthTex = nullptr; }
    if (g_rtv)        { g_rtv->Release();        g_rtv = nullptr; }
    if (g_renderTex)  { g_renderTex->Release();  g_renderTex = nullptr; }
    if (g_stagingTex) { g_stagingTex->Release(); g_stagingTex = nullptr; }
}

static bool CreateTextures()
{
    if (!g_device || g_width <= 0 || g_height <= 0) return false;
    HRESULT hr;

    // Render target texture (RGBA8, renderable)
    D3D11_TEXTURE2D_DESC rtDesc = {};
    rtDesc.Width = g_width;
    rtDesc.Height = g_height;
    rtDesc.MipLevels = 1;
    rtDesc.ArraySize = 1;
    rtDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    rtDesc.SampleDesc.Count = 1;
    rtDesc.Usage = D3D11_USAGE_DEFAULT;
    rtDesc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;
    hr = g_device->CreateTexture2D(&rtDesc, nullptr, &g_renderTex);
    if (FAILED(hr)) return false;

    hr = g_device->CreateRenderTargetView(g_renderTex, nullptr, &g_rtv);
    if (FAILED(hr)) return false;

    // Depth stencil texture
    D3D11_TEXTURE2D_DESC depthDesc = {};
    depthDesc.Width = g_width;
    depthDesc.Height = g_height;
    depthDesc.MipLevels = 1;
    depthDesc.ArraySize = 1;
    depthDesc.Format = DXGI_FORMAT_D24_UNORM_S8_UINT;
    depthDesc.SampleDesc.Count = 1;
    depthDesc.Usage = D3D11_USAGE_DEFAULT;
    depthDesc.BindFlags = D3D11_BIND_DEPTH_STENCIL;
    hr = g_device->CreateTexture2D(&depthDesc, nullptr, &g_depthTex);
    if (FAILED(hr)) return false;

    D3D11_DEPTH_STENCIL_VIEW_DESC dsvDesc = {};
    dsvDesc.Format = depthDesc.Format;
    dsvDesc.ViewDimension = D3D11_DSV_DIMENSION_TEXTURE2D;
    hr = g_device->CreateDepthStencilView(g_depthTex, &dsvDesc, &g_dsv);
    if (FAILED(hr)) return false;

    // Staging texture for CPU readback
    D3D11_TEXTURE2D_DESC stagingDesc = {};
    stagingDesc.Width = g_width;
    stagingDesc.Height = g_height;
    stagingDesc.MipLevels = 1;
    stagingDesc.ArraySize = 1;
    stagingDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    stagingDesc.SampleDesc.Count = 1;
    stagingDesc.Usage = D3D11_USAGE_STAGING;
    stagingDesc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    hr = g_device->CreateTexture2D(&stagingDesc, nullptr, &g_stagingTex);
    if (FAILED(hr)) return false;

    return true;
}

bool Initialize(HWND /*hwnd*/, int width, int height)
{
    if (width <= 0 || height <= 0) return false;
    g_width = width;
    g_height = height;

    HRESULT hr;

    // Create D3D11 device (no swap chain)
    D3D_FEATURE_LEVEL featureLevel;
    D3D_FEATURE_LEVEL featureLevels[] = { D3D_FEATURE_LEVEL_11_0 };

    hr = D3D11CreateDevice(
        nullptr,
        D3D_DRIVER_TYPE_HARDWARE,
        nullptr,
        0,
        featureLevels,
        1,
        D3D11_SDK_VERSION,
        &g_device,
        &featureLevel,
        &g_context
    );
    if (FAILED(hr)) return false;

    // Create render target + depth + staging textures
    if (!CreateTextures()) return false;

    // Depth stencil state (depth disabled)
    D3D11_DEPTH_STENCIL_DESC depthDescState = {};
    depthDescState.DepthEnable = FALSE;
    depthDescState.DepthWriteMask = D3D11_DEPTH_WRITE_MASK_ALL;
    depthDescState.DepthFunc = D3D11_COMPARISON_LESS;
    depthDescState.StencilEnable = FALSE;
    hr = g_device->CreateDepthStencilState(&depthDescState, &g_depthState);
    if (FAILED(hr)) return false;

    // Alpha blend state
    D3D11_BLEND_DESC blendDesc = {};
    blendDesc.AlphaToCoverageEnable = FALSE;
    blendDesc.IndependentBlendEnable = FALSE;
    blendDesc.RenderTarget[0].BlendEnable = TRUE;
    blendDesc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
    blendDesc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
    blendDesc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    blendDesc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
    blendDesc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_INV_SRC_ALPHA;
    blendDesc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    blendDesc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL;
    hr = g_device->CreateBlendState(&blendDesc, &g_blendState);
    if (FAILED(hr)) return false;

    return true;
}

void Shutdown()
{
    ReleaseAll();
    if (g_blendState)  { g_blendState->Release();  g_blendState = nullptr; }
    if (g_depthState)  { g_depthState->Release();  g_depthState = nullptr; }
    if (g_context)     { g_context->Release();     g_context = nullptr; }
    if (g_device)      { g_device->Release();      g_device = nullptr; }
}

void Resize(int width, int height)
{
    if (width <= 0 || height <= 0) return;
    if (width == g_width && height == g_height) return;
    g_width = width;
    g_height = height;

    ReleaseAll();
    CreateTextures();
}

void BeginFrame()
{
    if (!g_context || !g_rtv) return;

    const float clearColor[4] = { 0.0f, 0.0f, 0.0f, 0.0f }; // transparent
    g_context->OMSetRenderTargets(1, &g_rtv, g_dsv);
    g_context->ClearRenderTargetView(g_rtv, clearColor);
    if (g_dsv)
    {
        g_context->ClearDepthStencilView(g_dsv, D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL, 1.0f, 0);
    }
    if (g_depthState)
    {
        g_context->OMSetDepthStencilState(g_depthState, 0);
    }
    if (g_blendState)
    {
        const float blendFactor[4] = { 1.0f, 1.0f, 1.0f, 1.0f };
        g_context->OMSetBlendState(g_blendState, blendFactor, 0xFFFFFFFF);
    }

    // Set viewport
    D3D11_VIEWPORT vp = {};
    vp.TopLeftX = 0;
    vp.TopLeftY = 0;
    vp.Width = (float)g_width;
    vp.Height = (float)g_height;
    vp.MinDepth = 0.0f;
    vp.MaxDepth = 1.0f;
    g_context->RSSetViewports(1, &vp);

#ifdef LIVE2D_HAS_SDK
    // Notify Cubism renderer of new frame
    auto* userModel = Model::GetUserModel();
    if (userModel)
    {
        auto* renderer = userModel->GetRenderer<Live2D::Cubism::Framework::Rendering::CubismRenderer_D3D11>();
        if (renderer)
        {
            renderer->StartFrame(g_context);
        }
    }
#endif
}

void EndFrame()
{
#ifdef LIVE2D_HAS_SDK
    auto* userModel = Model::GetUserModel();
    if (userModel)
    {
        auto* renderer = userModel->GetRenderer<Live2D::Cubism::Framework::Rendering::CubismRenderer_D3D11>();
        if (renderer)
        {
            renderer->EndFrame();
        }
    }
#endif
    // No swap chain present — pixels are read via ReadPixels()
}

/// Copy render target to staging texture and return pointer to pixel data.
/// Returns nullptr if readback fails. Caller must call UnlockPixels() when done.
static D3D11_MAPPED_SUBRESOURCE g_mapped = {};
static bool g_mappedValid = false;

const void* ReadPixels()
{
    if (!g_context || !g_renderTex || !g_stagingTex) return nullptr;

    // Copy render target to staging
    g_context->CopyResource(g_stagingTex, g_renderTex);

    // Map staging texture for reading
    HRESULT hr = g_context->Map(g_stagingTex, 0, D3D11_MAP_READ, 0, &g_mapped);
    if (FAILED(hr))
    {
        g_mappedValid = false;
        return nullptr;
    }
    g_mappedValid = true;
    return g_mapped.pData;
}

void UnlockPixels()
{
    if (g_mappedValid && g_stagingTex)
    {
        g_context->Unmap(g_stagingTex, 0);
        g_mappedValid = false;
    }
}

int GetPixelStride()
{
    return g_mapped.RowPitch;
}

ID3D11Device* GetDevice() { return g_device; }
ID3D11DeviceContext* GetDeviceContext() { return g_context; }
int GetWidth() { return g_width; }
int GetHeight() { return g_height; }
HWND GetHwnd() { return nullptr; }

} // namespace Renderer
