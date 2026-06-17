// ============================================================
// cubism_animation.cpp — Motion playback control
// ============================================================

#include "bridge_internal.h"
#include <algorithm>
#include <cstdlib>
#include <ctime>

#ifdef LIVE2D_HAS_SDK
using namespace Live2D::Cubism::Framework;
#endif

namespace Animation {

static std::string g_currentGroup;
static std::string g_currentName;
static int g_lastMotionIndex = -1; // Track last played motion to avoid repeats
static bool g_seeded = false;

void Initialize()
{
    g_currentGroup = "idle";
    g_currentName = "";
    g_lastMotionIndex = -1;
    if (!g_seeded)
    {
        std::srand((unsigned)std::time(nullptr));
        g_seeded = true;
    }
}

// Convert string to lowercase
static std::string ToLower(const std::string& s)
{
    std::string result = s;
    std::transform(result.begin(), result.end(), result.begin(),
        [](unsigned char c) { return (char)std::tolower(c); });
    return result;
}

void SetMotion(const char* group, const char* name)
{
    if (!group) return;

    std::string groupLower = ToLower(group);

    // Skip if same group and same name (avoid redundant triggers)
    if (g_currentGroup == groupLower && (!name || g_currentName == name)) return;

    g_currentGroup = groupLower;
    g_currentName = name ? name : "";

#ifdef LIVE2D_HAS_SDK
    if (!Model::IsLoaded()) return;

    const auto& index = Model::GetMotionIndex();
    if (index.empty()) return;

    // Find the group in the index (case-insensitive — keys are already lowercase)
    auto it = index.find(groupLower);

    // Fallback to "idle" if group not found
    if (it == index.end() || it->second.empty())
    {
        it = index.find("idle");
        if (it == index.end())
        {
            // Try first available group
            it = index.begin();
        }
        if (it == index.end() || it->second.empty()) return;

        char logBuf[256];
        snprintf(logBuf, sizeof(logBuf), "[Animation] Group '%s' not found, falling back to '%s'",
            group, it->first.c_str());
        BridgeLog(logBuf);
    }

    const auto& motionIndices = it->second;
    if (motionIndices.empty()) return;

    // Select a motion: random, avoiding the last played one if possible
    int selectedIdx;
    if (motionIndices.size() == 1)
    {
        selectedIdx = motionIndices[0];
    }
    else
    {
        // Pick randomly, avoiding last motion
        int attempts = 0;
        do {
            selectedIdx = motionIndices[std::rand() % motionIndices.size()];
            attempts++;
        } while (selectedIdx == g_lastMotionIndex && attempts < 5);
    }

    // Get the motion pointer
    ACubismMotion* motion = Model::GetMotion(selectedIdx);
    if (!motion) return;

    // Get the motion manager
    auto* motionMgr = Model::GetMotionManager();
    if (!motionMgr) return;

    // Start the motion with priority 3 (normal priority)
    motionMgr->StartMotionPriority(motion, false, 3);
    g_lastMotionIndex = selectedIdx;

    char logBuf[256];
    snprintf(logBuf, sizeof(logBuf), "[Animation] Playing motion %d from group '%s'",
        selectedIdx, it->first.c_str());
    BridgeLog(logBuf);
#endif
}

const std::string& CurrentGroup() { return g_currentGroup; }
const std::string& CurrentName() { return g_currentName; }

} // namespace Animation
