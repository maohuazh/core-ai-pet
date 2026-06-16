// ============================================================
// cubism_renderer.cpp — D3D11 device, swap chain, render target
// ============================================================

#include "bridge_internal.h"
#include <d3d11.h>
#include <dxgi.h>

namespace Renderer {

static HWND              g_hwnd       = nullptr;
static int               g_width      = 0;
static int               g_height     = 0;

static ID3D11Device*           g_device       = nullptr;
static ID3D11DeviceContext*    g_context      = nullptr;
static IDXGISwapChain*         g_swapChain    = nullptr;
static ID3D11RenderTargetView* g_rtv          = nullptr;
static ID3D11Texture2D*        g_depthTex     = nullptr;
static ID3D11DepthStencilView* g_dsv          = nullptr;
static ID3D11DepthStencilState* g_depthState  = nullptr;

static bool CreateRenderTarget()
{
    if (!g_swapChain || !g_device) return false;

    HRESULT hr;
    ID3D11Texture2D* backBuffer = nullptr;
    hr = g_swapChain->GetBuffer(0, __uuidof(ID3D11Texture2D), (void**)&backBuffer);
    if (FAILED(hr)) return false;

    hr = g_device->CreateRenderTargetView(backBuffer, nullptr, &g_rtv);
    backBuffer->Release();
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

    return true;
}

static void ReleaseRenderTarget()
{
    if (g_dsv)      { g_dsv->Release();      g_dsv = nullptr; }
    if (g_depthTex)  { g_depthTex->Release();  g_depthTex = nullptr; }
    if (g_rtv)      { g_rtv->Release();      g_rtv = nullptr; }
}

bool Initialize(HWND hwnd, int width, int height)
{
    if (!hwnd || width <= 0 || height <= 0) return false;
    g_hwnd = hwnd;
    g_width = width;
    g_height = height;

    HRESULT hr;

    // Swap chain description
    DXGI_SWAP_CHAIN_DESC scDesc = {};
    scDesc.BufferCount = 2;
    scDesc.BufferDesc.Width = width;
    scDesc.BufferDesc.Height = height;
    scDesc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    scDesc.BufferDesc.RefreshRate.Numerator = 60;
    scDesc.BufferDesc.RefreshRate.Denominator = 1;
    scDesc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    scDesc.SampleDesc.Count = 1;
    scDesc.SampleDesc.Quality = 0;
    scDesc.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
    scDesc.OutputWindow = hwnd;
    scDesc.Windowed = TRUE;
    scDesc.Flags = 0;

    D3D_FEATURE_LEVEL featureLevel;
    D3D_FEATURE_LEVEL featureLevels[] = { D3D_FEATURE_LEVEL_11_0 };

    UINT createFlags = 0;
#ifdef _DEBUG
    createFlags |= D3D11_CREATE_DEVICE_DEBUG;
#endif

    hr = D3D11CreateDeviceAndSwapChain(
        nullptr,
        D3D_DRIVER_TYPE_HARDWARE,
        nullptr,
        createFlags,
        featureLevels,
        1,
        D3D11_SDK_VERSION,
        &scDesc,
        &g_swapChain,
        &g_device,
        &featureLevel,
        &g_context
    );

    if (FAILED(hr))
    {
        // Retry without debug flag
        hr = D3D11CreateDeviceAndSwapChain(
            nullptr,
            D3D_DRIVER_TYPE_HARDWARE,
            nullptr,
            0,
            featureLevels,
            1,
            D3D11_SDK_VERSION,
            &scDesc,
            &g_swapChain,
            &g_device,
            &featureLevel,
            &g_context
        );
        if (FAILED(hr)) return false;
    }

    // Create render target + depth stencil
    if (!CreateRenderTarget()) return false;

    // Depth stencil state (depth disabled — we use painter's algorithm)
    D3D11_DEPTH_STENCIL_DESC depthDescState = {};
    depthDescState.DepthEnable = FALSE;
    depthDescState.DepthWriteMask = D3D11_DEPTH_WRITE_MASK_ALL;
    depthDescState.DepthFunc = D3D11_COMPARISON_LESS;
    depthDescState.StencilEnable = FALSE;
    hr = g_device->CreateDepthStencilState(&depthDescState, &g_depthState);
    if (FAILED(hr)) return false;

    return true;
}

void Shutdown()
{
    ReleaseRenderTarget();
    if (g_depthState) { g_depthState->Release(); g_depthState = nullptr; }
    if (g_swapChain)  { g_swapChain->Release();  g_swapChain = nullptr; }
    if (g_context)    { g_context->Release();     g_context = nullptr; }
    if (g_device)     { g_device->Release();      g_device = nullptr; }
    g_hwnd = nullptr;
}

void Resize(int width, int height)
{
    if (width <= 0 || height <= 0) return;
    if (width == g_width && height == g_height) return;
    g_width = width;
    g_height = height;

    if (!g_swapChain) return;

    ReleaseRenderTarget();

    HRESULT hr = g_swapChain->ResizeBuffers(
        2, width, height,
        DXGI_FORMAT_R8G8B8A8_UNORM, 0
    );
    if (SUCCEEDED(hr))
    {
        CreateRenderTarget();
    }
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

    if (g_swapChain)
    {
        g_swapChain->Present(1, 0);
    }
}

ID3D11Device* GetDevice() { return g_device; }
ID3D11DeviceContext* GetDeviceContext() { return g_context; }
int GetWidth() { return g_width; }
int GetHeight() { return g_height; }
HWND GetHwnd() { return g_hwnd; }

} // namespace Renderer
