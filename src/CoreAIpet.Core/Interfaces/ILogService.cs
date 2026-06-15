namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 日志级别枚举
/// </summary>
public enum LogLevel
{
    Debug,
    Information,
    Warning,
    Error
}

/// <summary>
/// 日志服务接口
/// </summary>
public interface ILogService
{
    void Debug(string message);
    void Information(string message);
    void Warning(string message);
    void Error(string message);
    void Error(string message, Exception exception);

    /// <summary>获取最近的日志条目 (供 Debug 面板使用)</summary>
    IReadOnlyList<LogEntry> GetRecentLogs(int count = 100);

    /// <summary>新日志事件</summary>
    event EventHandler<LogEntry>? LogAdded;
}

/// <summary>
/// 日志条目
/// </summary>
public record LogEntry(DateTimeOffset Timestamp, LogLevel Level, string Message);
