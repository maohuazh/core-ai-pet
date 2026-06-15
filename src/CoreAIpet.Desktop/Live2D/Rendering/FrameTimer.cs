using System.Windows.Media;

namespace CoreAIpet.Desktop.Live2D.Rendering;

/// <summary>
/// 60fps 渲染计时器 — 基于 CompositionTarget.Rendering
/// </summary>
public class FrameTimer : IDisposable
{
    private int _frameCount;
    private DateTime _lastFpsUpdate = DateTime.Now;
    private float _currentFps;
    private bool _disposed;

    public float CurrentFps => _currentFps;
    public event Action? Tick;

    public void Start()
    {
        CompositionTarget.Rendering += OnRendering;
    }

    public void Stop()
    {
        CompositionTarget.Rendering -= OnRendering;
    }

    private void OnRendering(object? sender, EventArgs e)
    {
        Tick?.Invoke();

        _frameCount++;
        var now = DateTime.Now;
        var elapsed = (now - _lastFpsUpdate).TotalSeconds;
        if (elapsed >= 1.0)
        {
            _currentFps = (float)(_frameCount / elapsed);
            _frameCount = 0;
            _lastFpsUpdate = now;
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        Stop();
    }
}
