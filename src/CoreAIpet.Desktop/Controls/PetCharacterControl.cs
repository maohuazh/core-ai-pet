// ============================================================
// PetCharacterControl.xaml — WPF Canvas 占位角色
// ------------------------------------------------------------
// 纯 WPF 绘制的卡通角色，用于在 Live2D Bridge 未编译时
// 提供完整的角色交互体验（眼球跟随、状态切换、眨眼、呼吸）。
//
// 架构：实现 ICharacterRenderer 接口，未来可无缝替换为 Live2D 渲染器。
// ============================================================

using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using System.Windows.Media.Animation;
using System.Windows.Shapes;
using CoreAIpet.Core.Interfaces;
using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Desktop.Controls;

public partial class PetCharacterControl : Canvas, ICharacterRenderer
{
    // ============================================================
    // 渲染器信息
    // ============================================================
    public string RendererName => "WpfCanvas (Mock)";
    public bool IsReady => _isReady;
    public float FPS => _fps;

    private bool _isReady;
    private float _fps;
    private CharacterState _currentState = CharacterState.Idle;

    // ============================================================
    // 角色尺寸配置
    // ============================================================
    private double _faceWidth = 80;
    private double _faceHeight = 90;
    private double _eyeSpacing = 15;
    private double _eyeSize = 11;
    private double _pupilSize = 5;
    private double _mouthWidth = 12;
    private double _mouthHeight = 4;

    // ============================================================
    // 眼球追踪
    // ============================================================
    private double _eyeTargetX;
    private double _eyeTargetY;
    private const double MaxEyeOffset = 3;

    // ============================================================
    // 动画相位
    // ============================================================
    private readonly DateTime _startTime = DateTime.Now;
    private int _frameCount;
    private DateTime _lastFpsUpdate = DateTime.Now;

    // ============================================================
    // 视觉元素（WPF Shapes）
    // ============================================================
    private readonly Ellipse _faceShape;
    private readonly Ellipse _leftEyeWhite;
    private readonly Ellipse _rightEyeWhite;
    private readonly Ellipse _leftPupil;
    private readonly Ellipse _rightPupil;
    private readonly Ellipse _leftEyeHighlight;
    private readonly Ellipse _rightEyeHighlight;
    private readonly Path _mouthShape;
    private readonly Ellipse _blushLeft;
    private readonly Ellipse _blushRight;

    // ============================================================
    // 构造函数
    // ============================================================
    public PetCharacterControl()
    {
        Width = 100;
        Height = 120;
        ClipToBounds = false;

        // 创建视觉元素
        _faceShape = CreateFace();
        _leftEyeWhite = CreateEyeWhite();
        _rightEyeWhite = CreateEyeWhite();
        _leftPupil = CreatePupil();
        _rightPupil = CreatePupil();
        _leftEyeHighlight = CreateEyeHighlight();
        _rightEyeHighlight = CreateEyeHighlight();
        _mouthShape = CreateMouth();
        _blushLeft = CreateBlush();
        _blushRight = CreateBlush();

        // 添加到画布
        Children.Add(_faceShape);
        Children.Add(_blushLeft);
        Children.Add(_blushRight);
        Children.Add(_leftEyeWhite);
        Children.Add(_rightEyeWhite);
        Children.Add(_leftPupil);
        Children.Add(_rightPupil);
        Children.Add(_leftEyeHighlight);
        Children.Add(_rightEyeHighlight);
        Children.Add(_mouthShape);

        // 居中布局
        ArrangeElements();

        // 启动动画循环
        _isReady = true;
        CompositionTarget.Rendering += OnRendering;
    }

    // ============================================================
    // ICharacterRenderer 实现
    // ============================================================

    public void SetState(CharacterState state)
    {
        _currentState = state;
        UpdateExpression();
    }

    public void SetEyeTarget(float x, float y)
    {
        _eyeTargetX = Math.Clamp(x, -1, 1);
        _eyeTargetY = Math.Clamp(y, -1, 1);
    }

    public void SetParameter(string paramId, float value)
    {
        // 占位实现：Live2D 模式下会接收 ParamAngleX 等参数
        // 这里可以扩展为直接控制 Canvas 元素
    }

    public void RenderFrame()
    {
        // Canvas 自动重绘，无需手动触发
        _frameCount++;
        UpdateFps();
    }

