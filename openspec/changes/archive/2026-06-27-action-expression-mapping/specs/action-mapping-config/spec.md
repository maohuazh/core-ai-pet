## ADDED Requirements

### Requirement: Action mapping database table
The system SHALL create a `model_action_mappings` table in SQLite with the following schema:
- `id` (TEXT PRIMARY KEY)
- `model_id` (TEXT NOT NULL, foreign key to models.id with ON DELETE CASCADE)
- `trigger_key` (TEXT NOT NULL, constrained to 10 predefined values)
- `motion_group` (TEXT, nullable)
- `motion_name` (TEXT, nullable)
- `expression_name` (TEXT, nullable)
- `effect_name` (TEXT, nullable)
- `effect_duration` (INTEGER, nullable, milliseconds)
- `effect_position` (TEXT, default 'center', constrained to 'center'/'above'/'below')
- `use_default` (INTEGER NOT NULL, default 0)
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)
- UNIQUE constraint on (model_id, trigger_key)

#### Scenario: Table creation on first run
- **WHEN** the application starts for the first time
- **THEN** the `model_action_mappings` table SHALL be created with the specified schema

### Requirement: Get action mappings for a model
The system SHALL provide a Tauri command `get_action_mappings(model_id: String)` that returns all action mapping records for the specified model, ordered by trigger_key.

#### Scenario: Retrieve existing mappings
- **WHEN** `get_action_mappings` is called with a valid model_id that has mappings
- **THEN** the system SHALL return all mapping records for that model

#### Scenario: Retrieve mappings for model with no mappings
- **WHEN** `get_action_mappings` is called with a model_id that has no mappings
- **THEN** the system SHALL return an empty array

### Requirement: Save action mapping
The system SHALL provide a Tauri command `save_action_mapping` that creates or updates a single action mapping record. If a record with the same (model_id, trigger_key) already exists, it SHALL update the existing record; otherwise, it SHALL insert a new record.

#### Scenario: Create new mapping
- **WHEN** `save_action_mapping` is called with a (model_id, trigger_key) combination that does not exist
- **THEN** the system SHALL insert a new record and return success

#### Scenario: Update existing mapping
- **WHEN** `save_action_mapping` is called with a (model_id, trigger_key) combination that already exists
- **THEN** the system SHALL update the existing record and return success

### Requirement: Delete action mapping
The system SHALL provide a Tauri command `delete_action_mapping(id: String)` that deletes a single action mapping record by its id.

#### Scenario: Delete existing mapping
- **WHEN** `delete_action_mapping` is called with a valid id
- **THEN** the system SHALL delete the record and return success

#### Scenario: Delete non-existent mapping
- **WHEN** `delete_action_mapping` is called with an id that does not exist
- **THEN** the system SHALL return success (idempotent operation)

### Requirement: Validate daily_1 mandatory constraint
The system SHALL validate that the `daily_1` trigger has either `use_default = 1` or at least one of (motion_name, expression_name) configured.

#### Scenario: Save daily_1 with use_default
- **WHEN** saving a daily_1 mapping with `use_default = 1` and no motion/expression
- **THEN** the system SHALL accept the mapping

#### Scenario: Save daily_1 with motion configured
- **WHEN** saving a daily_1 mapping with `use_default = 0` and `motion_name` set
- **THEN** the system SHALL accept the mapping

#### Scenario: Save daily_1 with empty configuration
- **WHEN** saving a daily_1 mapping with `use_default = 0` and no motion/expression
- **THEN** the system SHALL reject the mapping with an error message

### Requirement: Frontend action mapping service
The system SHALL provide an `actionMappingService` TypeScript class that wraps all IPC calls for action mapping CRUD operations and provides helper methods for data conversion.

#### Scenario: Load mappings
- **WHEN** `loadMappings(modelId)` is called
- **THEN** the service SHALL invoke `get_action_mappings` and return the results

#### Scenario: Save mappings
- **WHEN** `saveMappings(modelId, formData[])` is called
- **THEN** the service SHALL validate daily_1 constraint, convert form data to parameters, and invoke `save_action_mapping` for each record
