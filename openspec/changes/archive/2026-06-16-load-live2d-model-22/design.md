## Context

Core AI Pet currently displays a placeholder character using WPF Canvas (`PetCharacterControl`). The application has a complete Live2D rendering pipeline already built:

- **Live2DBridge C++ DLL**: Provides `Bridge_LoadModel` and other rendering functions via P/Invoke
- **Live2DBridgeWrapper**: C# wrapper with thread-safe lifecycle management
- **Live2DRenderHost**: Rendering host control that integrates with the bridge
- **ICharacterRenderer interface**: Abstraction allowing renderer swap (Canvas ↔ Live2D)

The Live2D services are implemented but commented out in `CompositionRoot.cs`. The target model directory (`C:\Change Top\git\open-source\live2d\model\22`) contains multiple JSON model files representing different character variants/outfits.

## Goals / Non-Goals

**Goals:**
- Enable Live2D model rendering by uncommenting and wiring up existing Live2D infrastructure
- Load the "22" model collection from the specified external directory
- Support runtime selection of different model variants (default, christmas, summer, etc.)
- Persist model path and variant selection in application configuration
- Maintain seamless fallback to Canvas placeholder if Live2D model fails to load

**Non-Goals:**
- Modifying the Live2DBridge C++ DLL (already supports model loading)
- Creating or editing Live2D model assets
- Supporting multiple simultaneous models (single model at a time)
- Implementing model download/installation features

## Decisions

### 1. Model Path Configuration Strategy

**Decision**: Store model directory path in app config (`config.json`), default to `C:\Change Top\git\open-source\live2d\model\22`

**Rationale**: 
- External models may be updated independently of the app
- Users may have models in different locations
- Keeps model assets out of the application binary directory
- Easy to change without code modifications

**Alternatives considered**:
- Hardcode path → Too inflexible
- Embed models in app resources → Increases binary size, harder to update
- Environment variable → Less user-friendly than config file

### 2. Model Variant Selection

**Decision**: Use `model.default.json` as the initial model, with a settings UI to switch variants at runtime

**Rationale**:
- The model directory contains 20+ variants (default, christmas, summer, etc.)
- Users should be able to customize appearance without restarting
- `model.default.json` is a sensible starting point

**Alternatives considered**:
- Load all variants simultaneously → Wastes memory, not practical
- Random variant on startup → Poor UX, no control
- Hardcode single variant → Inflexible

### 3. Renderer Initialization Strategy

**Decision**: Initialize Live2D renderer on app startup, fall back to Canvas placeholder on failure

**Rationale**:
- Live2D is the primary intended renderer
- Canvas placeholder serves as debug/fallback mode
- Graceful degradation prevents app from being unusable if Live2D fails

**Alternatives considered**:
- Canvas-only with optional Live2D → Defeats purpose of having Live2D
- Fail fast on Live2D error → Poor UX, app becomes unusable
- Manual renderer selection → Too complex for initial implementation

### 4. Service Lifecycle Management

**Decision**: Register Live2D services as singletons in DI container, initialize in `MainWindow.OnLoaded`

**Rationale**:
- Single renderer instance per application (singleton pattern)
- Window handle (HWND) required for renderer initialization is available after window loads
- Matches existing service patterns (ConfigService, PositionService)

**Alternatives considered**:
- Transient services → Multiple renderer instances would conflict
- Initialize in App.OnStartup → HWND not available yet
- Lazy initialization → Adds complexity without clear benefit

## Risks / Trade-offs

**[Risk] Live2D SDK not available** → **Mitigation**: The Live2DBridge DLL uses dual-mode architecture (mock/SDK). If SDK is not linked, it falls back to mock mode automatically. Document SDK setup requirements in README.

**[Risk] Model path does not exist** → **Mitigation**: Validate path on startup, log error, and fall back to Canvas placeholder. Provide clear error message in debug panel.

**[Risk] Model file corrupted or incompatible** → **Mitigation**: Wrap `Bridge_LoadModel` in try-catch, log detailed error, fall back to Canvas. Add model validation in debug panel.

**[Risk] Performance issues with large models** → **Mitigation**: Monitor FPS via `Bridge_GetFPS`, display in debug panel. If FPS drops below threshold, show warning. Future optimization: model LOD switching.

**[Trade-off] External model path vs embedded**: External path provides flexibility but requires users to manage model files separately. This is acceptable for the target audience (developers/power users).

**[Trade-off] Runtime model switching vs restart-required**: Runtime switching is more complex to implement but provides better UX. The complexity is justified since the infrastructure already supports model unloading/reloading.
