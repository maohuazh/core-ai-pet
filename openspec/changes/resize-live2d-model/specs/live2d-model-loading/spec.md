## MODIFIED Requirements

### Requirement: System SHALL initialize Live2D renderer on window load
The system SHALL initialize the Live2D renderer when the main window loads. The renderer SHALL use offscreen rendering with pixel readback for display in the transparent WPF window. The window dimensions SHALL match the model display area (100×100 pixels by default).

#### Scenario: Initialize renderer on window load
- **WHEN** the main window finishes loading (`OnLoaded` event)
- **THEN** the system SHALL call `Bridge_Initialize` to initialize the SDK
- **THEN** the system SHALL call `Bridge_InitializeRenderer` with width=100 and height=100
- **THEN** the renderer SHALL be ready to display the Live2D model
- **THEN** the window SHALL be sized to 100×100 pixels to match the display area

#### Scenario: Renderer initialization fails
- **WHEN** `Bridge_Initialize` or `Bridge_InitializeRenderer` returns false
- **THEN** the system SHALL log the error with diagnostic information
- **THEN** the system SHALL fall back to the Canvas placeholder
- **THEN** the application SHALL continue running with reduced functionality

### Requirement: System SHALL load Live2D model from configured path
The system SHALL load a Live2D model from the path specified in the application configuration on startup. The model path SHALL be read from `config.json` under the key `live2d.modelPath`. If no path is configured, the system SHALL use the default path pointing to the Mao model directory (`vendor\models\Mao`). If Mao is not available, the system SHALL fall back to the Hiyori model.

#### Scenario: Load model from configured path
- **WHEN** the application starts and a model path is configured in `config.json`
- **THEN** the system SHALL call `Bridge_LoadModel` with the configured path
- **THEN** the Live2D model SHALL be rendered in the main window

#### Scenario: Load Mao model by default
- **WHEN** the application starts and no model path is configured
- **THEN** the system SHALL use the default path `vendor\models\Mao` (relative to application base directory)
- **THEN** the system SHALL attempt to load `Mao.model3.json` from that directory

#### Scenario: Fall back to Hiyori when Mao unavailable
- **WHEN** the Mao model directory does not exist
- **THEN** the system SHALL fall back to the Hiyori model at `vendor\models\Hiyori`
- **THEN** the system SHALL log a warning about Mao being unavailable

#### Scenario: Model path does not exist
- **WHEN** neither the Mao nor Hiyori model directory exists
- **THEN** the system SHALL log an error
- **THEN** the system SHALL fall back to displaying the WPF Canvas placeholder character
- **THEN** the application SHALL remain functional for other features
