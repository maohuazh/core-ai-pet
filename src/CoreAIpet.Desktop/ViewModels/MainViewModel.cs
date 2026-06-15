using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 主窗口 ViewModel — 状态管理、命令
/// </summary>
public partial class MainViewModel : ObservableObject
{
    [ObservableProperty] private bool _isMenuVisible;
    [ObservableProperty] private bool _isChatVisible;

    [RelayCommand]
    private void ToggleMenu()
    {
        IsMenuVisible = !IsMenuVisible;
    }

    [RelayCommand]
    private void OpenChat()
    {
        IsChatVisible = true;
    }

    [RelayCommand]
    private void CloseChat()
    {
        IsChatVisible = false;
    }
}
