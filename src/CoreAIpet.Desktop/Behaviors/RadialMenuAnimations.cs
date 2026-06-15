using System.Windows;
using System.Windows.Media.Animation;

namespace CoreAIpet.Desktop.Behaviors;

/// <summary>
/// 径向菜单动画 — Fade In/Scale Up 显示, Fade Out 隐藏 (200-300ms)
/// </summary>
public static class RadialMenuAnimations
{
    private static readonly Duration ShowDuration = new(TimeSpan.FromMilliseconds(250));
    private static readonly Duration HideDuration = new(TimeSpan.FromMilliseconds(200));

    public static void AnimateShow(FrameworkElement element)
    {
        element.Visibility = Visibility.Visible;

        var fadeIn = new DoubleAnimation(0, 1, ShowDuration) { EasingFunction = new QuadraticEase() };
        var scaleUp = new DoubleAnimation(0.5, 1.0, ShowDuration) { EasingFunction = new BackEase { EasingMode = EasingMode.EaseOut } };

        element.BeginAnimation(UIElement.OpacityProperty, fadeIn);

        var scaleTransform = new System.Windows.Media.ScaleTransform(0.5, 0.5);
        element.RenderTransform = scaleTransform;
        element.RenderTransformOrigin = new Point(0.5, 0.5);
        scaleTransform.BeginAnimation(ScaleTransform.ScaleXProperty, scaleUp);
        scaleTransform.BeginAnimation(ScaleTransform.ScaleYProperty, scaleUp);
    }

    public static void AnimateHide(FrameworkElement element)
    {
        var fadeOut = new DoubleAnimation(1, 0, HideDuration) { EasingFunction = new QuadraticEase() };
        fadeOut.Completed += (s, e) => element.Visibility = Visibility.Collapsed;
        element.BeginAnimation(UIElement.OpacityProperty, fadeOut);
    }
}
