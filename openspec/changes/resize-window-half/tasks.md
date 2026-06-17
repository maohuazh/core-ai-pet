## 1. MainWindow Layout

- [x] 1.1 Update `MainWindow.xaml` Window `Width` from 280 to 140 and `Height` from 380 to 190
- [x] 1.2 Update outer Grid `Margin` from 20 to 10 and inner Grid `Margin` from 20 to 10
- [x] 1.3 Update `DropShadowEffect` `BlurRadius` from 24 to 12 on the card Border
- [x] 1.4 Update `PetCharacterControl` Width/Height in MainWindow from 200/240 to 100/120
- [x] 1.5 Update `Live2DHostControl` Width/Height in MainWindow from 200/240 to 100/120
- [x] 1.6 Update `RadialMenuControl` Width/Height in MainWindow from 200/200 to 100/100

## 2. PetCharacterControl Internal Scaling

- [x] 2.1 Halve default `Width` (200→100) and `Height` (240→120) in constructor
- [x] 2.2 Halve `_faceWidth` (160→80) and `_faceHeight` (180→90)
- [x] 2.3 Halve `_eyeSpacing` (30→15), `_eyeSize` (22→11), `_pupilSize` (10→5)
- [x] 2.4 Halve `_mouthWidth` (24→12) and `_mouthHeight` (8→4)
- [x] 2.5 Halve `MaxEyeOffset` (6→3) and any other hardcoded offset constants

## 3. RadialMenuControl Scaling

- [x] 3.1 Halve `MenuRadius` (80→40) and `ItemSize` (40→20)

## 4. Verification

- [x] 4.1 Build the project and verify no compile errors
- [ ] 4.2 Run the app and verify the window is visibly half the original size
- [ ] 4.3 Verify Canvas character renders correctly (face, eyes, mouth fit within bounds)
- [ ] 4.4 Verify eye tracking still follows the cursor
- [ ] 4.5 Verify drop shadow renders without clipping
