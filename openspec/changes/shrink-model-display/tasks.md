## 1. 窗口尺寸调整

- [x] 1.1 修改 `src-tauri/tauri.conf.json` 中窗口 width/height 从 400 改为 300

## 2. 渲染器调整

- [x] 2.1 修改 `Live2DRenderer.ts` 中 PixiJS Application width/height 从 400 改为 300
- [x] 2.2 修改模型缩放因子从 `0.9` 改为 `0.6`

## 3. 验证

- [x] 3.1 构建验证：TypeScript 编译无错误，Rust 编译无错误
- [x] 3.2 功能验证：窗口缩小到 300x300，模型显示更小，悬浮按钮大小和位置不变
