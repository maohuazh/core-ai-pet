## 1. Rust 后端 - 窗口透明

- [ ] 1.1 在 `src-tauri/src/main.rs` 的 setup 中，获取窗口句柄并显式设置背景色为 `rgba(0, 0, 0, 0)`
- [ ] 1.2 验证 Rust 端编译通过（`cargo build`）

## 2. CSS 防御性透明样式

- [x] 2.1 在 `index.html` 中为 `html`、`body`、`#app` 添加 `border: none; outline: none`
- [x] 2.2 在 `src/App.vue` 中为 `.pet-container` 添加 `border: none; outline: none`
- [x] 2.3 在 `src/components/Live2DCanvas.vue` 中为 canvas 添加 `border: none; outline: none`

## 3. 验证

- [ ] 3.1 启动应用，确认窗口无任何可见边框或底色，仅显示 Live2D 模型
