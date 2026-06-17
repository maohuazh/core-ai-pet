## MODIFIED Requirements

### Requirement: Window dimensions SHALL match model display area
The system SHALL size the main window to exactly match the Live2D model display dimensions (150×150 pixels by default). No additional padding or margins SHALL exist around the model area.

#### Scenario: Window size matches model size
- **WHEN** the application starts with default model dimensions
- **THEN** the window width SHALL be 150 pixels
- **THEN** the window height SHALL be 150 pixels
- **THEN** the Live2D model SHALL fill the entire window area

#### Scenario: No padding around model
- **WHEN** the Live2D model is rendering
- **THEN** there SHALL be no margin between the window edge and the model display area
- **THEN** the model SHALL occupy the full window dimensions
