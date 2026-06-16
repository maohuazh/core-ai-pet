## Why

The Live2D model loads successfully (verified via debug logs: framework initialized, 10 motions loaded, 2 textures bound, idle motion started), but the character is not visible in the app window — only a blue-background box appears. The D3D11 rendering pipeline is functional but the model's visual output is not reaching the screen.

## What Changes

- Diagnose and fix the rendering pipeline so the Live2D model character is visible in the desktop pet window
- Likely areas: projection matrix setup, model matrix scaling/positioning, render state (alpha blending), swap chain presentation, or WPF HwndHost transparency/compositing
- Ensure the transparent background works correctly so only the character is visible (not a solid color box)

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `live2d-model-loading`: The rendering output of the loaded model must be visually correct — model character must appear on screen with transparent background, not a solid color box

## Impact

- **C++ rendering code**: `cubism_renderer.cpp` (D3D11 state, clear color, blend state), `bridge_api.cpp` (projection matrix, draw call), `cubism_model.cpp` (model matrix setup)
- **C# WPF layer**: `Live2DHostControl.cs` (HWND child window style, transparency), `MainWindow.xaml` (background, AllowsTransparency interaction with child HWND)
- **Shader files**: D3D11 blend/alpha state may need adjustment for proper transparency
- **No API or config changes**: This is a rendering fix, no external interface changes
