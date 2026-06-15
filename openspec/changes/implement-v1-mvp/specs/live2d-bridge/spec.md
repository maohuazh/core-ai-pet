## ADDED Requirements

### Requirement: C DLL export interface
The Live2DBridge DLL SHALL export C-compatible functions for: Initialize, Shutdown, LoadModel, UnloadModel, InitializeRenderer, Render, Resize, SetMotionGroup, SetParameter, SetEyeTarget, GetFPS.

#### Scenario: All functions exported
- **WHEN** the DLL is compiled
- **THEN** all 11 functions are visible in the DLL export table

#### Scenario: C# P/Invoke can call functions
- **WHEN** C# code declares corresponding DllImport attributes
- **THEN** the functions can be called without marshalling errors

### Requirement: Model loading from .moc3 files
The bridge SHALL load Live2D models from .moc3 file paths, initializing the model for rendering and animation.

#### Scenario: Load valid model
- **WHEN** Bridge_LoadModel is called with a valid .moc3 file path
- **THEN** the model is loaded and ready for rendering

#### Scenario: Load invalid model path
- **WHEN** Bridge_LoadModel is called with an invalid path
- **THEN** the function returns false and no crash occurs

### Requirement: D3D11 rendering to host HWND
The bridge SHALL initialize a Direct3D 11 renderer targeting a HWND provided by the WPF host, rendering each frame when Bridge_Render is called.

#### Scenario: Render single frame
- **WHEN** Bridge_InitializeRenderer is called with a valid HWND and Bridge_Render is called
- **THEN** one frame of the Live2D model is rendered to the HWND

#### Scenario: Resize rendering surface
- **WHEN** Bridge_Resize is called with new dimensions
- **THEN** the D3D11 swap chain is resized accordingly

### Requirement: Animation and state control
The bridge SHALL support playing animations by group/name and setting model parameters by ID.

#### Scenario: Play animation
- **WHEN** Bridge_SetMotionGroup("idle", "idle_01") is called
- **THEN** the specified idle animation begins playing

#### Scenario: Set model parameter
- **WHEN** Bridge_SetParameter("ParamAngleX", 0.5) is called
- **THEN** the model's head turns partially to the right

### Requirement: Eye tracking parameter calculation
The bridge SHALL convert normalized eye target coordinates (-1.0 to 1.0) into Live2D eye tracking parameters (ParamEyeBallX, ParamEyeBallY).

#### Scenario: Set eye target
- **WHEN** Bridge_SetEyeTarget(0.5, -0.3) is called
- **THEN** the character's eyes look right and slightly down
