cmake_minimum_required(VERSION 3.20)
project(Live2DBridge LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ============================================================
# Live2D Cubism Native SDK (通过 git submodule 引入)
# 下载 SDK 后放到 vendor/Live2DCubismSdk/，并取消下面的注释：
#
# set(LIVE2D_SDK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/vendor/Live2DCubismSdk)
# add_compile_definitions(LIVE2D_HAS_SDK=1)
# ============================================================

set(BRIDGE_SOURCES
    src/dllmain.cpp
    src/bridge_api.cpp
    src/cubism_model.cpp
    src/cubism_renderer.cpp
    src/cubism_animation.cpp
    src/eye_tracking.cpp
)

add_library(Live2DBridge SHARED ${BRIDGE_SOURCES})

target_include_directories(Live2DBridge PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    # ${LIVE2D_SDK_DIR}/include   # 启用 SDK 时取消注释
)

target_compile_definitions(Live2DBridge PRIVATE LIVE2DBRIDGE_EXPORTS)

# 链接 Direct3D 11 (渲染用) + GDI32 (占位渲染)
target_link_libraries(Live2DBridge PRIVATE
    d3d11
    dxgi
    gdi32
    user32
    # ${LIVE2D_SDK_DIR}/lib/Live2DCubismCore.lib   # 启用 SDK 时取消注释
)

# 使用 def 文件显式导出符号，避免 name mangling 问题
set_target_properties(Live2DBridge PROPERTIES
    LINK_FLAGS "/DEF:\"${CMAKE_CURRENT_SOURCE_DIR}/Live2DBridge.def\""
)

# 输出到 Desktop 项目的 bin 目录
set_target_properties(Live2DBridge PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../CoreAIpet.Desktop/bin/$<CONFIG>/net8.0-windows"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../CoreAIpet.Desktop/bin/$<CONFIG>/net8.0-windows"
)
