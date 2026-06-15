using System.Windows;
using System.Windows.Input;

namespace CoreAIpet.Desktop.Views;

/// <summary>
/// 聊天气泡窗口 — 输入框 + AI 回复显示
/// </summary>
public partial class ChatBubbleWindow : Window
{
    public event Action<string>? MessageSent;

    public ChatBubbleWindow()
    {
        InitializeComponent();
    }

    private void InputBox_KeyDown(object sender, KeyEventArgs e)
    {
        if (e.Key == Key.Enter && (Keyboard.Modifiers & ModifierKeys.Shift) == 0)
        {
            e.Handled = true;
            SendMessage();
        }
    }

    private void SendButton_Click(object sender, RoutedEventArgs e)
    {
        SendMessage();
    }

    private void SendMessage()
    {
        var text = InputBox.Text?.Trim();
        if (string.IsNullOrEmpty(text)) return;

        MessageSent?.Invoke(text);
        InputBox.Clear();
    }

    public void AppendResponse(string text)
    {
        ResponseText.Visibility = Visibility.Visible;
        ResponseText.Text += text;

        if (ResponseText.Text.Length > 300)
        {
            ViewMoreButton.Visibility = Visibility.Visible;
            ResponseText.MaxHeight = 60;
        }
    }

    public void SetResponse(string text)
    {
        ResponseText.Visibility = Visibility.Visible;
        ResponseText.Text = text;

        if (text.Length > 300)
        {
            ViewMoreButton.Visibility = Visibility.Visible;
            ResponseText.MaxHeight = 60;
        }
    }

    private void ViewMore_Click(object sender, RoutedEventArgs e)
    {
        ResponseText.MaxHeight = double.PositiveInfinity;
        ViewMoreButton.Visibility = Visibility.Collapsed;
    }
}
