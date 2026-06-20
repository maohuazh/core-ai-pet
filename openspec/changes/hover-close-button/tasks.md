## 1. 关闭按钮组件

- [x] 1.1 创建 `src/components/WindowCloseButton.vue` 组件，实现 28x28px × 按钮，默认 opacity 0.3，hover 时 opacity 1 + 红色背景
- [x] 1.2 点击时调用 `getCurrentWindow().close()` 关闭窗口

## 2. 集成到 App.vue

- [x] 2.1 在 `App.vue` 中导入并添加 `WindowCloseButton` 组件，使用 `v-if="showMenu"` 控制显示，定位在右上角

## 3. 验证

- [x] 3.1 构建验证：TypeScript 编译无错误，应用正常运行
- [x] 3.2 功能验证：悬停时按钮显示/隐藏正常，点击退出程序
