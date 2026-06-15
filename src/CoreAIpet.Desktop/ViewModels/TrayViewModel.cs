using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 托盘 ViewModel — 右键菜单命令
/// </summary>
public partial class TrayViewModel : ObservableObject
{
    [ObservableProperty] private bool _isVisible = true;

    [RelayCommand] private void Show() => IsVisible = true;
    [RelayCommand] private void Hide() => IsVisible = false;
}
