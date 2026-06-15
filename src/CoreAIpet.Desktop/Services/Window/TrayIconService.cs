using System.Windows;
using System.Windows.Forms;
using System.Drawing;
using WpfWindow = System.Windows.Window;

namespace CoreAIpet.Desktop.Services.Window;

/// <summary>
/// 系统托盘服务 — 托盘图标、右键菜单、双击恢复
/// </summary>
public class TrayIconService : IDisposable
{
    private NotifyIcon? _notifyIcon;
    private Action? _onShow;
    private Action? _onHide;
    private Action? _onSettings;
    private Action? _onExit;

    public void Initialize(WpfWindow ownerWindow, Action onShow, Action onHide, Action onSettings, Action onExit)
    {
        _onShow = onShow;
        _onHide = onHide;
        _onSettings = onSettings;
        _onExit = onExit;

        _notifyIcon = new NotifyIcon
        {
            Icon = SystemIcons.Application,
            Visible = true,
            Text = "CoreAIpet"
        };

        _notifyIcon.DoubleClick += (s, e) => _onShow?.Invoke();
        _notifyIcon.MouseClick += OnTrayClick;
    }

    private void OnTrayClick(object? sender, FormsMouseEventArgs e)
    {
        if (e.Button != FormsMouseButtons.Right) return;

        var menu = new ContextMenuStrip();
        menu.Items.Add("显示助手", null, (s, e) => _onShow?.Invoke());
        menu.Items.Add("隐藏助手", null, (s, e) => _onHide?.Invoke());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("设置", null, (s, e) => _onSettings?.Invoke());
        menu.Items.Add("重启", null, (s, e) =>
        {
            System.Diagnostics.Process.Start(Environment.ProcessPath!);
            _onExit?.Invoke();
        });
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("退出", null, (s, e) => _onExit?.Invoke());

        var method = typeof(NotifyIcon).GetMethod("ShowContextMenu",
            System.Reflection.BindingFlags.Instance | System.Reflection.BindingFlags.NonPublic);
        method?.Invoke(_notifyIcon, null);

        menu.Show(Cursor.Position);
    }

    public void Dispose()
    {
        if (_notifyIcon != null)
        {
            _notifyIcon.Visible = false;
            _notifyIcon.Dispose();
            _notifyIcon = null;
        }
    }
}
