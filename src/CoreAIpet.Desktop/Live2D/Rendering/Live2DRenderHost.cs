using CoreAIpet.Desktop.Live2D.Bridge;

namespace CoreAIpet.Desktop.Live2D.Rendering;

/// <summary>
/// Live2D 渲染宿主 — 编排 Bridge 初始化、模型加载、渲染循环、清理
/// </summary>
public class Live2DRenderHost : IDisposable
{
    private readonly Live2DBridgeWrapper _bridge;
    private readonly FrameTimer _frameTimer;
    private Live2DHostControl? _display;
    private bool _disposed;

    public float FPS => _bridge.FPS;

    public Live2DRenderHost(Live2DBridgeWrapper bridge)
    {
        _bridge = bridge;
        _frameTimer = new FrameTimer();
        _frameTimer.Tick += OnFrameTick;
    }

    private void OnFrameTick()
    {
        _bridge.Render();
        _display?.UpdateFromRenderer();
    }

    public bool Initialize(Live2DHostControl display, int width, int height)
    {
        _display = display;
        if (!_bridge.Initialize()) return false;
        // Initialize renderer without HWND (offscreen rendering)
        if (!_bridge.InitializeRenderer(IntPtr.Zero, width, height)) return false;
        display.Initialize(_bridge, width, height);
        return true;
    }

    public bool LoadModel(string modelPath)
    {
        return _bridge.LoadModel(modelPath);
    }

    public void UnloadModel()
    {
        _bridge.UnloadModel();
    }

    public void StartRendering()
    {
        _frameTimer.Start();
    }

    public void StopRendering()
    {
        _frameTimer.Stop();
    }

    public void SetMotion(string group, string name)
    {
        _bridge.SetMotion(group, name);
    }

    public void SetEyeTarget(float x, float y)
    {
        _bridge.SetEyeTarget(x, y);
    }

    public void Resize(int width, int height)
    {
        _bridge.Resize(width, height);
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        _frameTimer.Dispose();
        _bridge.Dispose();
    }
}
