using CoreAIpet.Core.Exceptions;

namespace CoreAIpet.Desktop.Live2D.Bridge;

/// <summary>
/// Live2D Bridge 安全封装 — 异常转换、线程安全、生命周期管理
/// </summary>
public class Live2DBridgeWrapper : IDisposable
{
    private readonly object _lock = new();
    private bool _initialized;
    private bool _modelLoaded;
    private bool _disposed;

    public float FPS
    {
        get
        {
            lock (_lock) { return _initialized ? Live2DBridgeNative.Bridge_GetFPS() : 0; }
        }
    }

    public bool Initialize(string sdkKey = "")
    {
        lock (_lock)
        {
            if (_initialized) return true;
            try
            {
                _initialized = Live2DBridgeNative.Bridge_Initialize(sdkKey);
                return _initialized;
            }
            catch (Exception ex)
            {
                throw new Live2DBridgeException("Failed to initialize bridge", ex);
            }
        }
    }

    public void Shutdown()
    {
        lock (_lock)
        {
            if (!_initialized) return;
            try
            {
                Live2DBridgeNative.Bridge_Shutdown();
                _initialized = false;
                _modelLoaded = false;
            }
            catch (Exception ex)
            {
                throw new Live2DBridgeException("Failed to shutdown bridge", ex);
            }
        }
    }

    public bool LoadModel(string modelPath)
    {
        lock (_lock)
        {
            if (!_initialized) return false;
            try
            {
                _modelLoaded = Live2DBridgeNative.Bridge_LoadModel(modelPath);
                return _modelLoaded;
            }
            catch (Exception ex)
            {
                throw new Live2DBridgeException($"Failed to load model: {modelPath}", ex);
            }
        }
    }

    public void UnloadModel()
    {
        lock (_lock)
        {
            if (!_modelLoaded) return;
            Live2DBridgeNative.Bridge_UnloadModel();
            _modelLoaded = false;
        }
    }

    public bool InitializeRenderer(IntPtr hwnd, int width, int height)
    {
        lock (_lock)
        {
            if (!_initialized) return false;
            return Live2DBridgeNative.Bridge_InitializeRenderer(hwnd, width, height);
        }
    }

    public void Render()
    {
        lock (_lock)
        {
            if (!_initialized || !_modelLoaded) return;
            Live2DBridgeNative.Bridge_Render();
        }
    }

    public void Resize(int width, int height)
    {
        lock (_lock)
        {
            if (!_initialized) return;
            Live2DBridgeNative.Bridge_Resize(width, height);
        }
    }

    public void SetMotion(string group, string name)
    {
        lock (_lock)
        {
            if (!_initialized || !_modelLoaded) return;
            Live2DBridgeNative.Bridge_SetMotionGroup(group, name);
        }
    }

    public void SetParameter(string paramId, float value)
    {
        lock (_lock)
        {
            if (!_initialized || !_modelLoaded) return;
            Live2DBridgeNative.Bridge_SetParameter(paramId, value);
        }
    }

    public void SetEyeTarget(float x, float y)
    {
        lock (_lock)
        {
            if (!_initialized || !_modelLoaded) return;
            Live2DBridgeNative.Bridge_SetEyeTarget(
                Math.Clamp(x, -1.0f, 1.0f),
                Math.Clamp(y, -1.0f, 1.0f));
        }
    }

    /// <summary>Read rendered pixels. Returns (pointer, stride). Caller must call UnlockPixels() after copying.</summary>
    public (IntPtr pointer, int stride) ReadPixels()
    {
        lock (_lock)
        {
            if (!_initialized || !_modelLoaded) return (IntPtr.Zero, 0);
            var ptr = Live2DBridgeNative.Bridge_ReadPixels();
            var stride = Live2DBridgeNative.Bridge_GetPixelStride();
            return (ptr, stride);
        }
    }

    public void UnlockPixels()
    {
        lock (_lock)
        {
            Live2DBridgeNative.Bridge_UnlockPixels();
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        Shutdown();
    }
}
