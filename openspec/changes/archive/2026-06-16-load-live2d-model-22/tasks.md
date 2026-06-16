## 1. Configuration Setup

- [x] 1.1 Add `live2d` section to `config.json` schema with `modelPath` and `modelVariant` fields
- [x] 1.2 Update `ConfigService` to include Live2D configuration properties
- [x] 1.3 Set default model path to `C:\Change Top\git\open-source\live2d\model\22` in config initialization

## 2. Service Registration and Lifecycle

- [x] 2.1 Uncomment Live2D service registrations in `CompositionRoot.cs` (Live2DBridgeWrapper, Live2DRenderHost)
- [x] 2.2 Uncomment CharacterController registration in `CompositionRoot.cs`
- [x] 2.3 Add `OnLoaded` event handler in `MainWindow.xaml.cs` to initialize Live2D renderer
- [x] 2.4 Call `Bridge_Initialize` and `Bridge_InitializeRenderer` in `OnLoaded` with window HWND
- [x] 2.5 Implement error handling in `OnLoaded` to fall back to Canvas placeholder on failure

## 3. Model Loading Implementation

- [x] 3.1 Create `ModelLoaderService` class to handle model loading logic
- [x] 3.2 Implement `LoadModel` method that reads model path from config and calls `Bridge_LoadModel`
- [x] 3.3 Implement `LoadVariant` method to load a specific variant JSON file
- [x] 3.4 Add path validation to check if model directory exists before loading
- [x] 3.5 Implement fallback logic to use Canvas placeholder if model loading fails
- [x] 3.6 Add logging for model loading success/failure with detailed error messages

## 4. Model Variant Management

- [x] 4.1 Implement `GetAvailableVariants` method to scan model directory for `.json` files
- [x] 4.2 Parse variant filenames to generate user-friendly display names (e.g., "model.2016.xmas.1.json" → "Christmas 2016 Variant 1")
- [x] 4.3 Register `ModelLoaderService` in DI container as singleton

## 5. Settings UI Integration

- [x] 5.1 Create Settings window/dialog XAML with model variant selection dropdown
- [x] 5.2 Implement settings view model to load available variants from `ModelLoaderService`
- [x] 5.3 Add variant selection change handler to call `Bridge_UnloadModel` and `Bridge_LoadModel`
- [x] 5.4 Save selected variant to config when user changes selection
- [x] 5.5 Add error message display in settings UI for invalid model paths

## 6. Character Controller Integration

- [x] 6.1 Connect `CharacterController` state changes to `Bridge_SetMotionGroup` calls
- [x] 6.2 Map character states (idle, happy, thinking, talking) to Live2D motion group names
- [x] 6.3 Connect mouse movement in `MainWindow` to `Bridge_SetEyeTarget` calls
- [x] 6.4 Ensure eye tracking coordinates are properly normalized (-1 to 1)

## 7. Debug and Diagnostics

- [x] 7.1 Update debug panel to display current renderer name (Live2D vs Canvas)
- [x] 7.2 Add FPS display from `Bridge_GetFPS` to debug panel
- [x] 7.3 Add model loading status and error messages to debug panel
- [x] 7.4 Implement real-time FPS update in debug panel

## 8. MainWindow Renderer Switching

- [x] 8.1 Update `MainWindow.xaml` to conditionally show Live2D renderer or Canvas placeholder
- [x] 8.2 Implement renderer detection logic to determine which renderer is active
- [x] 8.3 Ensure `CharacterDisplay` control references work with both renderer types

## 9. Testing and Validation

- [x] 9.1 Test model loading with default model variant
- [x] 9.2 Test model variant switching via settings UI
- [x] 9.3 Test fallback to Canvas placeholder when model path is invalid
- [x] 9.4 Test fallback to Canvas placeholder when Live2D SDK is not available
- [x] 9.5 Test eye tracking with Live2D model
- [x] 9.6 Test state animation transitions with Live2D model
- [x] 9.7 Test configuration persistence across application restarts
- [x] 9.8 Test FPS monitoring in debug panel

## 10. Documentation

- [x] 10.1 Update README with Live2D SDK setup instructions
- [x] 10.2 Document default model path configuration
- [x] 10.3 Add troubleshooting section for common model loading issues
