## 1. Model Registry (模型注册表)

- [x] 1.1 创建 `src/core/model/ModelRegistry.ts`，定义 `PetModelConfig` 接口（id, name, description, modelUrl, cubismVersion）
- [x] 1.2 实现 `ModelRegistry` 类，包含 `register(config)`、`getAll()`、`getById(id)`、`getDefault()` 方法
- [x] 1.3 注册 Hiyori 模型：`./models/Hiyori/Hiyori.model3.json`，Cubism 4
- [x] 1.4 注册 Mao 模型：`./models/Mao/Mao.model3.json`，Cubism 4
- [x] 1.5 注册 Natori 模型：`./models/Natori/Natori.model3.json`，Cubism 4
- [x] 1.6 导出单例 `modelRegistry` 实例，默认模型为 Hiyori

## 2. PetStore (响应式状态管理)

- [x] 2.1 创建 `src/core/model/PetStore.ts`，使用 Vue `reactive()` 定义 store 状态
- [x] 2.2 实现 `currentModel`（ref）和 `models`（ref）状态字段
- [x] 2.3 实现 `setCurrentModel(model)` 方法，更新 currentModel
- [x] 2.4 实现 `switchToNextModel()` 方法，循环切换到下一个模型
- [x] 2.5 初始化时从 `modelRegistry` 加载默认模型和全部模型列表

## 3. Renderer Enhancement (渲染器增强)

- [x] 3.1 在 `Live2DRenderer.ts` 中新增 `playMotion(group: string, index?: number): Promise<void>` 方法
- [x] 3.2 `playMotion` 不传 index 时从该组随机选择，无效组名时静默失败并打印警告
- [x] 3.3 新增 `playExpression(nameOrIndex: string | number): Promise<void>` 方法
- [x] 3.4 模型无表情时 `playExpression` 静默失败并打印警告
- [x] 3.5 新增 `getMotionGroups(): { name: string, count: number }[]` 方法，从 `model.internalModel.motionManager.definitions` 自动解析
- [x] 3.6 新增 `getExpressions(): { name: string }[]` 方法，从模型 settings 自动解析表情列表
- [x] 3.7 重构 `loadModel` 中的 idle 动画启动逻辑：自动查找含 "idle"（不区分大小写）的动画组并播放第一个，替代硬编码的 `"Idle"`

## 4. Live2DCanvas Integration (画布集成)

- [x] 4.1 移除 `Live2DCanvas.vue` 中硬编码的 CDN Haru 模型 URL
- [x] 4.2 引入 `PetStore`，使用 `petStore.currentModel.modelUrl` 作为模型加载来源
- [x] 4.3 监听 `currentModel` 变化，自动调用 `renderer.loadModel(newModelUrl)` 加载新模型
- [x] 4.4 模型切换完成后在控制台输出新模型名称日志

## 5. Model Switching UI (切换 UI)

- [x] 5.1 在 `PetHoverMenu.vue` 菜单项数组中新增 `{ action: "switchModel", icon: "🔄", label: "切换模型" }`
- [x] 5.2 在 `App.vue` 的 `handleMenuAction` 中处理 `switchModel` 动作，调用 `petStore.switchToNextModel()`

## 6. Verification (验证)

- [ ] 6.1 启动应用，验证默认加载 Hiyori 模型并自动播放 idle 动画
- [ ] 6.2 点击"切换模型"按钮，验证依次切换 Hiyori → Mao → Natori → Hiyori
- [ ] 6.3 验证切换后新模型正确渲染、旧模型被销毁、控制台输出模型名称
- [ ] 6.4 验证窗口拖拽功能在切换模型后仍正常工作
