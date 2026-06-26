## ADDED Requirements

### Requirement: Extract Live2D motions
The system SHALL provide a Tauri command `get_available_motions(model_id: String)` that extracts all available motions from a Live2D model's `.model3.json` file by parsing the `FileReferences.Motions` section.

#### Scenario: Extract motions from Live2D model
- **WHEN** `get_available_motions` is called with a Live2D model_id
- **THEN** the system SHALL read the `.model3.json` file, parse all motion groups and their motions, and return a list of `MotionInfo` objects with `group`, `name`, and `display_name` fields

#### Scenario: Extract motions from model with no motions
- **WHEN** `get_available_motions` is called with a Live2D model_id that has no motions defined
- **THEN** the system SHALL return an empty array

### Requirement: Extract Live2D expressions
The system SHALL provide a Tauri command `get_available_expressions(model_id: String)` that extracts all available expressions from a Live2D model's `.model3.json` file by parsing the `FileReferences.Expressions` section.

#### Scenario: Extract expressions from Live2D model
- **WHEN** `get_available_expressions` is called with a Live2D model_id
- **THEN** the system SHALL read the `.model3.json` file, parse all expressions, and return a list of `ExpressionInfo` objects with `name`, `display_name`, and optional `file` fields

#### Scenario: Extract expressions from model with no expressions
- **WHEN** `get_available_expressions` is called with a Live2D model_id that has no expressions defined
- **THEN** the system SHALL return an empty array

### Requirement: Extract SpriteSheet motions
The system SHALL extract all available motions from a SpriteSheet model's `manifest.json` file by parsing the `motions` section.

#### Scenario: Extract motions from SpriteSheet model
- **WHEN** `get_available_motions` is called with a SpriteSheet model_id
- **THEN** the system SHALL read the `manifest.json` file, parse all motion keys and their metadata, and return a list of `MotionInfo` objects with `group`, `name`, and `display_name` fields

#### Scenario: Extract motions from SpriteSheet model with no motions
- **WHEN** `get_available_motions` is called with a SpriteSheet model_id that has no motions defined
- **THEN** the system SHALL return an empty array

### Requirement: Extract SpriteSheet expressions
The system SHALL extract all available expressions from a SpriteSheet model's `manifest.json` file by parsing the `expressions` section.

#### Scenario: Extract expressions from SpriteSheet model
- **WHEN** `get_available_expressions` is called with a SpriteSheet model_id
- **THEN** the system SHALL read the `manifest.json` file, parse all expression keys and their metadata, and return a list of `ExpressionInfo` objects with `name`, `display_name`, and optional `file` fields

#### Scenario: Extract expressions from SpriteSheet model with no expressions
- **WHEN** `get_available_expressions` is called with a SpriteSheet model_id that has no expressions defined
- **THEN** the system SHALL return an empty array

### Requirement: Handle unknown model type
The system SHALL reject resource extraction requests for unknown model types.

#### Scenario: Extract motions from unknown model type
- **WHEN** `get_available_motions` is called with a model_id of an unknown type
- **THEN** the system SHALL return an error message "未知模型类型"

#### Scenario: Extract expressions from unknown model type
- **WHEN** `get_available_expressions` is called with a model_id of an unknown type
- **THEN** the system SHALL return an error message "未知模型类型"
