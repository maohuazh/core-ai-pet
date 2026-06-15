using CoreAIpet.Core.Interfaces;
using Microsoft.Extensions.Logging;

namespace CoreAIpet.Desktop.Services.Diagnostics;

/// <summary>
/// 日志服务 — 封装 Serilog，提供环形缓冲区给 Debug 面板
/// </summary>
public class LogService : ILogService
{
    private readonly Microsoft.Extensions.Logging.ILogger<LogService> _logger;
    private readonly List<LogEntry> _recentLogs = new();
    private readonly object _lock = new();
    private const int MaxBufferSize = 500;

    public event EventHandler<LogEntry>? LogAdded;

    public LogService(Microsoft.Extensions.Logging.ILogger<LogService> logger)
    {
        _logger = logger;
    }

    public void Debug(string message)
    {
        _logger.LogDebug("{Message}", message);
        AddLog(Core.Interfaces.LogLevel.Debug, message);
    }

    public void Information(string message)
    {
        _logger.LogInformation("{Message}", message);
        AddLog(Core.Interfaces.LogLevel.Information, message);
    }

    public void Warning(string message)
    {
        _logger.LogWarning("{Message}", message);
        AddLog(Core.Interfaces.LogLevel.Warning, message);
    }

    public void Error(string message)
    {
        _logger.LogError("{Message}", message);
        AddLog(Core.Interfaces.LogLevel.Error, message);
    }

    public void Error(string message, Exception exception)
    {
        _logger.LogError(exception, "{Message}", message);
        AddLog(Core.Interfaces.LogLevel.Error, $"{message}: {exception.Message}");
    }

    public IReadOnlyList<LogEntry> GetRecentLogs(int count = 100)
    {
        lock (_lock)
        {
            return _recentLogs.TakeLast(count).ToList().AsReadOnly();
        }
    }

    private void AddLog(Core.Interfaces.LogLevel level, string message)
    {
        var entry = new LogEntry(DateTimeOffset.Now, level, message);
        lock (_lock)
        {
            _recentLogs.Add(entry);
            if (_recentLogs.Count > MaxBufferSize)
            {
                _recentLogs.RemoveAt(0);
            }
        }
        LogAdded?.Invoke(this, entry);
    }
}
