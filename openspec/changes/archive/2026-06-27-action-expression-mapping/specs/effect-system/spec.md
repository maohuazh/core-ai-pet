## ADDED Requirements

### Requirement: Predefined effects list
The system SHALL provide a static list of 12 predefined effects with the following properties for each: `id`, `name` (display name), `icon` (emoji or image path), `description`, and `defaultDuration` (milliseconds).

The predefined effects SHALL be:
- sparkle (闪光, ✨, 1500ms)
- heart (爱心, ❤️, 2000ms)
- sweat_drop (汗滴, 💧, 2500ms)
- exclamation (感叹号, ❗, 1500ms)
- question (问号, ❓, 2000ms)
- music_note (音符, 🎵, 2000ms)
- zzz (睡眠, 💤, 3000ms)
- anger (怒气, 💢, 1500ms)
- blush (脸红, 😊, 2000ms)
- star (星星, ⭐, 1500ms)
- check_mark (对勾, ✅, 1500ms)
- warning (警告, ⚠️, 2500ms)

#### Scenario: Retrieve all available effects
- **WHEN** the frontend requests the list of available effects
- **THEN** the system SHALL return all 12 predefined effects with their complete metadata

### Requirement: Effect position constraint
The system SHALL constrain effect positions to three values: 'center', 'above', and 'below'.

#### Scenario: Save effect with valid position
- **WHEN** saving an action mapping with `effect_position` set to 'center', 'above', or 'below'
- **THEN** the system SHALL accept the value

#### Scenario: Save effect with invalid position
- **WHEN** saving an action mapping with `effect_position` set to any other value
- **THEN** the system SHALL reject the value with a database constraint error

### Requirement: Effect duration default
The system SHALL use a default duration of 2000ms when an effect is configured without an explicit duration.

#### Scenario: Save effect without duration
- **WHEN** saving an action mapping with `effect_name` set but `effect_duration` as NULL
- **THEN** the system SHALL use 2000ms as the default duration at the frontend layer

### Requirement: Effect playback interface
The system SHALL provide an `effectManager` interface with a `play(effectId: string, options: { duration?: number, position?: string })` method that triggers the visual effect on the pet window.

#### Scenario: Play effect with default options
- **WHEN** `effectManager.play('sparkle')` is called
- **THEN** the system SHALL play the sparkle effect with duration 1500ms and position 'center'

#### Scenario: Play effect with custom options
- **WHEN** `effectManager.play('heart', { duration: 3000, position: 'above' })` is called
- **THEN** the system SHALL play the heart effect with duration 3000ms and position 'above'

### Requirement: Effect system as Phase 2 placeholder
The system SHALL implement the effect data model and UI selection, but effect playback SHALL be a no-op in Phase 1 (no visual effect rendered).

#### Scenario: Configure effect in mapping
- **WHEN** a user selects an effect in the action mapping configuration panel
- **THEN** the system SHALL save the effect configuration to the database

#### Scenario: Trigger effect at runtime
- **WHEN** an action mapping with an effect is triggered at runtime
- **THEN** the system SHALL log the effect playback attempt but not render any visual effect (Phase 1 placeholder)
