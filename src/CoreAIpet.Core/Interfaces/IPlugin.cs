namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 插件核心接口 — 定义完整的插件生命周期
/// 生命周期: Load → Activate → Execute(循环) → Deactivate → Unload
/// </summary>
public interface IPlugin : IDisposable
{
    /// <summary>插件唯一标识 (如 "coreai.jira")</summary>
    string Id { get; }

    /// <summary>显示名称</summary>
    string Name { get; }

    /// <summary>版本号 (SemVer)</summary>
    string Version { get; }

    /// <summary>当前状态</summary>
    PluginState State { get; }

    /// <summary>
    /// 加载阶段: 接收上下文, 初始化内部资源
    /// </summary>
    Task LoadAsync(IPluginContext context);

    /// <summary>
    /// 激活阶段: 插件开始运行
    /// </summary>
    Task ActivateAsync();

    /// <summary>
    /// 执行阶段: 插件核心逻辑
    /// </summary>
    Task ExecuteAsync(CancellationToken cancellationToken);

    /// <summary>
    /// 停用阶段: 暂停运行但保留状态
    /// </summary>
    Task DeactivateAsync();

    /// <summary>
    /// 卸载阶段: 完全释放资源
    /// </summary>
    Task UnloadAsync();

    /// <summary>
    /// 返回此插件提供给径向菜单的菜单项
    /// </summary>
    IReadOnlyList<Models.Plugin.PluginMenuItem> GetMenuItems();

    /// <summary>
    /// 处理菜单项点击事件
    /// </summary>
    Task HandleMenuActionAsync(string actionId);
}

/// <summary>
/// 插件状态枚举
/// </summary>
public enum PluginState
{
    Unloaded,
    Loading,
    Loaded,
    Activating,
    Active,
    Deactivating,
    Deactivated,
    Unloading,
    Error
}
