## ADDED Requirements

### Requirement: No visible window border or background
窗口 SHALL 在所有平台上完全透明，不显示任何可见边框、轮廓线或背景色。仅 Live2D 模型像素区域可见。

#### Scenario: Window has no visible border on Windows
- **WHEN** 应用在 Windows 上启动
- **THEN** 窗口 SHALL 无任何可见边框或底色
- **THEN** 仅 Live2D 模型像素可见，窗口其余区域完全透明

#### Scenario: Rust backend enforces transparent background
- **WHEN** Tauri 窗口创建后
- **THEN** Rust 后端 SHALL 显式设置窗口背景色为 `rgba(0, 0, 0, 0)`
- **THEN** 不依赖 `tauri.conf.json` 的 `transparent: true` 作为唯一透明保障

#### Scenario: CSS prevents default borders
- **WHEN** 页面加载完成
- **THEN** `html`、`body`、`#app`、`.pet-container`、`canvas` 元素 SHALL 显式设置 `border: none; outline: none; background: transparent`
- **THEN** 不受任何浏览器默认样式影响
