## Why

The current pet window (280×380) is too large for a desktop companion. It takes up significant screen real estate and dominates the workspace rather than complementing it. Halving the window size will make it a more subtle, unobtrusive presence on the desktop.

## What Changes

- Reduce `MainWindow` dimensions from 280×380 to 140×190
- Reduce the character display area (`PetCharacterControl` and `Live2DHostControl`) from 200×240 to 100×120 proportionally
- Reduce the outer `Grid` margin so the content still fits within the smaller window
- Scale or hide bottom text labels to fit the smaller space
- Adjust the radial menu size to fit the new layout

## Capabilities

### New Capabilities
- `window-resize`: Covers the MainWindow dimension change and cascading layout adjustments for all child controls (character display, labels, radial menu)

### Modified Capabilities
<!-- none -->

## Impact

- `Views/MainWindow.xaml` — Window Width/Height, Grid margins, child control sizes
- `Controls/PetCharacterControl.cs` — Default Width/Height and hardcoded internal positions may need scaling
- `Controls/RadialMenuControl.cs` — Menu size adjustment
- `Live2D/Rendering/Live2DHostControl.cs` — Display area dimensions
