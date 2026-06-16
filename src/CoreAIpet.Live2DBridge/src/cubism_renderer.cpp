// ============================================================
// cubism_renderer.cpp — D3D11 渲染器（带 GDI 占位后备）
// ------------------------------------------------------------
// 未启用 SDK 时：使用 GDI 绘制一个带动画的占位角色（圆脸+眼睛），
// 验证 HWND / 渲染循环 / resize 均工作正常。
// 启用 SDK 后：改用 D3D11 + CubismRenderer 渲染真实模型。
// ============================================================

#include "bridge_internal.h"
#include <d3d11.h>
#include <cmath>

namespace Renderer {

static HWND g_hwnd = nullptr;
static int g_width = 0;
static int g_height = 0;

// D3D11 对象（SDK 模式下启用）
#ifdef LIVE2D_HAS_SDK
static ID3D11Device* g_device = nullptr;
static ID3D11DeviceContext* g_context = nullptr;
static IDXGISwapChain* g_swapChain = nullptr;
static ID3D11RenderTargetView* g_rtv = nullptr;
#endif

bool Initialize(HWND hwnd, int width, int height)
{
    if (!hwnd || width <= 0 || height <= 0) return false;
    g_hwnd = hwnd;
    g_width = width;
    g_height = height;

#ifdef LIVE2D_HAS_SDK
    // TODO: 创建 D3D11 device + swap chain + RTV，绑定到 hwnd
    // D3D11CreateDeviceAndSwapChain(...)
#endif
    return true;
}

void Shutdown()
{
#ifdef LIVE2D_HAS_SDK
    if (g_rtv) { g_rtv->Release(); g_rtv = nullptr; }
    if (g_swapChain) { g_swapChain->Release(); g_swapChain = nullptr; }
    if (g_context) { g_context->Release(); g_context = nullptr; }
    if (g_device) { g_device->Release(); g_device = nullptr; }
#endif
    g_hwnd = nullptr;
}

void Resize(int width, int height)
{
    if (width <= 0 || height <= 0) return;
    g_width = width;
    g_height = height;
#ifdef LIVE2D_HAS_SDK
    // TODO: g_swapChain->ResizeBuffers(...)，重建 RTV
#endif
}

void BeginFrame()
{
#ifdef LIVE2D_HAS_SDK
    if (!g_context || !g_rtv) return;
    const float clear[4] = { 0, 0, 0, 0 };  // 透明背景
    g_context->ClearRenderTargetView(g_rtv, clear);
#endif
}

void EndFrame()
{
#ifdef LIVE2D_HAS_SDK
    if (g_swapChain) g_swapChain->Present(1, 0);
#endif
}

// 占位渲染：使用 Win32 GDI 在 HWND 上画一个「脸 + 眼睛」。
// 眼睛位置受 Animation 与 EyeTracking 参数影响。
void DrawPlaceholder()
{
    if (!g_hwnd) return;

    HDC hdc = ::GetDC(g_hwnd);
    if (!hdc) return;

    // 双缓冲
    HDC memDC = ::CreateCompatibleDC(hdc);
    HBITMAP bmp = ::CreateCompatibleBitmap(hdc, g_width, g_height);
    HBITMAP oldBmp = (HBITMAP)::SelectObject(memDC, bmp);

    RECT rc{ 0, 0, g_width, g_height };
    // 背景：淡蓝渐变（模拟透明桌面区域）
    HBRUSH bg = ::CreateSolidBrush(RGB(240, 245, 250));
    ::FillRect(memDC, &rc, bg);
    ::DeleteObject(bg);

    // 动画相位（使用 tick 驱动）
    DWORD t = ::GetTickCount();
    float phase = (float)(t % 3000) / 3000.0f * 2.0f * 3.14159f;
    float bob = std::sin(phase) * 4.0f;  // 上下浮动 ±4px

    // 根据 motion group 决定脸色
    COLORREF faceColor = RGB(255, 224, 189);  // 默认 Idle: 米色
    const auto& mot = Animation::Current();
    if (mot.group == "happy")         faceColor = RGB(255, 200, 200);
    else if (mot.group == "thinking") faceColor = RGB(220, 220, 255);
    else if (mot.group == "talking")  faceColor = RGB(255, 230, 180);

    // 脸：圆
    int cx = g_width / 2;
    int cy = g_height / 2 + (int)bob;
    int radius = (g_width < g_height ? g_width : g_height) / 2 - 10;
    if (radius < 20) radius = 20;

    HBRUSH faceBrush = ::CreateSolidBrush(faceColor);
    HPEN pen = ::CreatePen(PS_SOLID, 2, RGB(80, 80, 80));
    HBRUSH oldBr = (HBRUSH)::SelectObject(memDC, faceBrush);
    HPEN oldPen = (HPEN)::SelectObject(memDC, pen);
    ::Ellipse(memDC, cx - radius, cy - radius, cx + radius, cy + radius);
    ::SelectObject(memDC, oldBr);
    ::SelectObject(memDC, oldPen);
    ::DeleteObject(faceBrush);
    ::DeleteObject(pen);

    // 眼球追踪偏移（像素）
    float eyeTx = 0.0f, eyeTy = 0.0f;
    EyeTracking::GetTarget(eyeTx, eyeTy);
    int eyeOffX = (int)(eyeTx * radius * 0.25f);  // 水平 ±25% 半径
    int eyeOffY = (int)(-eyeTy * radius * 0.20f);

    // 嘴巴（说话时张合）
    int mouthOpen = 0;
    if (mot.group == "talking") {
        mouthOpen = (int)(std::sin(phase * 4.0) * 6.0 + 6.0);
    }

    // 眼睛：黑色小圆
    int eyeR = radius / 8;
    int eyeY = cy - radius / 5;
    int eyeSpacing = radius / 2;
    HBRUSH eyeBr = ::CreateSolidBrush(RGB(30, 30, 30));
    ::SelectObject(memDC, eyeBr);
    ::Ellipse(memDC, cx - eyeSpacing - eyeR + eyeOffX, eyeY - eyeR + eyeOffY,
                   cx - eyeSpacing + eyeR + eyeOffX, eyeY + eyeR + eyeOffY);
    ::Ellipse(memDC, cx + eyeSpacing - eyeR + eyeOffX, eyeY - eyeR + eyeOffY,
                   cx + eyeSpacing + eyeR + eyeOffX, eyeY + eyeR + eyeOffY);
    ::DeleteObject(eyeBr);

    // 嘴
    if (mouthOpen > 2) {
        HBRUSH mouthBr = ::CreateSolidBrush(RGB(180, 60, 60));
        ::SelectObject(memDC, mouthBr);
        ::Ellipse(memDC, cx - mouthOpen, cy + radius / 3 - mouthOpen / 2,
                       cx + mouthOpen, cy + radius / 3 + mouthOpen / 2);
        ::DeleteObject(mouthBr);
    } else {
        HPEN mouthPen = ::CreatePen(PS_SOLID, 2, RGB(80, 40, 40));
        ::SelectObject(memDC, mouthPen);
        ::MoveToEx(memDC, cx - radius / 5, cy + radius / 3, nullptr);
        ::LineTo(memDC, cx + radius / 5, cy + radius / 3);
        ::DeleteObject(mouthPen);
    }

    // 状态文字（左上角调试）
    ::SetBkMode(memDC, TRANSPARENT);
    ::SetTextColor(memDC, RGB(80, 80, 80));
    char label[128];
    std::snprintf(label, sizeof(label), "MOCK | state=%s  fps=%.0f",
                  mot.group.empty() ? "idle" : mot.group.c_str(), FpsCounter::Value());
    ::TextOutA(memDC, 6, 6, label, (int)std::strlen(label));

    ::BitBlt(hdc, 0, 0, g_width, g_height, memDC, 0, 0, SRCCOPY);
    ::SelectObject(memDC, oldBmp);
    ::DeleteObject(bmp);
    ::DeleteDC(memDC);
    ::ReleaseDC(g_hwnd, hdc);
}

int GetWidth() { return g_width; }
int GetHeight() { return g_height; }
HWND GetHwnd() { return g_hwnd; }

} // namespace Renderer
