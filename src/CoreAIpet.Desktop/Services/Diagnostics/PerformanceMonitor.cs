using System.Diagnostics;
using CoreAIpet.Core.Interfaces;

namespace CoreAIpet.Desktop.Services.Diagnostics;

/// <summary>
/// 性能监控 — FPS / CPU / 内存采集
/// </summary>
public class PerformanceMonitor
{
    private readonly Process _process;
    private float _cpuPercent;

    public float FPS { get; set; }
    public float CpuPercent => _cpuPercent;
    public long MemoryMB => _process.WorkingSet64 / (1024 * 1024);

    public PerformanceMonitor()
    {
        _process = Process.GetCurrentProcess();
    }

    public void Refresh()
    {
        _process.Refresh();
        // CPU 百分比简化计算
        _cpuPercent = (float)Math.Min(100, _process.TotalProcessorTime.TotalMilliseconds /
            (Environment.ProcessorCount * _process.UserProcessorTime.TotalMilliseconds + 1) * 100);
    }
}
