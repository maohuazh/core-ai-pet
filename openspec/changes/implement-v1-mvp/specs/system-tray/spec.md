## ADDED Requirements

### Requirement: System tray icon
The application SHALL reside in the system tray with a visible icon.

#### Scenario: Tray icon visible
- **WHEN** the application is running
- **THEN** an icon is visible in the system tray area

### Requirement: Tray right-click context menu
Right-clicking the tray icon SHALL show a context menu with: Show Assistant, Hide Assistant, Settings, Restart, Exit.

#### Scenario: Show context menu
- **WHEN** the user right-clicks the tray icon
- **THEN** a context menu appears with five options

#### Scenario: Hide assistant
- **WHEN** the user clicks "Hide Assistant"
- **THEN** the character window is hidden but the tray icon remains

#### Scenario: Show assistant
- **WHEN** the user clicks "Show Assistant"
- **THEN** the character window is shown again

#### Scenario: Exit application
- **WHEN** the user clicks "Exit"
- **THEN** the application saves state and terminates gracefully

### Requirement: Double-click tray icon to restore
Double-clicking the tray icon SHALL restore the character window.

#### Scenario: Restore from tray
- **WHEN** the user double-clicks the tray icon while the window is hidden
- **THEN** the character window is shown at its last position
