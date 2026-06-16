// ============================================================
// eye_tracking.cpp — Eye tracking parameters
// ============================================================

#include "bridge_internal.h"
#include <cmath>
#include <algorithm>

namespace EyeTracking {

static float g_targetX = 0.0f;
static float g_targetY = 0.0f;

static float Clamp(float v, float lo, float hi)
{
    if (v < lo) return lo;
    if (v > hi) return hi;
    return v;
}

void Apply(float x, float y)
{
    if (!std::isfinite(x)) x = 0.0f;
    if (!std::isfinite(y)) y = 0.0f;
    x = Clamp(x, -1.0f, 1.0f);
    y = Clamp(y, -1.0f, 1.0f);

    g_targetX = x;
    g_targetY = y;

    // Write eye ball parameters to the model
    Model::SetParameter("ParamEyeBallX", x);
    Model::SetParameter("ParamEyeBallY", y);
}

void GetTarget(float& x, float& y)
{
    x = g_targetX;
    y = g_targetY;
}

} // namespace EyeTracking
