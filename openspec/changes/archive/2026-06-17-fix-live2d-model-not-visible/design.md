## Context

The Live2D model loads successfully (verified: framework init, 10 motions, 2 textures, idle motion). The D3D11 device and swap chain are created. `Bridge_Render()` runs every frame. But the character is invisible — only the blue-border card background from the WPF XAML is visible in the app window.

Two rendering bugs prevent the model from appearing:

**Bug 1 — Projection matrix makes the model ~2 pixels tall (root cause)**

Current code in `bridge_api.cpp`:
```cpp
float scaleX = 2.0f / (float)w;   // = 2/200 = 0.01
float scaleY = 2.0f / (float)h;   // = 2/280 = 0.007
projection.Scale(scaleX, scaleY);
```

The Hiyori model's layout has `height=2.0` in model coordinates (standard Cubism convention). After `Scale(0.01, 0.007)`, the model occupies `2.0 × 0.007 = 0.014` in NDC space, which is `0.014 × 140 = ~2 pixels` on screen. Essentially invisible.

The official CubismNative D3D11 sample uses `Scale(1.0f, 1.0f)` as projection and lets the model matrix (which already has `SetHeight` from layout) handle all sizing.

**Bug 2 — No D3D11 blend state for alpha transparency**

`cubism_renderer.cpp` creates a depth stencil state but no blend state. D3D11 default is blend disabled. While `CubismRenderer_D3D11` sets blend states internally per-draw-call, we should also ensure the initial OM state supports alpha blending for any custom rendering.

## Goals / Non-Goals

**Goals:**
- Live2D model character is visible in the app window, correctly sized and positioned
- Transparent background — only the character pixels are visible, not a solid box
- Model fills the display area vertically while maintaining aspect ratio

**Non-Goals:**
- Model clicking/hit-testing (separate feature)
- Multiple model support
- Performance optimization

## Decisions

### Decision 1: Fix projection matrix to use Cubism-standard approach

**Choice**: Replace the custom `Scale(2/w, 2/h)` with identity projection, letting the model matrix (which already has layout from `model3.json`) handle all coordinate mapping.

**Rationale**: The CubismNative SDK samples all use this approach. The model matrix from `GetModelMatrix()` already contains center/width/height from the layout map. An identity projection maps model coordinates directly to NDC, and the model matrix positions the model to fill the desired area.

**Alternative considered**: Keep the custom projection but fix it to `Scale(screenHeight, screenHeight)`. This works but duplicates what the model matrix already does.

### Decision 2: Set model height to fill display area

After setting identity projection, explicitly call `modelMatrix->SetHeight(2.0f)` to ensure the model fills the vertical space (model coordinates are ±1 = 2 units tall, mapping to full NDC height = full screen height).

### Decision 3: Add D3D11 alpha blend state

Create and bind a standard alpha-blend state:
```cpp
SrcBlend = SRC_ALPHA, DestBlend = INV_SRC_ALPHA, BlendOp = ADD
SrcBlendAlpha = ONE, DestBlendAlpha = INV_SRC_ALPHA
```

Apply in `BeginFrame()` before any drawing.

## Risks / Trade-offs

- **[Risk] Model too large or clipped** → The model matrix `SetHeight(2.0f)` fills the full vertical space. If the model's aspect ratio doesn't match the display area, horizontal clipping may occur. Mitigation: use `SetWidth` with aspect-ratio-aware sizing instead.
- **[Risk] WPF HwndHost transparency not working** → The child HWND renders opaque D3D11 output. If the clear color alpha doesn't composite through to WPF, a black box may appear instead of transparent. Mitigation: the swap chain uses `DXGI_FORMAT_R8G8B8A8_UNORM` which includes alpha; WPF's HwndHost should composite it. If not, may need `WS_EX_LAYERED` on the child HWND.
