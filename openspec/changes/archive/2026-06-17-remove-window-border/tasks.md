## 1. Remove visual frame elements

- [x] 1.1 Remove the outer `<Grid Margin="10">` wrapper — flatten to just the content Grid
- [x] 1.2 Remove the `<Border>` element (blue background, rounded corners, drop shadow)
- [x] 1.3 Remove the inner `<Grid Margin="10">` margin
- [x] 1.4 Remove the bottom StackPanel with "CoreAIpet" title and instruction text

## 2. Adjust window dimensions

- [x] 2.1 Set Window Width="200" Height="280" to match model display area
- [x] 2.2 Ensure Live2DDisplay and CharacterDisplay fill the window completely

## 3. Verify functionality

- [x] 3.1 Build and run — verify model is visible without any frame/background
- [x] 3.2 Verify window can be dragged by clicking on model area
- [x] 3.2 Verify radial menu still appears on mouse enter and functions correctly
