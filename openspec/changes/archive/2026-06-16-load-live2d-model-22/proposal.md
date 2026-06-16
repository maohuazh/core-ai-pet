## Why

The Core AI Pet application currently uses a placeholder/mock character for visual representation. To provide a more engaging and professional desktop pet experience, we need to load and display an actual Live2D model. The model located at `C:\Change Top\git\open-source\live2d\model\22` contains multiple character variants (different outfits and styles) that can be used as the primary visual avatar for the AI pet.

## What Changes

- Add support for loading Live2D models from external directories (specifically the "22" model collection)
- Enable runtime selection of different model variants (multiple JSON model files available)
- Integrate the model loading with the existing Live2DBridge rendering pipeline
- Support dynamic model switching without application restart

## Capabilities

### New Capabilities

- `live2d-model-loading`: Capability to load, initialize, and manage Live2D models from external file paths, including model variant selection and lifecycle management

### Modified Capabilities

(none)

## Impact

- **Code**: C# WPF application layer (CoreAIpet.Desktop) will need model path configuration and variant selection UI
- **Dependencies**: Live2DBridge C++ DLL already supports model loading via `Bridge_LoadModel` - no changes needed at the bridge layer
- **Assets**: Requires access to Live2D model files at `C:\Change Top\git\open-source\live2d\model\22`
- **Configuration**: Need to persist selected model path and variant in application settings
