using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 径向菜单 ViewModel — 从 PluginManager 获取菜单项，动态计算位置
/// </summary>
public partial class RadialMenuViewModel : ObservableObject
{
    [ObservableProperty]
    private bool _isVisible;

    [ObservableProperty]
    private ObservableCollection<PluginMenuItem> _menuItems = new();

    public void LoadMenuItems(IEnumerable<PluginMenuItem> items)
    {
        MenuItems.Clear();
        foreach (var item in items)
        {
            MenuItems.Add(item);
        }
    }

    [RelayCommand]
    private void Show()
    {
        IsVisible = true;
    }

    [RelayCommand]
    private void Hide()
    {
        IsVisible = false;
    }
}
