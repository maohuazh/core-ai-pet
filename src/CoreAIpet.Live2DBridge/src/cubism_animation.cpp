// ============================================================
// cubism_animation.cpp — Motion playback control
// ============================================================
// The actual motion playback is handled by CubismMotionManager
// inside the model's Update(). This module just tracks which
// motion group is requested.

#include "bridge_internal.h"

namespace Animation {

static std::string g_currentGroup;
static std::string g_currentName;

void Initialize()
{
    g_currentGroup = "idle";
    g_currentName = "";
}

void SetMotion(const char* group, const char* name)
{
    if (!group) return;
    if (g_currentGroup == group && (!name || g_currentName == name)) return;

    g_currentGroup = group;
    g_currentName = name ? name : "";

    // TODO: Map group/name to loaded ACubismMotion and start via motion manager.
    // For now, motion playback is triggered during model loading (idle motion).
}

const std::string& CurrentGroup() { return g_currentGroup; }
const std::string& CurrentName() { return g_currentName; }

} // namespace Animation
