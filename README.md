# Core AI Pet

A Windows desktop AI pet assistant.

## Live2D Model Configuration

### Default Model Path

The application loads Live2D models from an external directory. By default, it uses:

```
C:\Change Top\git\open-source\live2d\model\22
```

You can change this path in the settings UI or by editing `config.json`:

```json
{
  "live2d": {
    "modelPath": "C:\\path\\to\\your\\model\\directory",
    "modelVariant": "model.default.json"
  }
}
```

### Model Variants

The model directory contains multiple JSON files representing different character variants/outfits:

- `model.default.json` - Default appearance
- `model.2016.xmas.1.json` - Christmas 2016 variant
- `model.2017.summer.super.1.json` - Summer 2017 variant
- And many more...

You can switch between variants at runtime via the Settings window (Live2D tab).

### Live2D SDK Setup (Optional)

The application uses a dual-mode architecture:

1. **Mock Mode** (default): Uses WPF Canvas placeholder character - no SDK required
2. **SDK Mode**: Uses Live2D Cubism Native SDK for real model rendering

To enable SDK mode:

1. Download [Live2D Cubism Native SDK for Windows](https://www.live2d.com/en/sdk/about-cubism-sdk/)
2. Extract to `vendor/Live2DCubismSdk/`
3. Edit `src/CoreAIpet.Live2DBridge/CMakeLists.txt`:
   - Uncomment `LIVE2D_HAS_SDK` definition
   - Uncomment SDK library paths
4. Rebuild the Live2DBridge DLL

```powershell
cd src\CoreAIpet.Live2DBridge
mkdir build; cd build
cmake .. -G "Visual Studio 17 2022" -A x64
cmake --build . --config Release
```

### Troubleshooting

**Model fails to load:**
- Check that the model path exists and contains `.json` files
- Verify the path in Settings → Live2D tab
- Check debug panel for error messages

**Falls back to Canvas placeholder:**
- This happens automatically if Live2D initialization fails
- Common causes: invalid model path, missing SDK, corrupted model files
- Check debug panel for detailed error information

**Low FPS:**
- Large models may impact performance
- Check FPS in debug panel
- Consider using simpler model variants

**Eye tracking not working:**
- Ensure mouse is over the main window
- Check that Live2D model supports eye tracking parameters
- Verify renderer initialization in debug panel
