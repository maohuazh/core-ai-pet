## 1. Fix Projection Matrix

- [x] 1.1 In `bridge_api.cpp` `Bridge_Render()`, replace `Scale(2.0f/w, 2.0f/h)` with identity projection — remove the scaleX/scaleY calculations and `projection.Scale()` call entirely
- [x] 1.2 After getting `modelMatrix`, call `modelMatrix->SetHeight(2.0f)` to ensure model fills vertical space (matching standard Cubism convention)

## 2. Add Alpha Blend State

- [x] 2.1 In `cubism_renderer.cpp` `Initialize()`, create a D3D11 blend state with SrcBlend=SRC_ALPHA, DestBlend=INV_SRC_ALPHA, BlendOp=ADD
- [x] 2.2 In `cubism_renderer.cpp` `BeginFrame()`, bind the blend state via `OMSetBlendState()` before any drawing
- [x] 2.3 In `cubism_renderer.cpp` `Shutdown()`, release the blend state

## 3. Verify

- [x] 3.1 Rebuild C++ DLL and C# project, launch app
- [x] 3.2 Confirm Live2D model character is visible in the app window (not just the blue border box)
- [x] 3.3 Confirm transparent background — desktop shows through around the character
