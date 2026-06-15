## ADDED Requirements

### Requirement: Drag character to reposition
The system SHALL allow the user to drag the character window by holding the left mouse button. The character SHALL follow the mouse at 60fps.

#### Scenario: Drag character to new position
- **WHEN** the user holds left mouse button on the character and moves the mouse
- **THEN** the character window follows the mouse cursor smoothly

#### Scenario: Position saved on release
- **WHEN** the user releases the mouse button after dragging
- **THEN** the window position (X, Y) is persisted to storage

### Requirement: Restore position on restart
The system SHALL restore the character window to its last saved position when the application restarts.

#### Scenario: Position restored after restart
- **WHEN** the application is restarted
- **THEN** the character appears at the same screen position as before closing

#### Scenario: Default position on first launch
- **WHEN** the application is launched for the first time (no saved position)
- **THEN** the character appears at the default position (bottom-right area)

### Requirement: Click-through mode
The system SHALL support a click-through mode where the character is visible but does not intercept mouse clicks. Toggle via Ctrl+Alt+P.

#### Scenario: Enable click-through
- **WHEN** the user presses Ctrl+Alt+P
- **THEN** the character remains visible but mouse clicks pass through to windows behind it

#### Scenario: Disable click-through
- **WHEN** the user presses Ctrl+Alt+P again
- **THEN** the character resumes normal mouse interaction

### Requirement: Window transparency and always-on-top
The main window SHALL be borderless, transparent-background, and always-on-top. The window SHALL NOT appear in the taskbar.

#### Scenario: Transparent borderless window
- **WHEN** the application is running
- **THEN** only the Live2D character is visible, with no window chrome or background
