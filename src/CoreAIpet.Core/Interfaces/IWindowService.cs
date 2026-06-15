namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 窗口操作服务 — 封装 Win32 窗口操作
/// </summary>
public interface IWindowService
{
    /// <summary>显示主窗口</summary>
    void Show();

    /// <summary>隐藏主窗口</summary>
    void Hide();

    /// <summary>切换点击穿透模式</summary>
    void ToggleClickThrough();

    /// <summary>设置点击穿透</summary>
    void SetClickThrough(bool enabled);

    /// <summary>设置窗口置顶</summary>
    void SetAlwaysOnTop(bool enabled);

    /// <summary>设置窗口缩放 (50~200)</summary>
    void SetScale(double scalePercent);

    /// <summary>设置窗口透明度 (20~100)</summary>
    void SetOpacity(double opacityPercent);
}
