using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Shapes;
using CoreAIpet.Core.Models.Plugin;

namespace CoreAIpet.Desktop.Controls;

/// <summary>
/// 径向菜单控件 — 以角色为中心环形排列菜单项
/// </summary>
public class RadialMenuControl : Canvas
{
    private readonly List<MenuItemData> _items = new();
    private const double MenuRadius = 40;
    private const double ItemSize = 20;

    public event Action<PluginMenuItem>? ItemClicked;

    public void SetItems(IReadOnlyList<PluginMenuItem> items)
    {
        _items.Clear();
        Children.Clear();

        var count = items.Count;
        if (count == 0) return;

        var angleStep = 360.0 / count;

        for (int i = 0; i < count; i++)
        {
            var angle = angleStep * i - 90; // 从顶部开始
            var rad = angle * Math.PI / 180;
            var x = MenuRadius * Math.Cos(rad);
            var y = MenuRadius * Math.Sin(rad);

            var itemGrid = CreateItemGrid(items[i]);
            SetLeft(itemGrid, x + Width / 2 - ItemSize / 2);
            SetTop(itemGrid, y + Height / 2 - ItemSize / 2);
            Children.Add(itemGrid);

            _items.Add(new MenuItemData(items[i], itemGrid));
        }
    }

    private Grid CreateItemGrid(PluginMenuItem item)
    {
        var grid = new Grid
        {
            Width = ItemSize,
            Height = ItemSize,
            Cursor = Cursors.Hand
        };

        var border = new Border
        {
            Background = new System.Windows.Media.SolidColorBrush(System.Windows.Media.Color.FromArgb(200, 50, 50, 50)),
            CornerRadius = new CornerRadius(ItemSize / 2),
            ToolTip = item.Tooltip
        };
        grid.Children.Add(border);

        var text = new TextBlock
        {
            Text = item.Label.Length > 3 ? item.Label[..3] : item.Label,
            Foreground = System.Windows.Media.Brushes.White,
            HorizontalAlignment = System.Windows.HorizontalAlignment.Center,
            VerticalAlignment = System.Windows.VerticalAlignment.Center,
            FontSize = 10
        };
        grid.Children.Add(text);

        grid.MouseEnter += (s, e) =>
        {
            border.RenderTransform = new System.Windows.Media.ScaleTransform(1.1, 1.1);
            border.RenderTransformOrigin = new Point(0.5, 0.5);
        };
        grid.MouseLeave += (s, e) =>
        {
            border.RenderTransform = null;
        };
        grid.MouseLeftButtonDown += (s, e) =>
        {
            ItemClicked?.Invoke(item);
        };

        return grid;
    }

    private record MenuItemData(PluginMenuItem Item, Grid Grid);
}
