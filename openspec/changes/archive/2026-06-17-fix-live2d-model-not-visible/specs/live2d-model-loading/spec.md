## ADDED Requirements

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
- **THEN** the swap chain clear color SHALL be (0,0,0,0) — fully transparent black
