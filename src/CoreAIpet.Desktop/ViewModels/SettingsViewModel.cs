using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using CoreAIpet.Core.Interfaces;

namespace CoreAIpet.Desktop.ViewModels;

/// <summary>
/// 设置 ViewModel — 绑定 IConfigService
/// </summary>
public partial class SettingsViewModel : ObservableObject
{
    private readonly IConfigService _config;

    [ObservableProperty] private double _scale = 100;
    [ObservableProperty] private double _opacity = 100;
    [ObservableProperty] private bool _alwaysOnTop = true;
    [ObservableProperty] private bool _clickThrough;
    [ObservableProperty] private bool _autoStart;
    [ObservableProperty] private string _activeProvider = "openai";

    public SettingsViewModel(IConfigService config)
    {
        _config = config;
        LoadSettings();
    }

    private void LoadSettings()
    {
        Scale = _config.Appearance.Scale * 100;
        Opacity = _config.Appearance.Opacity * 100;
        AlwaysOnTop = _config.System.AlwaysOnTop;
        ClickThrough = _config.System.ClickThrough;
        AutoStart = _config.System.AutoStart;
        ActiveProvider = _config.AISettings.ActiveProvider;
    }

    [RelayCommand]
    private async Task SaveAsync()
    {
        _config.Appearance.Scale = Scale / 100.0;
        _config.Appearance.Opacity = Opacity / 100.0;
        _config.System.AlwaysOnTop = AlwaysOnTop;
        _config.System.ClickThrough = ClickThrough;
        _config.System.AutoStart = AutoStart;
        _config.AISettings.ActiveProvider = ActiveProvider;
        await _config.SaveAsync();
    }
}
