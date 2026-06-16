// ============================================================
// ICharacterRenderer.cs — 角色渲染器统一接口
// ------------------------------------------------------------
// 所有角色渲染器（WPF Canvas 占位 / Live2D Bridge / 未来其他引擎）
// 都实现此接口，确保上层代码（ViewModel / Controller）无需改动即可切换。
//
// 切换方式：仅需修改 CompositionRoot 中的 DI 注册。
// ============================================================

using CoreAIpet.Core.Models.Character;

namespace CoreAIpet.Core.Interfaces;

/// <summary>
/// 角色渲染器接口 — 屏蔽底层渲染实现（Canvas / Live2D / 其他）
/// </summary>
public interface ICharacterRenderer
{
    /// <summary>渲染器名称，用于调试日志</summary>
    string RendererName { get; }

    /// <summary>是否已初始化并可渲染</summary>
    bool IsReady { get; }

    /// <summary>当前 FPS（渲染器自统计）</summary>
    float FPS { get; }

    /// <summary>设置角色状态，触发对应动画</summary>
    void SetState(CharacterState state);

    /// <summary>眼球追踪目标（归一化坐标 -1..1）</summary>
    void SetEyeTarget(float x, float y);

    /// <summary>设置自定义参数（如 ParamAngleX, ParamMouthOpenY 等）</summary>
    void SetParameter(string paramId, float value);

    /// <summary>请求重绘一帧（由外部渲染循环驱动）</summary>
    void RenderFrame();

    /// <summary>调整渲染尺寸（窗口 resize 时调用）</summary>
    void Resize(int width, int height);

    /// <summary>释放资源</summary>
    void Dispose();
}
