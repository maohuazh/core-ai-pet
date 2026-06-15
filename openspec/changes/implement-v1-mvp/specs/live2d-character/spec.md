## ADDED Requirements

### Requirement: Live2D character display on transparent borderless window
The system SHALL display a Live2D character in a borderless, transparent, always-on-top window. The window SHALL have no title bar and SHALL NOT appear in the taskbar.

#### Scenario: Character visible on desktop
- **WHEN** the application starts
- **THEN** a Live2D character is visible on the desktop in a transparent window with no borders

#### Scenario: Window stays on top
- **WHEN** other windows are opened or focused
- **THEN** the character window remains on top of all other windows

### Requirement: Character state animations
The system SHALL support four character states with corresponding animations: Idle (breathing, blinking, slight sway), Happy (hover response), Thinking (AI processing), Talking (AI replying).

#### Scenario: Idle animation loops
- **WHEN** the character is in Idle state with no user interaction
- **THEN** the idle animation (breathing + blinking + sway) plays in a continuous loop

#### Scenario: Transition to Happy on hover
- **WHEN** the mouse enters the character area
- **THEN** the character switches to Happy animation

#### Scenario: Transition to Thinking on message send
- **WHEN** the user sends a chat message
- **THEN** the character switches to Thinking animation

#### Scenario: Transition to Talking on response
- **WHEN** the AI starts returning a response
- **THEN** the character switches to Talking animation

#### Scenario: Return to Idle after response
- **WHEN** the AI response is complete and 3 seconds have elapsed
- **THEN** the character returns to Idle state

### Requirement: Eye tracking follows mouse cursor
The system SHALL make the character's eyes follow the mouse cursor position, limited to ±30° horizontal and ±15° vertical to prevent model deformation.

#### Scenario: Eyes follow mouse movement
- **WHEN** the mouse moves within the screen bounds
- **THEN** the character's eyes track the mouse position within the angle limits

#### Scenario: Eye tracking stays within limits
- **WHEN** the mouse is at the extreme edge of the screen
- **THEN** the eye rotation does not exceed ±30° horizontal or ±15° vertical
