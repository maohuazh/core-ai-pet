namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 窗口位置记录
/// </summary>
public record WindowPosition(double X, double Y);

/// <summary>
/// 窗口位置持久化接口
/// </summary>
public interface IPositionService
{
    /// <summary>加载上次保存的窗口位置</summary>
    Task<WindowPosition?> LoadAsync();

    /// <summary>保存当前窗口位置</summary>
    Task SaveAsync(WindowPosition position);
}
