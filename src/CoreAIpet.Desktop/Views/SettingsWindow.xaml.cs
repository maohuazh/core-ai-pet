using System.Windows;
using Microsoft.Extensions.DependencyInjection;
using CoreAIpet.Desktop.Services.Configuration;
using CoreAIpet.Desktop.Services.Live2D;

namespace CoreAIpet.Desktop.Views;

public partial class SettingsWindow : Window
{
    private readonly ConfigService _configService;
    private readonly ModelLoaderService _modelLoader;

    public SettingsWindow()
    {
        InitializeComponent();

        var app = (App)App.Current;
        _configService = app.Host.Services.GetRequiredService<ConfigService>();
        _modelLoader = app.Host.Services.GetRequiredService<ModelLoaderService>();

        LoadSettings();
    }

    private void LoadSettings()
    {
        // Live2D 设置
        ModelPathBox.Text = _configService.Live2D.ModelPath;

        // 加载可用变体
        var variants = _modelLoader.GetAvailableVariants();
        VariantCombo.ItemsSource = variants;

        // 选择当前变体
        var currentVariant = variants.Find(v => v.Filename == _configService.Live2D.ModelVariant);
        if (currentVariant != null)
        {
            VariantCombo.SelectedItem = currentVariant;
        }

        // 状态
        UpdateStatus();
    }

    private void ApplyVariant_Click(object sender, RoutedEventArgs e)
    {
        if (VariantCombo.SelectedItem is not ModelVariant selectedVariant)
        {
            System.Windows.MessageBox.Show("请选择一个模型变体", "提示", MessageBoxButton.OK, MessageBoxImage.Warning);
            return;
        }

        // 保存配置
        _configService.Live2D.ModelVariant = selectedVariant.Filename;
        _ = _configService.SaveAsync();

        // 加载新变体
        var success = _modelLoader.LoadVariant(
            _configService.Live2D.ModelPath,
            selectedVariant.Filename);

        if (success)
        {
            StatusText.Text = "✓ 变体已成功应用";
            StatusText.Foreground = System.Windows.Media.Brushes.Green;
        }
        else
        {
            StatusText.Text = $"✗ 加载失败: {_modelLoader.LastError}";
            StatusText.Foreground = System.Windows.Media.Brushes.Red;
        }
    }

    private void UpdateStatus()
    {
        if (_modelLoader.IsLive2DActive)
        {
            StatusText.Text = "✓ Live2D 模型已加载";
            StatusText.Foreground = System.Windows.Media.Brushes.Green;
        }
        else
        {
            StatusText.Text = "⚠ 使用 Canvas 占位符（Live2D 未加载）";
            StatusText.Foreground = System.Windows.Media.Brushes.Orange;
        }
    }
}
