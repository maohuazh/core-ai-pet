## ADDED Requirements

### Requirement: Performance metrics display
The debug panel SHALL display real-time FPS, CPU usage, and memory consumption.

#### Scenario: FPS display
- **WHEN** the debug panel is open
- **THEN** the current rendering FPS is displayed and updated in real-time

#### Scenario: Memory display
- **WHEN** the debug panel is open
- **THEN** the application's working set memory (MB) is displayed

### Requirement: Plugin status display
The debug panel SHALL show the status of all loaded plugins (Id, Name, State, Connection status).

#### Scenario: List all plugins
- **WHEN** the debug panel is open
- **THEN** each loaded plugin is listed with its Id, Name, current State, and connection status

### Requirement: Log viewer
The debug panel SHALL display recent application logs with timestamp, level, and message.

#### Scenario: View recent logs
- **WHEN** the debug panel is open
- **THEN** the most recent 100 log entries are displayed

#### Scenario: New logs appear in real-time
- **WHEN** a new log entry is written while the panel is open
- **THEN** the new entry appears at the bottom of the log list
