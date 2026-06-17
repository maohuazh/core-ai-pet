## ADDED Requirements

### Requirement: System SHALL build motion group index on model load
The system SHALL build a motion group index table during model loading. The index SHALL map each motion group name (case-insensitive) to a list of motion indices in the global motions array. The index SHALL be available for lookup immediately after model loading completes.

#### Scenario: Index built after model load
- **WHEN** a Live2D model finishes loading (after `Bridge_LoadModel` returns true)
- **THEN** the system SHALL have built a motion group index from all loaded motions
- **THEN** each motion group name from `model3.json` SHALL be a key in the index
- **THEN** each key SHALL map to the list of motion indices belonging to that group

#### Scenario: Index supports case-insensitive lookup
- **WHEN** a motion group is looked up with name "idle" or "Idle" or "IDLE"
- **THEN** the system SHALL return the same result regardless of case

### Requirement: System SHALL play motion by group name
The system SHALL play a Live2D motion when `SetMotion(group, name)` is called with a valid group name. The motion SHALL be selected from the specified group and played through the `CubismMotionManager`.

#### Scenario: Play motion from existing group
- **WHEN** `SetMotion` is called with a group name that exists in the index
- **THEN** the system SHALL select a motion from that group
- **THEN** the system SHALL call `CubismMotionManager::StartMotion()` with the selected motion
- **THEN** the motion SHALL begin playing with fade-in

#### Scenario: Random motion selection when name is empty
- **WHEN** `SetMotion` is called with a group name and an empty or null name
- **THEN** the system SHALL randomly select one motion from the group's motion list
- **THEN** the selected motion SHALL NOT be the same as the currently playing motion (when possible)

#### Scenario: Group not found
- **WHEN** `SetMotion` is called with a group name that does not exist in the index
- **THEN** the system SHALL fall back to the "Idle" group (case-insensitive)
- **THEN** the system SHALL log a warning about the missing group

### Requirement: Motion playback SHALL use fade transitions
The system SHALL use fade-in and fade-out transitions when switching between motions. The fade duration SHALL be taken from the motion's configured `FadeInTime` and `FadeOutTime` in `model3.json`.

#### Scenario: Fade transition on motion change
- **WHEN** a new motion starts playing while another motion is active
- **THEN** the previous motion SHALL fade out
- **THEN** the new motion SHALL fade in
- **THEN** the transition SHALL be smooth (no visual popping)

### Requirement: System SHALL expose motion group query API
The system SHALL provide a native API to query available motion groups and motion counts per group. This enables the C# side to adapt to different models' motion group structures.

#### Scenario: Query available groups
- **WHEN** `Bridge_GetMotionGroupCount` is called after model load
- **THEN** it SHALL return the number of motion groups in the loaded model

#### Scenario: Query group name
- **WHEN** `Bridge_GetMotionGroupName(index)` is called with a valid index
- **THEN** it SHALL return the name of the motion group at that index
