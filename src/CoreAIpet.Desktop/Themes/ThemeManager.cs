using System.Windows;

namespace CoreAIpet.Desktop.Themes;

/// <summary>
/// 主题管理 — 切换 Light/Dark 主题
/// </summary>
public static class ThemeManager
{
    private static readonly Uri DarkUri = new("Themes/DarkTheme.xaml", UriKind.Relative);
    private static readonly Uri LightUri = new("Themes/LightTheme.xaml", UriKind.Relative);

    public static void ApplyTheme(string theme)
    {
        var uri = theme.Equals("light", StringComparison.OrdinalIgnoreCase) ? LightUri : DarkUri;
        var existing = Application.Current.Resources.MergedDictionaries
            .FirstOrDefault(d => d.Source?.OriginalString?.Contains("Theme") == true);

        if (existing != null)
            Application.Current.Resources.MergedDictionaries.Remove(existing);

        Application.Current.Resources.MergedDictionaries.Add(new ResourceDictionary { Source = uri });
    }
}
