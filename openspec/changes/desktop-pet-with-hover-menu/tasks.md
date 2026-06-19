## 1. 项目初始化

- [x] 1.1 使用 Tauri CLI 创建项目脚手架（`cargo create-tauri-app`），选择 Vue 3 + TypeScript 模板
- [x] 1.2 配置 tauri.conf.json：设置 decorations: false、transparent: true、alwaysOnTop: true、skipTaskbar: true
- [x] 1.3 配置 Cargo.toml：添加 tauri 依赖和窗口管理相关 feature
- [x] 1.4 配置 vite.config.ts：设置开发服务器端口和构建输出目录
- [x] 1.5 验证项目可以成功构建和运行（`npm run tauri dev`）

## 2. Live2D SDK 集成

- [ ] 2.1 下载 Live2D Cubism 5 SDK，将 cubismcore.js 放入 `src/core/renderer/live2d/lib/`
- [ ] 2.2 下载示例 Live2D 模型（如 Haru），放入 `public/models/haru/`
- [x] 2.3 在 index.html 中通过 `<script>` 标签加载 cubismcore.js
- [x] 2.4 安装 pixi.js（v7.x）和 pixi-live2d-display 依赖
- [ ] 2.5 验证 Cubism SDK 可以正常加载（控制台无报错）

## 3. Rust 后端实现

- [x] 3.1 创建 `src-tauri/src/commands/window.rs`：实现 start_dragging、set_window_position、get_window_position 命令
- [x] 3.2 创建 `src-tauri/src/services/window/`：实现 WindowService 封装窗口操作逻辑
- [x] 3.3 在 `src-tauri/src/lib.rs` 中注册所有 Tauri Commands
- [x] 3.4 验证 Tauri Commands 可以通过前端 invoke 调用

## 4. Vue 前端 - Live2D 渲染器

- [x] 4.1 创建 `src/core/renderer/live2d/Live2DRenderer.ts`：实现 IRenderer 接口，封装 pixi-live2d-display
- [x] 4.2 实现 Live2D 模型加载逻辑：初始化 PixiJS Application，加载 .model3.json
- [x] 4.3 实现模型动画播放：自动播放 idle 动画，使用 requestAnimationFrame 渲染循环
- [x] 4.4 创建 `src/components/Live2DCanvas.vue`：包裹 Live2DRenderer，提供 Canvas 容器
- [x] 4.5 验证 Live2D 模型可以在窗口中正常显示和播放动画

## 5. Vue 前端 - 透明窗口和拖拽

- [x] 5.1 修改 `src/App.vue`：设置窗口容器样式（width: 100vw, height: 100vh, background: transparent）
- [x] 5.2 设置 CSS pointer-events: none 在窗口容器上，实现透明区域鼠标穿透
- [x] 5.3 设置 CSS pointer-events: auto 在 Live2DCanvas 上，使模型区域可交互
- [x] 5.4 实现拖拽功能：在 Live2DCanvas 上监听 mousedown 事件，调用 invoke('start_dragging')
- [x] 5.5 验证窗口透明、鼠标穿透、拖拽功能正常工作

## 6. Vue 前端 - 悬浮菜单

- [x] 6.1 创建 `src/components/PetHoverMenu.vue`：实现悬浮菜单组件
- [x] 6.2 实现菜单显示逻辑：监听 mouseenter 事件，设置 showMenu = true
- [x] 6.3 实现菜单隐藏逻辑：监听 mouseleave 事件，设置 showMenu = false（延迟 200ms）
- [x] 6.4 实现径向布局：使用 CSS transform 将 5 个按钮按圆形排列（半径 80px）
- [x] 6.5 实现 popIn 动画：使用 CSS transition，每个按钮延迟 0.05s
- [x] 6.6 实现按钮样式：圆形（40px）、半透明背景、白色边框、图标
- [x] 6.7 实现按钮 hover 效果：放大 1.2 倍、显示 tooltip、背景高亮
- [x] 6.8 实现按钮点击事件：chat（占位）、settings（占位）、menu（占位）、minimize（hide window）、close（exit app）
- [x] 6.9 验证悬浮菜单在鼠标进入宠物区域时正确显示，离开时正确隐藏

## 7. 集成和测试

- [x] 7.1 在 App.vue 中集成 Live2DCanvas 和 PetHoverMenu 组件
- [x] 7.2 端到端测试：启动应用，验证透明窗口、Live2D 渲染、拖拽、悬浮菜单全部正常
- [ ] 7.3 修复已知问题：窗口穿透精度、菜单动画流畅度、拖拽体验
- [x] 7.4 打包测试：运行 `npm run tauri build`，验证生成的可执行文件可以正常运行
