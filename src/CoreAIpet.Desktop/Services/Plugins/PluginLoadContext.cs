using System.IO;
using System.Reflection;
using System.Runtime.Loader;

namespace CoreAIpet.Desktop.Services.Plugins;

/// <summary>
/// 插件隔离加载上下文 — 每个插件独立 AssemblyLoadContext
/// </summary>
public class PluginLoadContext : AssemblyLoadContext
{
    private readonly AssemblyDependencyResolver _resolver;

    public PluginLoadContext(string pluginPath) : base(isCollectible: true)
    {
        _resolver = new AssemblyDependencyResolver(pluginPath);
    }

    protected override Assembly? Load(AssemblyName assemblyName)
    {
        // 优先使用宿主已加载的程序集 (共享 Core 接口)
        var existing = Default.Assemblies.FirstOrDefault(a => a.GetName().Name == assemblyName.Name);
        if (existing != null) return null; // 返回 null 让运行时使用 Default 上下文

        var path = _resolver.ResolveAssemblyToPath(assemblyName);
        return path != null ? LoadFromAssemblyPath(path) : null;
    }
}
