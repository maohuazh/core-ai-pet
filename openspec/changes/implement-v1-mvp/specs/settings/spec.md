## ADDED Requirements

### Requirement: Appearance settings
The system SHALL allow configuring character scale (50%-200%), opacity (20%-100%), and theme (Light/Dark).

#### Scenario: Change character scale
- **WHEN** the user sets scale to 150% in settings
- **THEN** the character window resizes to 150% of its base size

#### Scenario: Change opacity
- **WHEN** the user sets opacity to 50%
- **THEN** the character window becomes semi-transparent

#### Scenario: Switch theme
- **WHEN** the user switches to Light theme
- **THEN** all UI elements (chat bubble, menus, settings) use light colors

### Requirement: System settings
The system SHALL support: auto-start with Windows, always-on-top toggle, and click-through toggle.

#### Scenario: Enable auto-start
- **WHEN** the user enables "auto-start" in settings
- **THEN** the application launches automatically on next Windows login

#### Scenario: Toggle always-on-top
- **WHEN** the user disables "always-on-top"
- **THEN** other windows can overlap the character window

### Requirement: AI backend settings
The system SHALL allow configuring AI backends (OpenAI, Azure OpenAI, Ollama) with Endpoint, API Key, and Model fields. The user SHALL be able to select the active provider.

#### Scenario: Configure OpenAI
- **WHEN** the user enters an OpenAI API key and model name
- **THEN** the configuration is saved and the connection can be tested

#### Scenario: Select active provider
- **WHEN** the user switches the active provider dropdown to "Ollama"
- **THEN** subsequent AI requests use the Ollama backend

### Requirement: Settings persistence
All settings SHALL be persisted to config.json and restored on restart.

#### Scenario: Settings survive restart
- **WHEN** the user changes settings and restarts the application
- **THEN** all settings are restored to their saved values
