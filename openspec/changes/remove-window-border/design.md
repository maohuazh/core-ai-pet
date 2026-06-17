## Context

The desktop assistant currently wraps the Live2D model in a decorative card with:
- 10px outer margin (transparent padding)
- Blue background panel (#FF2B3A6B) with rounded corners
- Drop shadow effect
- 10px inner margin
- Text labels below the model ("CoreAIpet" title + instruction text)

The window is already borderless (`WindowStyle="None"`, `AllowsTransparency="True"`), but the visual card design prevents the model from appearing as a floating character on the desktop.

## Goals / Non-Goals

**Goals:**
- Display only the Live2D model character with transparent surroundings
- Window size matches model display area (200×280 pixels)
- Preserve mouse drag functionality for window repositioning
- Maintain radial menu functionality

**Non-Goals:**
- Changing the Live2D rendering pipeline (already uses WriteableBitmap pixel readback)
- Changing model dimensions or projection
- Adding click-through transparency (still captures mouse events for drag/menu)
- Modifying the Canvas placeholder behavior (it will also become borderless)

## Decisions

### Decision 1: Remove Border element entirely
**Choice**: Delete the `<Border>` element with blue background, rounded corners, and drop shadow effect.
**Rationale**: The card frame is purely decorative and conflicts with the desktop pet use case where the character should appear to float on the desktop.
**Alternatives considered**: Making the border transparent → simpler to just remove the element.

### Decision 2: Remove all margins
**Choice**: Remove the outer `<Grid Margin="10">` and inner `<Grid Margin="10">` padding.
**Rationale**: With the border removed, margins create unwanted transparent space around the model. Removing them makes the window exactly match the model area.
**Alternatives considered**: Reducing margins to smaller values → no benefit, model should fill the window.

### Decision 3: Remove bottom text labels
**Choice**: Remove the StackPanel containing "CoreAIpet" title and instruction text.
**Rationale**: Text labels add visual clutter and increase window height beyond the model area. The application name is visible in the taskbar (if shown) and users don't need persistent instructions.
**Alternatives considered**: Keeping labels but making them transparent → adds complexity without user benefit.

### Decision 4: Set window size to model dimensions
**Choice**: Set `Width="200" Height="280"` on the Window element (matching PetCharacterControl and Live2DHostControl dimensions).
**Rationale**: With no margins or borders, the window should exactly match the model display area to avoid any extra transparent space.
**Alternatives considered**: Dynamic sizing based on model dimensions → over-engineering since model dimensions are fixed at 200×280.

### Decision 5: Keep mouse event handling on entire window
**Choice**: No change to mouse event handlers (OnMouseLeftButtonDown, OnMouseMove, etc. are on the Window).
**Rationale**: WPF routes mouse events through the visual tree even for transparent areas. The existing handlers on the Window element will continue to work for drag operations even though the background is transparent.
**Alternatives considered**: Adding drag handler to specific child elements → more complex, no benefit.

## Risks / Trade-offs

- **[No visual feedback for drag area]** → Users may not realize they can drag the window since there's no visible frame. Mitigation: the model itself is clickable, and user testing will confirm if this is intuitive enough.
- **[Clicking on transparent areas might not work]** → WPF transparent areas (Background="Transparent") DO receive mouse events. Mitigation: tested behavior, no action needed.
- **[Radial menu positioning]** → The radial menu is centered in the content grid. With no margins, it's centered on the model. Mitigation: this is actually correct behavior for a model-sized window.
- **[Canvas placeholder also borderless]** → The placeholder character will also display without a frame. Mitigation: consistent behavior, acceptable.
