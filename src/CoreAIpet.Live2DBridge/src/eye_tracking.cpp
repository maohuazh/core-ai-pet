// ============================================================
// eye_tracking.cpp — 眼球追踪参数计算
// ------------------------------------------------------------
// 输入：x, y ∈ [-1, 1]（鼠标相对角色中心归一化坐标）
// 输出：写入模型参数 ParamEyeBallX / ParamEyeBallY
// 角度限制：水平 ±30°，垂直 ±15°
//
// Cubism 标准参数范围：
//   ParamEyeBallX: -1 .. 1
//   ParamEyeBallY: -1 .. 1
// 这里把 ±30°/±15° 映射到参数全范围，实现「软限制」。
// ============================================================

#include "bridge_internal.h"
#include <cmath>
#include <algorithm>

namespace EyeTracking {

// 角度限制（度）
static constexpr float kMaxYawDeg   = 30.0f;
static constexpr float kMaxPitchDeg = 15.0f;

// 当前目标（供 Renderer 绘制占位图时读取）
static float g_targetX = 0.0f;
static float g_targetY = 0.0f;

static float Clamp(float v, float lo, float hi)
{
    if (v < lo) return lo;
    if (v > hi) return hi;
    return v;
}

// 将「视角度数」映射到 Cubism 参数（-1..1），带 soft-clamp
static float DegToParam(float deg, float maxDeg)
{
    float p = deg / maxDeg;      // -1..1
    return Clamp(p, -1.0f, 1.0f);
}

void Apply(float x, float y)
{
    // 输入合法性
    if (!std::isfinite(x)) x = 0.0f;
    if (!std::isfinite(y)) y = 0.0f;
    x = Clamp(x, -1.0f, 1.0f);
    y = Clamp(y, -1.0f, 1.0f);

    g_targetX = x;
    g_targetY = y;

    // 将归一化坐标视为「视角比例」：x=1 → +30°, x=-1 → -30°
    float yawDeg   = x * kMaxYawDeg;
    float pitchDeg = y * kMaxPitchDeg;

    float paramX = DegToParam(yawDeg,   kMaxYawDeg);
    float paramY = DegToParam(pitchDeg, kMaxPitchDeg);

    Model::SetParameter("ParamEyeBallX", paramX);
    Model::SetParameter("ParamEyeBallY", paramY);
}

void GetTarget(float& x, float& y)
{
    x = g_targetX;
    y = g_targetY;
}

} // namespace EyeTracking
