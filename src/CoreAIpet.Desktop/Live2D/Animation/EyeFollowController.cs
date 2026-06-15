using System.Windows;
using System.Windows.Input;
using CoreAIpet.Desktop.Live2D.Rendering;
using CoreAIpet.Desktop.Win32;

namespace CoreAIpet.Desktop.Live2D.Animation;

/// <summary>
/// 眼球追踪控制器 — 鼠标位置 → 归一化坐标 → Bridge_SetEyeTarget
/// 水平 ±30°, 垂直 ±15°
/// </summary>
public class EyeFollowController
{
    private readonly Live2DRenderHost _renderHost;

    // 角度限制归一化值 (实际渲染时由 C++ 侧限制)
    private const double MaxHorizontal = 1.0;
    private const double MaxVertical = 0.5; // 垂直范围较小

    public EyeFollowController(Live2DRenderHost renderHost)
    {
        _renderHost = renderHost;
    }

    public void Update()
    {
        if (!NativeMethods.GetCursorPos(out var cursorPos))
            return;

        // 获取主屏幕尺寸
        var screenWidth = SystemParameters.PrimaryScreenWidth;
        var screenHeight = SystemParameters.PrimaryScreenHeight;

        // 归一化到 -1.0 ~ 1.0
        var normalizedX = (cursorPos.X / screenWidth) * 2.0 - 1.0;
        var normalizedY = (cursorPos.Y / screenHeight) * 2.0 - 1.0;

        // 应用范围限制
        normalizedX = Math.Clamp(normalizedX, -MaxHorizontal, MaxHorizontal);
        normalizedY = Math.Clamp(normalizedY, -MaxVertical, MaxVertical);

        _renderHost.SetEyeTarget((float)normalizedX, (float)normalizedY);
    }
}
