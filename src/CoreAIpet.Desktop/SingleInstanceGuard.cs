namespace CoreAIpet.Desktop;

/// <summary>
/// 单实例检测 — 使用 Mutex 确保应用只运行一个实例
/// </summary>
public static class SingleInstanceGuard
{
    private const string MutexName = "Global\\CoreAIpet_SingleInstance";
    private static Mutex? _mutex;

    /// <summary>
    /// 尝试获取单实例锁
    /// </summary>
    public static bool TryAcquire()
    {
        _mutex = new Mutex(true, MutexName, out bool createdNew);
        return createdNew;
    }

    /// <summary>
    /// 释放单实例锁
    /// </summary>
    public static void Release()
    {
        if (_mutex != null)
        {
            try
            {
                _mutex.ReleaseMutex();
            }
            catch { /* 忽略 */ }
            _mutex.Dispose();
            _mutex = null;
        }
    }
}
