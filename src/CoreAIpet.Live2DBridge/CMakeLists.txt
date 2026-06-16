cmake_minimum_required(VERSION 3.20)
project(Live2DBridge LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# MSVC: UTF-8 source encoding
if(MSVC)
    add_compile_options(/utf-8)
endif()

# ============================================================
# Paths
# ============================================================
set(FRAMEWORK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/vendor/CubismNativeFramework)
set(CORE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/vendor/Core)

# ============================================================
# Enable SDK mode
# ============================================================
add_compile_definitions(LIVE2D_HAS_SDK=1)

# ============================================================
# Framework sources (D3D11 only)
# ============================================================
set(FRAMEWORK_SOURCES
    ${FRAMEWORK_DIR}/src/CubismCdiJson.cpp
    ${FRAMEWORK_DIR}/src/CubismDefaultParameterId.cpp
    ${FRAMEWORK_DIR}/src/CubismFramework.cpp
    ${FRAMEWORK_DIR}/src/CubismModelSettingJson.cpp
    ${FRAMEWORK_DIR}/src/Effect/CubismBreath.cpp
    ${FRAMEWORK_DIR}/src/Effect/CubismEyeBlink.cpp
    ${FRAMEWORK_DIR}/src/Effect/CubismLook.cpp
    ${FRAMEWORK_DIR}/src/Effect/CubismPose.cpp
    ${FRAMEWORK_DIR}/src/Id/CubismId.cpp
    ${FRAMEWORK_DIR}/src/Id/CubismIdManager.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismMath.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismMatrix44.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismModelMatrix.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismTargetPoint.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismVector2.cpp
    ${FRAMEWORK_DIR}/src/Math/CubismViewMatrix.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismMoc.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismModel.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismModelMultiplyAndScreenColor.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismModelUserData.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismModelUserDataJson.cpp
    ${FRAMEWORK_DIR}/src/Model/CubismUserModel.cpp
    ${FRAMEWORK_DIR}/src/Motion/ACubismMotion.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismBreathUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismExpressionMotion.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismExpressionMotionManager.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismExpressionUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismEyeBlinkUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismLipSyncUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismLookUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismMotion.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismMotionJson.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismMotionManager.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismMotionQueueEntry.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismMotionQueueManager.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismPhysicsUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismPoseUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/CubismUpdateScheduler.cpp
    ${FRAMEWORK_DIR}/src/Motion/ICubismUpdater.cpp
    ${FRAMEWORK_DIR}/src/Motion/IParameterProvider.cpp
    ${FRAMEWORK_DIR}/src/Physics/CubismPhysics.cpp
    ${FRAMEWORK_DIR}/src/Physics/CubismPhysicsJson.cpp
    ${FRAMEWORK_DIR}/src/Rendering/CubismRenderer.cpp
    ${FRAMEWORK_DIR}/src/Rendering/csmBlendMode.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismDeviceInfo_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismOffscreenManager_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismOffscreenRenderTarget_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismOffscreenSurface_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismRenderState_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismRenderTarget_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismRenderer_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Rendering/D3D11/CubismShader_D3D11.cpp
    ${FRAMEWORK_DIR}/src/Type/csmRectF.cpp
    ${FRAMEWORK_DIR}/src/Type/csmString.cpp
    ${FRAMEWORK_DIR}/src/Utils/CubismDebug.cpp
    ${FRAMEWORK_DIR}/src/Utils/CubismJson.cpp
    ${FRAMEWORK_DIR}/src/Utils/CubismString.cpp
)

# ============================================================
# Our sources
# ============================================================
set(BRIDGE_SOURCES
    src/dllmain.cpp
    src/bridge_api.cpp
    src/cubism_model.cpp
    src/cubism_renderer.cpp
    src/cubism_animation.cpp
    src/eye_tracking.cpp
    src/cubism_allocator.cpp
)

# ============================================================
# Build DLL
# ============================================================
add_library(Live2DBridge SHARED ${BRIDGE_SOURCES} ${FRAMEWORK_SOURCES})

target_include_directories(Live2DBridge PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${FRAMEWORK_DIR}/src
    ${CORE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/vendor/stb
)

target_compile_definitions(Live2DBridge PRIVATE LIVE2DBRIDGE_EXPORTS)

target_link_libraries(Live2DBridge PRIVATE
    d3d11
    dxgi
    d3dcompiler
    gdi32
    user32
    ole32
    windowscodecs
    ${CORE_DIR}/lib/windows/x86_64/Live2DCubismCore.lib
)

# Export symbols
set_target_properties(Live2DBridge PROPERTIES
    LINK_FLAGS "/DEF:\"${CMAKE_CURRENT_SOURCE_DIR}/Live2DBridge.def\""
)

# Output to Desktop project bin directory
set_target_properties(Live2DBridge PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../CoreAIpet.Desktop/bin/$<CONFIG>/net8.0-windows"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../CoreAIpet.Desktop/bin/$<CONFIG>/net8.0-windows"
)

# Copy Core DLL to output directory
add_custom_command(TARGET Live2DBridge POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${CORE_DIR}/dll/windows/x86_64/Live2DCubismCore.dll"
        "$<TARGET_FILE_DIR:Live2DBridge>/Live2DCubismCore.dll"
    COMMENT "Copying Live2DCubismCore.dll to output"
)

# Copy D3D11 shader files to output directory
add_custom_command(TARGET Live2DBridge POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory "$<TARGET_FILE_DIR:Live2DBridge>/FrameworkShaders"
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${FRAMEWORK_DIR}/src/Rendering/D3D11/Shaders/CubismEffect.fx"
        "$<TARGET_FILE_DIR:Live2DBridge>/FrameworkShaders/CubismEffect.fx"
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${FRAMEWORK_DIR}/src/Rendering/D3D11/Shaders/CubismBlendMode.fx"
        "$<TARGET_FILE_DIR:Live2DBridge>/FrameworkShaders/CubismBlendMode.fx"
    COMMENT "Copying D3D11 shader files to output"
)
