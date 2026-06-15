cmake_minimum_required(VERSION 3.20)
project(Live2DBridge LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Live2D Cubism Native SDK (通过 git submodule 引入)
# set(LIVE2D_SDK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/vendor/Live2DCubismSdk)

add_library(Live2DBridge SHARED
    src/dllmain.cpp
    src/bridge_api.cpp
    # src/cubism_model.cpp      # TODO: 实现
    # src/cubism_renderer.cpp   # TODO: 实现
    # src/cubism_animation.cpp  # TODO: 实现
    # src/eye_tracking.cpp      # TODO: 实现
)

target_include_directories(Live2DBridge PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    # ${LIVE2D_SDK_DIR}/include
)

target_compile_definitions(Live2DBridge PRIVATE LIVE2DBRIDGE_EXPORTS)

# 链接 Direct3D 11 (渲染用)
target_link_libraries(Live2DBridge PRIVATE
    d3d11
    dxgi
    # ${LIVE2D_SDK_DIR}/lib/Live2DCubismCore.lib
)

# 输出到 Desktop 项目的 bin 目录
set_target_properties(Live2DBridge PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/../CoreAIpet.Desktop/bin/$<CONFIG>/net8.0-windows"
)
