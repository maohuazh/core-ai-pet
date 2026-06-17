# Borderless Model Display

## Purpose

This capability provides a frameless, transparent display for the Live2D model character. The model appears to float directly on the desktop without any visible container, border, or background.

## Requirements

### Requirement: Live2D model SHALL display without visual frame
The system SHALL render the Live2D model directly on the desktop without any visible border, background container, or decorative frame around the model. Only the model character pixels SHALL be visible; all surrounding areas SHALL be fully transparent.

#### Scenario: Model displays without card frame
- **WHEN** the Live2D model is loaded and rendering
- **THEN** the model character SHALL be visible with transparent background
- **THEN** no blue background panel SHALL be visible behind the model
- **THEN** no rounded corners or drop shadow SHALL surround the model

#### Scenario: Transparent areas show desktop
- **WHEN** the desktop assistant is running with a Live2D model loaded
- **THEN** areas of the window not occupied by the model pixels SHALL be transparent
- **THEN** the desktop background SHALL be visible through transparent areas

### Requirement: Window dimensions SHALL match model display area
The system SHALL size the main window to exactly match the Live2D model display dimensions (150×150 pixels by default). No additional padding or margins SHALL exist around the model area.

#### Scenario: Window size matches model size
- **WHEN** the application starts with default model dimensions
- **THEN** the window width SHALL be 150 pixels
- **THEN** the window height SHALL be 150 pixels
- **THEN** the Live2D model SHALL fill the entire window area

#### Scenario: No padding around model
- **WHEN** the Live2D model is rendering
- **THEN** there SHALL be no margin between the window edge and the model display area
- **THEN** the model SHALL occupy the full window dimensions

### Requirement: Mouse drag SHALL work on transparent window
The system SHALL allow users to drag the transparent, frameless window by clicking and dragging on the model area. Mouse events SHALL propagate through transparent areas to enable window repositioning.

#### Scenario: Drag window from model area
- **WHEN** user presses and holds the left mouse button on the Live2D model
- **THEN** the window SHALL begin dragging with the mouse movement
- **THEN** releasing the mouse button SHALL stop dragging

### Requirement: Bottom text labels SHALL be removed
The system SHALL NOT display the "CoreAIpet" title text or instruction text below the model. The display area SHALL contain only the Live2D model and the radial menu (when visible).

#### Scenario: No text labels in display
- **WHEN** the desktop assistant is running
- **THEN** no "CoreAIpet" text SHALL be visible
- **THEN** no instruction text SHALL be visible
- **THEN** only the Live2D model and menu controls SHALL appear
