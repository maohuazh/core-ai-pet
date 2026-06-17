## ADDED Requirements

### Requirement: Display area SHALL be 100×100 pixels
The system SHALL render the Live2D model in a 100×100 pixel square display area. The render target, WriteableBitmap, and window dimensions SHALL all be 100×100 pixels.

#### Scenario: Window size is 100×100
- **WHEN** the application starts with default configuration
- **THEN** the main window width SHALL be 100 pixels
- **THEN** the main window height SHALL be 100 pixels
- **THEN** the Live2D render target SHALL be 100×100 pixels
- **THEN** the WriteableBitmap SHALL be 100×100 pixels

#### Scenario: Fallback dimensions are 100×100
- **WHEN** `ActualWidth` or `ActualHeight` of the display control returns 0 or negative
- **THEN** the system SHALL fall back to width=100 and height=100
- **THEN** the renderer SHALL initialize with 100×100 dimensions

### Requirement: Projection SHALL fit model in square viewport
The system SHALL adjust the Live2D projection matrix so the model is centered and fits within the 100×100 square viewport without distortion. The model SHALL be scaled to fit the smaller of the two dimensions (width or height).

#### Scenario: Model centered in square viewport
- **WHEN** the Live2D model is rendered in the 100×100 viewport
- **THEN** the model SHALL be horizontally and vertically centered
- **THEN** the model SHALL fit entirely within the viewport (no clipping)
- **THEN** the model SHALL NOT be stretched or distorted

### Requirement: Eye tracking SHALL use square coordinate space
The system SHALL calculate eye tracking coordinates using the 100×100 square display area. The normalized coordinates SHALL range from -1 to 1 on both axes relative to the center of the square.

#### Scenario: Mouse position normalized to square area
- **WHEN** the user moves the mouse over the 100×100 display area
- **THEN** the system SHALL calculate `nx = (mouseX - 50) / 50`
- **THEN** the system SHALL calculate `ny = (mouseY - 50) / 50`
- **THEN** the coordinates SHALL be passed to `Bridge_SetEyeTarget`

### Requirement: Default display size SHALL be defined as constant
The system SHALL define the default display dimensions as named constants (`DefaultDisplayWidth = 100`, `DefaultDisplayHeight = 100`) to avoid magic numbers. All C# code referencing the display size SHALL use these constants.

#### Scenario: Constants used in C# code
- **WHEN** C# code needs the default display width or height
- **THEN** it SHALL reference `DisplayConstants.DefaultDisplayWidth` or `DisplayConstants.DefaultDisplayHeight`
- **THEN** the values SHALL be 100

### Requirement: Native renderer fallback SHALL use 100×100
The native bridge code SHALL use 100×100 as the fallback dimensions when invalid width/height values are provided.

#### Scenario: Native fallback dimensions
- **WHEN** `Bridge_InitializeRenderer` is called with width <= 0 or height <= 0
- **THEN** the width SHALL default to 100
- **THEN** the height SHALL default to 100
