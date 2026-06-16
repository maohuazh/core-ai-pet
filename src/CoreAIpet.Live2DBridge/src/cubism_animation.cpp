// ============================================================
// cubism_animation.cpp — 动画状态机
// ------------------------------------------------------------
// 根据 motion group 驱动模型参数（眨眼、嘴型、呼吸）。
// 启用 SDK 后替换为 CubismMotion::StartUpdate 真实驱动。
// ============================================================

#include "bridge_internal.h"
#include <cmath>

namespace Animation {

static MotionState g_state;
static float g_blinkPhase = 0.0f;
static float g_breathPhase = 0.0f;

void Initialize()
{
    g_state = MotionState{};
    g_state.group = "idle";
    g_state.playing = true;
    g_state.startTick = ::GetTickCount();
    g_blinkPhase = 0.0f;
    g_breathPhase = 0.0f;
}

void SetMotion(const char* group, const char* name)
{
    if (!group) return;
    if (g_state.group == group && (!name || g_state.name == name)) return;

    g_state.group = group;
    g_state.name = name ? name : "";
    g_state.startTick = ::GetTickCount();
    g_state.playing = true;

#ifdef LIVE2D_HAS_SDK
    // TODO: 从当前 model 查找 motion，调用 motion->startUpdate()
#endif
}

// 每帧更新：写入眨眼、呼吸、口型等参数到 Model
void Update()
{
    DWORD now = ::GetTickCount();
    float t = (now - g_state.startTick) / 1000.0f;

    // 呼吸：缓慢正弦，幅度 0..1
    g_breathPhase += 0.02f;
    float breath = (std::sin(g_breathPhase) + 1.0f) * 0.5f;
    Model::SetParameter("ParamBreath", (float)breath);

    // 眨眼：周期 ~4s，闭眼时长 0.1s
    g_blinkPhase += 0.016f;
    float blink = 1.0f;
    float phase = std::fmod(g_blinkPhase, 4.0f);
    if (phase > 3.9f && phase < 4.0f) blink = 0.0f;
    Model::SetParameter("ParamEyeLOpen", blink);
    Model::SetParameter("ParamEyeROpen", blink);

    // 口型：仅在 talking 状态驱动
    if (g_state.group == "talking") {
        float mouth = (std::sin(t * 18.0f) + 1.0f) * 0.5f;
        Model::SetParameter("ParamMouthOpenY", (float)mouth);
    } else {
        Model::SetParameter("ParamMouthOpenY", 0.0f);
    }

    // 微笑：happy 状态下嘴角上翘
    float smile = (g_state.group == "happy") ? 1.0f : 0.0f;
    Model::SetParameter("ParamMouthForm", smile);

#ifdef LIVE2D_HAS_SDK
    // TODO: model->Update() → 真实 SDK 参数更新
#endif
}

const MotionState& Current()
{
    return g_state;
}

} // namespace Animation
