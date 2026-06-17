using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using CoreAIpet.Desktop.Live2D.Bridge;

namespace CoreAIpet.Desktop.Live2D.Rendering;

/// <summary>
/// Live2D 显示控件 — 通过 WriteableBitmap 显示 D3D11 渲染结果。
/// 不使用 HwndHost，因为 AllowsTransparency + HwndHost 不兼容。
/// </summary>
public class Live2DHostControl : System.Windows.Controls.UserControl
{
    private readonly System.Windows.Controls.Image _image;
    private WriteableBitmap? _bitmap;
    private Live2DBridgeWrapper? _bridge;
    private int _width;
    private int _height;
    private byte[]? _pixelBuffer; // reusable buffer for pixel conversion

    public Live2DHostControl()
    {
        _image = new System.Windows.Controls.Image
        {
            Stretch = Stretch.None,
            HorizontalAlignment = System.Windows.HorizontalAlignment.Center,
            VerticalAlignment = System.Windows.VerticalAlignment.Center,
        };
        Content = _image;
    }

    public void Initialize(Live2DBridgeWrapper bridge, int width, int height)
    {
        _bridge = bridge;
        _width = width;
        _height = height;
        _pixelBuffer = new byte[width * height * 4];

        // Create WriteableBitmap (Pbgra32 = premultiplied BGRA, what WPF expects)
        _bitmap = new WriteableBitmap(width, height, 96, 96, PixelFormats.Pbgra32, null);
        _image.Source = _bitmap;

        Width = width;
        Height = height;
    }

    /// <summary>
    /// Copy rendered pixels from D3D11 to WriteableBitmap.
    /// Called each frame after Bridge_Render().
    /// D3D11 outputs RGBA (straight alpha), WPF expects BGRA (premultiplied alpha).
    /// </summary>
    public void UpdateFromRenderer()
    {
        if (_bridge == null || _bitmap == null || _pixelBuffer == null) return;

        var (ptr, stride) = _bridge.ReadPixels();
        if (ptr == IntPtr.Zero) return;

        try
        {
            int srcRowBytes = _width * 4;

            // Copy each row from native to managed buffer, converting RGBA → premultiplied BGRA
            for (int y = 0; y < _height; y++)
            {
                IntPtr srcRow = IntPtr.Add(ptr, y * stride);
                Marshal.Copy(srcRow, _pixelBuffer, y * srcRowBytes, srcRowBytes);

                // Convert in-place: RGBA → premultiplied BGRA
                int offset = y * srcRowBytes;
                for (int x = 0; x < srcRowBytes; x += 4)
                {
                    byte r = _pixelBuffer[offset + x + 0];
                    byte g = _pixelBuffer[offset + x + 1];
                    byte b = _pixelBuffer[offset + x + 2];
                    byte a = _pixelBuffer[offset + x + 3];

                    _pixelBuffer[offset + x + 0] = (byte)(b * a / 255); // B
                    _pixelBuffer[offset + x + 1] = (byte)(g * a / 255); // G
                    _pixelBuffer[offset + x + 2] = (byte)(r * a / 255); // R
                    _pixelBuffer[offset + x + 3] = a;                     // A
                }
            }

            // Write converted pixels to WriteableBitmap
            _bitmap.WritePixels(new Int32Rect(0, 0, _width, _height), _pixelBuffer, _width * 4, 0);
        }
        finally
        {
            _bridge.UnlockPixels();
        }
    }

    protected override Size MeasureOverride(Size constraint)
    {
        return new Size(_width > 0 ? _width : 200, _height > 0 ? _height : 280);
    }
}
