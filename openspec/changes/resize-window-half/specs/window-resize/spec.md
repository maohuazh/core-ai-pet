## ADDED Requirements

### Requirement: Main window dimensions halved
The main window (`MainWindow`) SHALL have its `Width` set to 140 and `Height` set to 190, exactly half of the current 280×380.

#### Scenario: Window renders at half size
- **WHEN** the application launches
- **THEN** the MainWindow appears with dimensions 140×190

### Requirement: Character display area halved
Both `PetCharacterControl` and `Live2DHostControl` SHALL have `Width="100"` and `Height="120"`.

#### Scenario: Character area renders at half size
- **WHEN** the MainWindow renders
- **THEN** the character display area occupies 100×120 pixels

### Requirement: PetCharacterControl internal dimensions halved
All hardcoded character dimension fields in `PetCharacterControl` SHALL be halved: `_faceWidth` 80, `_faceHeight` 90, `_eyeSpacing` 15, `_eyeSize` 11, `_pupilSize` 5, `_mouthWidth` 12, `_mouthHeight` 4. Default `Width` 100, `Height` 120.

#### Scenario: Canvas character renders proportionally at new size
- **WHEN** PetCharacterControl renders without Live2D
- **THEN** the face, eyes, mouth, and blush elements fit within the 100×120 canvas

### Requirement: RadialMenuControl constants halved
`RadialMenuControl` SHALL have `MenuRadius` = 40 and `ItemSize` = 20. The control in MainWindow SHALL be set to `Width="100" Height="100"`.

#### Scenario: Radial menu fits within smaller window
- **WHEN** the radial menu is opened
- **THEN** menu items are positioned in a circle of radius 40 and each item is 20×20

### Requirement: Layout margins halved
The outer Grid margin and inner Grid margin in MainWindow SHALL be reduced from 20 to 10.

#### Scenario: Content fits within the smaller window
- **WHEN** MainWindow renders
- **THEN** the character, labels, and menu are properly contained without overflow

### Requirement: Drop shadow blur radius halved
The `DropShadowEffect` `BlurRadius` on the card Border SHALL be reduced from 24 to 12.

#### Scenario: Shadow renders without clipping
- **WHEN** MainWindow renders
- **THEN** the card shadow is fully visible within the 10px outer margin

### Requirement: Bottom text labels remain legible
Bottom text labels (`FontSize="18"` title and `FontSize="11"` subtitle) SHALL remain at their current sizes or be minimally adjusted, prioritizing readability over strict proportional scaling.

#### Scenario: Labels are readable at new window size
- **WHEN** MainWindow renders
- **THEN** "CoreAIpet" title and subtitle text are clearly readable