    public void Resize(int width, int height)
    {
        Width = width;
        Height = height;
        ArrangeElements();
    }

    public void Dispose()
    {
        CompositionTarget.Rendering -= OnRendering;
        Children.Clear();
    }

    // ============================================================
    // 视觉元素创建
    // ============================================================

    private Ellipse CreateFace()
    {
        return new Ellipse
        {
            Width = _faceWidth,
            Height = _faceHeight,
            Fill = new RadialGradientBrush(
                Color.FromRgb(255, 224, 189),
                Color.FromRgb(255, 200, 150))
            {
                GradientOrigin = new Point(0.3, 0.3)
            },
            Stroke = new SolidColorBrush(Color.FromRgb(180, 140, 100)),
            StrokeThickness = 2
        };
    }

    private Ellipse CreateEyeWhite()
    {
        return new Ellipse
        {
            Width = _eyeSize,
            Height = _eyeSize,
            Fill = Brushes.White,
            Stroke = new SolidColorBrush(Color.FromRgb(80, 60, 40)),
            StrokeThickness = 1.5
        };
    }

    private Ellipse CreatePupil()
    {
        return new Ellipse
        {
            Width = _pupilSize,
            Height = _pupilSize,
            Fill = new RadialGradientBrush(
                Color.FromRgb(60, 40, 20),
                Color.FromRgb(20, 10, 5))
        };
    }

    private Ellipse CreateEyeHighlight()
    {
        return new Ellipse
        {
            Width = 3,
            Height = 3,
            Fill = new SolidColorBrush(Color.FromArgb(200, 255, 255, 255))
        };
    }

    private Path CreateMouth()
    {
        return new Path
        {
            Fill = new SolidColorBrush(Color.FromRgb(180, 80, 80)),
            Stroke = new SolidColorBrush(Color.FromRgb(120, 40, 40)),
            StrokeThickness = 1
        };
    }

    private Ellipse CreateBlush()
    {
        return new Ellipse
        {
            Width = 12,
            Height = 7,
            Fill = new SolidColorBrush(Color.FromArgb(60, 255, 150, 150))
        };
    }

    // ============================================================
    // 布局
    // ============================================================

    private void ArrangeElements()
    {
        double cx = Width / 2;
        double cy = Height / 2;

        SetLeft(_faceShape, cx - _faceWidth / 2);
        SetTop(_faceShape, cy - _faceHeight / 2);

        double eyeY = cy - 5;
        SetLeft(_leftEyeWhite, cx - _eyeSpacing - _eyeSize / 2);
        SetTop(_leftEyeWhite, eyeY - _eyeSize / 2);
        SetLeft(_rightEyeWhite, cx + _eyeSpacing - _eyeSize / 2);
        SetTop(_rightEyeWhite, eyeY - _eyeSize / 2);

        UpdatePupilPosition();

        SetLeft(_blushLeft, cx - _eyeSpacing - 10);
        SetTop(_blushLeft, eyeY + 10);
        SetLeft(_blushRight, cx + _eyeSpacing - 2);
        SetTop(_blushRight, eyeY + 10);

        UpdateMouthShape();
    }

    private void UpdatePupilPosition()
    {
        double cx = Width / 2;
        double eyeY = Height / 2 - 5;

        double offsetX = _eyeTargetX * MaxEyeOffset;
        double offsetY = -_eyeTargetY * MaxEyeOffset;

        double leftPupilX = cx - _eyeSpacing - _pupilSize / 2 + offsetX;
        double leftPupilY = eyeY - _pupilSize / 2 + offsetY;
        SetLeft(_leftPupil, leftPupilX);
        SetTop(_leftPupil, leftPupilY);
        SetLeft(_leftEyeHighlight, leftPupilX + 1);
        SetTop(_leftEyeHighlight, leftPupilY + 1);

        double rightPupilX = cx + _eyeSpacing - _pupilSize / 2 + offsetX;
        double rightPupilY = eyeY - _pupilSize / 2 + offsetY;
        SetLeft(_rightPupil, rightPupilX);
        SetTop(_rightPupil, rightPupilY);
        SetLeft(_rightEyeHighlight, rightPupilX + 1);
        SetTop(_rightEyeHighlight, rightPupilY + 1);
    }

