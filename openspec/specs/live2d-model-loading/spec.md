# Live2D Model Loading

## Purpose

This capability handles loading, initializing, and managing Live2D models from external file paths. It supports model variant selection, runtime switching, configuration persistence, and graceful fallback to placeholder renderers when Live2D is unavailable.

## Requirements

### Requirement: System SHALL load Live2D model from configured path
The system SHALL load a Live2D model from the path specified in the application configuration on startup. The model path SHALL be read from `config.json` under the key `live2d.modelPath`. If no path is configured, the system SHALL use the default path `C:\Change Top\git\open-source\live2d\model\22`.

#### Scenario: Load model from configured path
- **WHEN** the application starts and a model path is configured in `config.json`
- **THEN** the system SHALL call `Bridge_LoadModel` with the configured path
- **THEN** the Live2D model SHALL be rendered in the main window

#### Scenario: Load model from default path when not configured
- **WHEN** the application starts and no model path is configured
- **THEN** the system SHALL use the default path `C:\Change Top\git\open-source\live2d\model\22`
- **THEN** the system SHALL attempt to load `model.default.json` from that directory

#### Scenario: Model path does not exist
- **WHEN** the configured model path does not exist on the filesystem
- **THEN** the system SHALL log an error with the invalid path
- **THEN** the system SHALL fall back to displaying the WPF Canvas placeholder character
- **THEN** the application SHALL remain functional for other features

### Requirement: System SHALL support model variant selection
The system SHALL allow users to select different model variants from the model directory at runtime. Each variant is a separate JSON file in the model directory (e.g., `model.default.json`, `model.2016.xmas.1.json`, `model.2017.summer.super.1.json`).

#### Scenario: Switch model variant via settings
- **WHEN** user opens the settings dialog and selects a different model variant
- **THEN** the system SHALL call `Bridge_UnloadModel` to release the current model
- **THEN** the system SHALL call `Bridge_LoadModel` with the new variant path
- **THEN** the new model variant SHALL be rendered without requiring application restart

#### Scenario: List available model variants
- **WHEN** user opens the settings dialog
- **THEN** the system SHALL scan the model directory for all `.json` files
- **THEN** the system SHALL display a list of available variants with friendly names (derived from filenames)

#### Scenario: Selected variant file is missing
- **WHEN** user selects a model variant that no longer exists
- **THEN** the system SHALL log an error
- **THEN** the system SHALL fall back to the Canvas placeholder
- **THEN** the settings dialog SHALL show an error message to the user

### Requirement: System SHALL persist model configuration
The system SHALL save the selected model path and variant to `config.json` so that the user's choice persists across application restarts.

#### Scenario: Save model configuration on variant change
- **WHEN** user selects a new model variant in settings
- **THEN** the system SHALL update `config.json` with the new model path and variant
- **THEN** the configuration SHALL be written to disk immediately

#### Scenario: Load persisted configuration on startup
- **WHEN** the application starts
- **THEN** the system SHALL read the model path and variant from `config.json`
- **THEN** the system SHALL load the previously selected variant

### Requirement: System SHALL initialize Live2D renderer on window load
The system SHALL initialize the Live2D renderer when the main window loads. The renderer SHALL use offscreen rendering with pixel readback for display in the transparent WPF window. The window dimensions SHALL match the model display area (150×150 pixels by default).

#### Scenario: Initialize renderer on window load
- **WHEN** the main window finishes loading (`OnLoaded` event)
- **THEN** the system SHALL call `Bridge_Initialize` to initialize the SDK
- **THEN** the system SHALL call `Bridge_InitializeRenderer` with width=150 and height=150
- **THEN** the renderer SHALL be ready to display the Live2D model
- **THEN** the window SHALL be sized to 150×150 pixels to match the display area

#### Scenario: Renderer initialization fails
- **WHEN** `Bridge_Initialize` or `Bridge_InitializeRenderer` returns false
- **THEN** the system SHALL log the error with diagnostic information
- **THEN** the system SHALL fall back to the Canvas placeholder
- **THEN** the application SHALL continue running with reduced functionality

### Requirement: System SHALL integrate Live2D with character controller
The system SHALL connect the Live2D renderer to the existing `CharacterController` so that state changes (idle, happy, thinking, talking) trigger corresponding Live2D motion playback. The motion group mapping SHALL be adapted to the actual model's available motion groups. Eye tracking SHALL continue to work with the Live2D model.

#### Scenario: State changes trigger Live2D motions
- **WHEN** the character state changes (e.g., from idle to happy)
- **THEN** the system SHALL call `Bridge_SetMotionGroup` with the mapped motion group name
- **THEN** the Live2D model SHALL play the corresponding motion animation
- **THEN** the motion transition SHALL use fade-in/fade-out

#### Scenario: State mapping adapts to model's motion groups
- **WHEN** the character state changes and the mapped group exists in the model
- **THEN** the system SHALL use the model's actual motion group names (e.g., "Idle", "TapBody")
- **THEN** if the mapped group does not exist, the system SHALL fall back to the Idle group

#### Scenario: Eye tracking works with Live2D model
- **WHEN** the user moves the mouse over the main window
- **THEN** the system SHALL calculate normalized eye target coordinates (-1 to 1)
- **THEN** the system SHALL call `Bridge_SetEyeTarget` with the coordinates
- **THEN** the Live2D model's eyes SHALL follow the mouse cursor

### Requirement: System SHALL provide model loading status feedback
The system SHALL provide clear feedback to the user about model loading status, including success, failure, and fallback scenarios.

#### Scenario: Model loads successfully
- **WHEN** the Live2D model loads successfully
- **THEN** the system SHALL display the model in the main window
- **THEN** the debug panel SHALL show the renderer name as "Live2D Cubism SDK"

#### Scenario: Model loading fails and falls back to Canvas
- **WHEN** model loading fails for any reason
- **THEN** the system SHALL display the Canvas placeholder character
- **THEN** the debug panel SHALL show the renderer name as "WpfCanvas (Mock)"
- **THEN** the debug panel SHALL display an error message explaining the failure

#### Scenario: FPS monitoring for Live2D model
- **WHEN** the Live2D model is rendering
- **THEN** the debug panel SHALL display the current FPS from `Bridge_GetFPS`
- **THEN** the FPS value SHALL update in real-time

### Requirement: Live2D model SHALL render visibly in the application window
The system SHALL render the loaded Live2D model character so it is visible on screen. The projection matrix SHALL map model coordinates to screen space such that the model fills the display area vertically. The D3D11 render pipeline SHALL use alpha blending so the background is transparent and only the character pixels are visible.

#### Scenario: Model is visible after loading
- **WHEN** the Live2D model finishes loading and the render loop is running
- **THEN** the model character SHALL be visible in the Live2DDisplay area of the main window
- **THEN** the area around the character SHALL be transparent (showing the desktop behind the app)
- **THEN** the model SHALL occupy the full vertical height of the display area

#### Scenario: Projection maps model coordinates correctly
- **WHEN** the model has a layout height of 2.0 (standard Cubism convention)
- **THEN** the projection SHALL map this to the full vertical extent of the render target
- **THEN** the model SHALL NOT be invisible (less than 10 pixels) or microscopically small

#### Scenario: Alpha blending is enabled
- **WHEN** the D3D11 renderer draws model triangles
- **THEN** the output merger SHALL use source-alpha / inverse-source-alpha blending
- **THEN** transparent pixels in the model texture SHALL produce transparent output
- **THEN** the render target clear color SHALL be (0,0,0,0) — fully transparent black
