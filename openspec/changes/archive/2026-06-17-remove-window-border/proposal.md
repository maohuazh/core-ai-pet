## Why

The desktop assistant currently displays the Live2D model inside a decorative card with blue background, rounded corners, and drop shadow. For a desktop pet application, users expect the character to appear as if floating directly on the desktop without any visible frame or container. The current card design creates visual clutter and prevents the model from blending naturally with the desktop environment.

## What Changes

- **Remove card border and background**: Eliminate the Border element with blue background (#FF2B3A6B), rounded corners (CornerRadius="16"), and drop shadow effect
- **Remove decorative margins**: Remove the 10px outer margin and 10px inner margin that create padding around the model
- **Remove bottom text labels**: Remove the "CoreAIpet" title and "鼠标移动 → 眼球跟随" instruction text
- **Resize window to match model**: Set window dimensions to exactly match the Live2D model display area (200×280 pixels)
- **Maintain transparency**: Keep AllowsTransparency="True" and Background="Transparent" so only the model pixels are visible

## Capabilities

### New Capabilities
- `borderless-model-display`: Display Live2D model directly on desktop without any frame, border, or background container

### Modified Capabilities
- `live2d-model-loading`: Window sizing and layout requirements change to match model dimensions exactly

## Impact

- **Files**: `MainWindow.xaml` layout structure, potentially `MainWindow.xaml.cs` if window sizing logic needs adjustment
- **Visual**: Application will appear as just the character on desktop (no card/frame)
- **Drag behavior**: Mouse drag will need to work on transparent areas or the model itself (currently works on the card background)
- **Radial menu**: Menu positioning may need adjustment since it was centered in the card layout
