## MODIFIED Requirements

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