    private void UpdateMouthShape()
    {
        double cx = Width / 2;
        double cy = Height / 2 + 15;

        double w = _mouthWidth;
        double h = _mouthHeight;

        // 根据状态调整嘴型
        switch (_currentState)
        {
            case CharacterState.Happy:
                // 微笑弧线
                _mouthShape.Data = new PathGeometry(new[]
                {
                    new PathFigure(new Point(cx - w / 2, cy), new[]
                    {
                        new QuadraticBezierSegment(new Point(cx - w / 4, cy + h), new Point(cx, cy + h), true)
                    }, false)
                });
                _mouthShape.Stroke = new SolidColorBrush(Color.FromRgb(120, 40, 40));
                _mouthShape.Fill = Brushes.Transparent;
                _mouthShape.StrokeThickness = 2.5;
                break;

            case CharacterState.Talking:
                // 张开的嘴巴（椭圆）
                _mouthShape.Data = new EllipseGeometry(new Point(cx, cy), w / 2, h);
                _mouthShape.StrokeThickness = 1;
                break;

            case CharacterState.Thinking:
                // 小圆嘴（惊讶/思考）
                _mouthShape.Data = new EllipseGeometry(new Point(cx, cy), 2, 2);
                _mouthShape.StrokeThickness = 1;
                break;

            default: // Idle
                // 平静的小弧线
                _mouthShape.Data = new PathGeometry(new[]
                {
                    new PathFigure(new Point(cx - w / 3, cy), new[]
                    {
                        new QuadraticBezierSegment(new Point(cx - w / 6, cy + 1), new Point(cx, cy + 1), true)
                    }, false)
                });
                _mouthShape.Stroke = new SolidColorBrush(Color.FromRgb(120, 40, 40));
                _mouthShape.Fill = Brushes.Transparent;
                _mouthShape.StrokeThickness = 2;
                break;
        }
    }

    // ============================================================
    // 表情更新
    // ============================================================

    private void UpdateExpression()
    {
        // 更新嘴型
        UpdateMouthShape();

        // 根据状态调整腮红
        double blushOpacity = _currentState == CharacterState.Happy ? 0.6 : 0.3;
        _blushLeft.Opacity = blushOpacity;
        _blushRight.Opacity = blushOpacity;
    }

    // ============================================================
    // 渲染循环
    // ============================================================

    private void OnRendering(object? sender, EventArgs e)
    {
        var elapsed = DateTime.Now - _startTime;
        double t = elapsed.TotalSeconds;

        // 呼吸动画：上下浮动
        double breath = Math.Sin(t * 2) * 1.5;
        SetTop(_faceShape, Height / 2 - _faceHeight / 2 + breath);
        SetTop(_leftEyeWhite, Height / 2 - 5 - _eyeSize / 2 + breath);
        SetTop(_rightEyeWhite, Height / 2 - 5 - _eyeSize / 2 + breath);
        SetTop(_leftPupil, Height / 2 - 5 - _pupilSize / 2 + breath + (-_eyeTargetY * MaxEyeOffset));
        SetTop(_rightPupil, Height / 2 - 5 - _pupilSize / 2 + breath + (-_eyeTargetY * MaxEyeOffset));
        SetTop(_blushLeft, Height / 2 + 10 + breath);
        SetTop(_blushRight, Height / 2 + 10 + breath);

        // 眨眼：每 4 秒一次，持续 0.15 秒
        double blinkPhase = t % 4;
        bool isBlinking = blinkPhase > 3.85;
        double eyeScale = isBlinking ? 0.1 : 1.0;
        _leftEyeWhite.Height = _eyeSize * eyeScale;
        _rightEyeWhite.Height = _eyeSize * eyeScale;

        // Talking 状态：嘴巴张合动画
        if (_currentState == CharacterState.Talking)
        {
            double mouthOpen = (Math.Sin(t * 15) + 1) * 0.5;
            double cx = Width / 2;
            double cy = Height / 2 + 15 + breath;
            _mouthShape.Data = new EllipseGeometry(
                new Point(cx, cy),
                _mouthWidth / 2,
                _mouthHeight * (0.5 + mouthOpen));
        }

        RenderFrame();
    }

    private void UpdateFps()
    {
        var now = DateTime.Now;
        var elapsed = (now - _lastFpsUpdate).TotalSeconds;
        if (elapsed >= 0.5)
        {
            _fps = (float)(_frameCount / elapsed);
            _frameCount = 0;
            _lastFpsUpdate = now;
        }
    }
}
