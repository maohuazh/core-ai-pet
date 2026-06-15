## ADDED Requirements

### Requirement: Plugin discovery from plugins directory
The system SHALL scan the `plugins/` directory at startup, discover plugin subdirectories containing `plugin.json`, and load them.

#### Scenario: Discover plugins on startup
- **WHEN** the application starts
- **THEN** all subdirectories under `plugins/` containing a valid `plugin.json` are discovered

#### Scenario: Ignore invalid plugin directories
- **WHEN** a subdirectory under `plugins/` has no `plugin.json` or it is malformed
- **THEN** that directory is skipped and an error is logged

### Requirement: Plugin isolation via AssemblyLoadContext
Each external plugin DLL SHALL be loaded in a separate `AssemblyLoadContext` with `isCollectible: true` for dependency isolation and unload support.

#### Scenario: Plugin loaded in isolated context
- **WHEN** a plugin DLL is loaded
- **THEN** it runs in its own AssemblyLoadContext, isolated from other plugins and the host

### Requirement: Plugin lifecycle management
The system SHALL manage the full plugin lifecycle: Load → Activate → Execute → Deactivate → Unload.

#### Scenario: Plugin load phase
- **WHEN** a plugin is discovered
- **THEN** `LoadAsync(IPluginContext)` is called, providing config, logger, event bus, and AI services

#### Scenario: Plugin activate phase
- **WHEN** all plugins are loaded
- **THEN** `ActivateAsync()` is called for each plugin to start connections and background tasks

#### Scenario: Plugin unload on shutdown
- **WHEN** the application is shutting down
- **THEN** `DeactivateAsync()` followed by `UnloadAsync()` is called for each plugin

### Requirement: Dynamic menu registration
Plugins SHALL register menu items via `IPluginHost.RegisterMenuItem()`. The radial menu SHALL dynamically render all registered items.

#### Scenario: Plugin registers menu item
- **WHEN** a plugin calls `RegisterMenuItem()` during its Load phase
- **THEN** the item appears in the radial menu

#### Scenario: Menu updates when plugin is unloaded
- **WHEN** a plugin is unloaded
- **THEN** its menu items are removed from the radial menu

### Requirement: Plugin error handling
If a plugin fails to load or throws an unhandled exception, the system SHALL log the error and continue running without that plugin.

#### Scenario: Plugin load failure
- **WHEN** a plugin throws an exception during LoadAsync
- **THEN** the error is logged, the plugin is marked as Error state, and other plugins continue normally
