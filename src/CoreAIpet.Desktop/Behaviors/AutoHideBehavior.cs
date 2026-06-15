using System.Windows;
using System.Windows.Input;
using System.Windows.Threading;

namespace CoreAIpet.Desktop.Behaviors;

/// <summary>
/// 自动隐藏行为 — 鼠标离开后延迟隐藏
/// </summary>
public class AutoHideBehavior
{
    private readonly DispatcherTimer _hideTimer;
    private readonly UIElement _target;
    private readonly Action _hideAction;

    public AutoHideBehavior(UIElement target, Action hideAction, double delaySeconds = 1.0)
    {
        _target = target;
        _hideAction = hideAction;
        _hideTimer = new DispatcherTimer
        {
            Interval = TimeSpan.FromSeconds(delaySeconds)
        };
        _hideTimer.Tick += (_, _) =>
        {
            _hideTimer.Stop();
            _hideAction();
        };

        _target.MouseLeave += OnMouseLeave;
        _target.MouseEnter += OnMouseEnter;
    }

    private void OnMouseEnter(object sender, MouseEventArgs e)
    {
        _hideTimer.Stop();
    }

    private void OnMouseLeave(object sender, MouseEventArgs e)
    {
        _hideTimer.Start();
    }

    public void CancelHide()
    {
        _hideTimer.Stop();
    }
}
