using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 为径向菜单提供菜单项的接口
/// </summary>
public interface IPluginMenuProvider
{
    /// <summary>获取插件提供的菜单项</summary>
    IReadOnlyList<PluginMenuItem> GetMenuItems();
}
