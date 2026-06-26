## ADDED Requirements

### Requirement: External event listener
The system SHALL provide a `triggerHandler` module that listens for `external-event` events on the eventBus and maps them to trigger keys.

#### Scenario: Receive external event
- **WHEN** an `external-event` is emitted on the eventBus with type 'chat-message-received'
- **THEN** the triggerHandler SHALL map it to trigger_key 'new_message'

#### Scenario: Receive unknown event type
- **WHEN** an `external-event` is emitted with an unrecognized type
- **THEN** the triggerHandler SHALL ignore the event and not trigger any action

### Requirement: Trigger key to action mapping lookup
The system SHALL query the database for the action mapping configuration corresponding to the current active model and the trigger key.

#### Scenario: Find existing mapping
- **WHEN** a trigger event occurs with trigger_key 'new_message' and the active model has a mapping configured
- **THEN** the system SHALL retrieve the mapping record from the database

#### Scenario: No mapping configured
- **WHEN** a trigger event occurs and the active model has no mapping for that trigger_key
- **THEN** the system SHALL not trigger any action

### Requirement: Execute mapped motion
The system SHALL call the Renderer's `playMotion(motionGroup, motionName)` method when a mapping has `motion_name` configured.

#### Scenario: Execute motion from mapping
- **WHEN** a triggered mapping has `motion_group = 'TapBody'` and `motion_name = 'tap_body_01'`
- **THEN** the system SHALL call `renderer.playMotion('TapBody', 'tap_body_01')`

#### Scenario: Skip motion when not configured
- **WHEN** a triggered mapping has `motion_name = NULL`
- **THEN** the system SHALL not call `playMotion`

### Requirement: Execute mapped expression
The system SHALL call the Renderer's `playExpression(expressionName)` method when a mapping has `expression_name` configured.

#### Scenario: Execute expression from mapping
- **WHEN** a triggered mapping has `expression_name = 'happy'`
- **THEN** the system SHALL call `renderer.playExpression('happy')`

#### Scenario: Skip expression when not configured
- **WHEN** a triggered mapping has `expression_name = NULL`
- **THEN** the system SHALL not call `playExpression`

### Requirement: Execute mapped effect
The system SHALL call the `effectManager.play(effectName, options)` method when a mapping has `effect_name` configured.

#### Scenario: Execute effect from mapping
- **WHEN** a triggered mapping has `effect_name = 'sparkle'`, `effect_duration = 1500`, and `effect_position = 'center'`
- **THEN** the system SHALL call `effectManager.play('sparkle', { duration: 1500, position: 'center' })`

#### Scenario: Skip effect when not configured
- **WHEN** a triggered mapping has `effect_name = NULL`
- **THEN** the system SHALL not call `effectManager.play`

### Requirement: Handle use_default flag
The system SHALL execute the model's default idle motion when a mapping has `use_default = 1`.

#### Scenario: Trigger mapping with use_default
- **WHEN** a triggered mapping has `use_default = 1`
- **THEN** the system SHALL call `renderer.playMotion('Idle', 0)` (or the model's default idle motion)

#### Scenario: use_default overrides explicit configuration
- **WHEN** a triggered mapping has `use_default = 1` and also has motion/expression configured
- **THEN** the system SHALL execute the default motion and ignore the explicit configuration

### Requirement: Daily tick timer
The system SHALL provide a `daily-tick` event on the eventBus that randomly triggers `daily_1`, `daily_2`, or `daily_3` with weighted probabilities (70%, 20%, 10%).

#### Scenario: Daily tick triggers daily_1
- **WHEN** the daily-tick timer fires and random value < 0.7
- **THEN** the system SHALL trigger the `daily_1` action mapping

#### Scenario: Daily tick triggers daily_2
- **WHEN** the daily-tick timer fires and 0.7 <= random value < 0.9
- **THEN** the system SHALL trigger the `daily_2` action mapping

#### Scenario: Daily tick triggers daily_3
- **WHEN** the daily-tick timer fires and random value >= 0.9
- **THEN** the system SHALL trigger the `daily_3` action mapping

#### Scenario: Daily tick interval
- **WHEN** the daily-tick timer is active
- **THEN** the timer SHALL fire every 5 minutes by default

### Requirement: Event type to trigger key mapping
The system SHALL map the following external event types to trigger keys:
- 'chat-message-received' → 'new_message'
- 'jira-task-assigned' → 'new_task'
- 'email-received' → 'new_email'
- 'jira-task-status-changed' → 'task_in_progress'
- 'jira-task-completed' → 'task_completed'
- 'jira-task-deadline-approaching' → 'task_approaching_deadline'
- 'jira-task-overdue' → 'task_overdue'

#### Scenario: Map chat message event
- **WHEN** a 'chat-message-received' event is received
- **THEN** the system SHALL map it to trigger_key 'new_message'

#### Scenario: Map Jira task completed event
- **WHEN** a 'jira-task-completed' event is received
- **THEN** the system SHALL map it to trigger_key 'task_completed'
