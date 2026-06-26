## ADDED Requirements

### Requirement: Action mapping entry button
The system SHALL display a "⚙ 动作映射" button on each model card in the settings model config list. Clicking the button SHALL open the action mapping configuration panel for that model.

#### Scenario: Display action mapping button
- **WHEN** the settings model config panel is displayed
- **THEN** each model card SHALL show a "⚙ 动作映射" button

#### Scenario: Open action mapping panel
- **WHEN** the user clicks the "⚙ 动作映射" button on a model card
- **THEN** the system SHALL open the action mapping configuration panel for that model, replacing the model list view

### Requirement: Action mapping panel navigation
The action mapping configuration panel SHALL provide a "← 返回模型列表" button that returns to the model list view. If there are unsaved changes, the system SHALL display a confirmation dialog before navigating away.

#### Scenario: Return without unsaved changes
- **WHEN** the user clicks "← 返回模型列表" and there are no unsaved changes
- **THEN** the system SHALL return to the model list view immediately

#### Scenario: Return with unsaved changes
- **WHEN** the user clicks "← 返回模型列表" and there are unsaved changes
- **THEN** the system SHALL display a confirmation dialog asking "有未保存的修改，确定要返回吗？"

#### Scenario: Confirm return with unsaved changes
- **WHEN** the user confirms the dialog
- **THEN** the system SHALL discard unsaved changes and return to the model list view

#### Scenario: Cancel return with unsaved changes
- **WHEN** the user cancels the dialog
- **THEN** the system SHALL remain on the action mapping panel with changes preserved
