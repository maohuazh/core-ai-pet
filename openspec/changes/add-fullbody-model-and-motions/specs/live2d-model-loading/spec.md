## MODIFIED Requirements

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
