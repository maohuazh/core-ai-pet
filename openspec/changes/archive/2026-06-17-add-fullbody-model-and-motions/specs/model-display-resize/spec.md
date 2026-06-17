## MODIFIED Requirements

### Requirement: Projection SHALL fit model in square viewport
The system SHALL adjust the Live2D projection matrix so the full-body model is completely visible within the 150×150 square viewport without clipping. The model height parameter SHALL be tuned to ensure the character's head and feet are both visible.

#### Scenario: Full body visible in viewport
- **WHEN** the Live2D model is rendered in the 150×150 viewport
- **THEN** the model's full body (head to feet) SHALL be visible
- **THEN** no part of the model SHALL be clipped at the viewport edges
- **THEN** the model SHALL be horizontally centered
