## Context

The CoreAIpet desktop pet runs in a borderless, transparent WPF window (`MainWindow`) currently sized at 280×380. Inside, the character display area is 200×240, with surrounding Grid margins of 20px for the transparent/shadow region. The `PetCharacterControl` hardcodes internal face/eye/mouth dimensions based on the 200×240 canvas. The `RadialMenuControl` uses `MenuRadius=80`, `ItemSize=40` in a 200×200 area.

## Goals / Non-Goals

**Goals:**
- Halve the main window dimensions: 280×380 → 140×190
- Halve all child control dimensions proportionally so the layout remains visually consistent
- Preserve all existing interactions (eye tracking, radial menu, Live2D display)

**Non-Goals:**
- Adding user-configurable window size settings (out of scope — this is a one-time change)
- Changing the visual design, colors, or fonts
- Modifying DebugWindow, SettingsWindow, or ChatBubbleWindow sizes

## Decisions

### 1. Scale all hardcoded dimensions by 0.5× uniformly
**Decision**: Every pixel value inside MainWindow and its child controls will be halved.
**Rationale**: Proportional scaling preserves the visual design without introducing arbitrary new values. This is simpler than recalculating each element independently.

**Alternatives considered**:
- Scale only the window and let child controls auto-fit → Rejected: PetCharacterControl and RadialMenuControl use hardcoded pixel values, so they would overflow or look wrong.
- Use a `ScaleTransform` on the root → Rejected: WPF Canvas-based controls with absolute positions don't scale well with transforms; text would blur.

### 2. Keep the outer margin-to-content ratio consistent
**Decision**: Outer Grid margin (20→10), inner Grid margin (20→10). Content area shrinks from ~240×340 to ~120×170.
**Rationale**: The margin provides space for the drop shadow. Halving it proportionally keeps the shadow visible without wasting space.

### 3. Halve PetCharacterControl internal dimensions
**Decision**: All `_faceWidth`, `_faceHeight`, `_eyeSpacing`, `_eyeSize`, `_pupilSize`, `_mouthWidth`, `_mouthHeight`, and default `Width`/`Height` values halved.
**Rationale**: These values were tuned for 200×240. At 100×120 they must scale proportionally.

### 4. Halve RadialMenuControl constants
**Decision**: `MenuRadius` 80→40, `ItemSize` 40→20. MainWindow sets it to 100×100.
**Rationale**: The menu must fit within the smaller character area.

## Risks / Trade-offs

- **Text readability**: The bottom labels ("CoreAIpet", "鼠标移动 → 眼球跟随") may be too small after halving font sizes proportionally. → Mitigation: Keep font sizes at current values (18 and 11) or reduce minimally; accept slight visual imbalance rather than unreadable text.
- **Drop shadow clipping**: Halving the margin from 20 to 10 may clip the 24-blur shadow. → Mitigation: Reduce shadow `BlurRadius` from 24 to 12 to match the new margin.
- **Hit-test accuracy**: Smaller clickable area for the window. → Mitigation: Acceptable trade-off; the window remains draggable and the menu still functional.
