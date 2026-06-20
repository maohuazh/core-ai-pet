# Core AI Pet

A Windows desktop AI pet assistant built with Tauri 2 + Vue 3 + Live2D.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | Tauri 2 |
| Backend | Rust |
| Frontend | Vue 3 + TypeScript |
| 2D Rendering | PixiJS 7 |
| Live2D | pixi-live2d-display |
| Storage | SQLite (rusqlite) |
| Build | Vite |

## Features

- 🎨 **Live2D Characters** — Multiple models (Hiyori, Mao, Natori, Haru)
- 🔄 **Model Switching** — Cycle through registered models
- 🖱️ **Drag & Drop** — Move the pet anywhere on screen
- 📌 **Always on Top** — Pet stays above other windows
- 🪟 **Transparent Window** — Borderless, transparent desktop overlay
- 💾 **Position Memory** — Window position persists across restarts
- 🔘 **Hover Menu** — 6 circular function buttons appear on hover
- 🧩 **Plugin System** — Extensible plugin architecture (config-driven)
- 📊 **State Machine** — Character state management (Idle/Thinking/Talking/etc)
- 📡 **Event Bus** — Publish-subscribe event system

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run build
npm run tauri build
```

## Architecture

See [docs/Architecture.md](docs/Architecture.md) for detailed architecture documentation.

### Quick Overview

```
src/                          # Frontend (Vue 3 + TypeScript)
├── components/               # UI components
├── core/
│   ├── avatar/               # Avatar abstraction layer
│   ├── events/               # EventBus bridge
│   ├── model/                # Model registry & state
│   ├── plugin/               # Plugin management
│   ├── renderer/live2d/      # Live2D rendering engine
│   ├── state/                # State machine bridge
│   └── storage/              # SQLite bridge
└── main.ts

src-tauri/src/                # Backend (Rust + Tauri 2)
├── core/
│   ├── state/                # State machine
│   ├── eventbus/             # Event bus
│   └── plugin/               # Plugin runtime
├── infrastructure/
│   └── storage/              # SQLite persistence
├── commands/                 # Tauri commands
└── main.rs
```

## Tauri Commands API

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `start_dragging` | - | `Result<()>` | Start window drag |
| `set_window_position` | `x: f64, y: f64` | `Result<()>` | Set window position |
| `get_window_position` | - | `Result<(f64, f64)>` | Get window position |
| `get_state` | - | `Result<String>` | Get current pet state |
| `set_state` | `state: String` | `Result<String>` | Force set pet state |
| `emit_event` | `eventType, source, payload` | `Result<()>` | Publish event |
| `subscribe_event` | `eventType: String` | `Result<String>` | Subscribe to event |
| `storage_get` | `key: String` | `Result<Option<String>>` | Get config value |
| `storage_set` | `key, value: String` | `Result<()>` | Set config value |
| `plugin_list` | - | `Result<Vec<PluginInfo>>` | List all plugins |
| `plugin_enable` | `pluginId: String` | `Result<()>` | Enable plugin |
| `plugin_disable` | `pluginId: String` | `Result<()>` | Disable plugin |

## License

Private project.
