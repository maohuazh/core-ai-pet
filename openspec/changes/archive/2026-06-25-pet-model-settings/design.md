## Context

当前系统有两套独立的模型数据源：
1. **前端内存** — `ModelRegistry` 硬编码注册 7 个宠物模型（4 Live2D + 3 Sprite），`PetStore` 从中读取并通过 Vue 响应式驱动 `Live2DCanvas` 渲染
2. **后端数据库** — SQLite `models` 表有完整 schema 但无种子数据，设置窗口通过 `invoke('get_models')` 查询得到空列表

设置窗口的"模型"标签页虽然 UI 已实现卡片列表 + 切换按钮，但因 DB 为空显示"暂无模型"。且 `set_active_model` 仅更新 DB 状态，不通知宠物窗口，切换无效。

两个 Tauri 窗口（宠物窗口 + 设置窗口）各自独立 webview，不共享 JS 状态，跨窗口通信必须走 Tauri 事件系统。

## Goals / Non-Goals

**Goals:**
- 设置窗口宠物列表展示所有内置宠物（Live2D + Sprite），数据来自 SQLite
- 标签名从"模型"改为"宠物"，符合桌面宠物产品定位
- 在设置窗口切换宠物后，桌面宠物窗口实时切换渲染
- 重启应用后恢复上次选择的宠物

**Non-Goals:**
- 不做模型导入功能（已有占位按钮，后续迭代）
- 不做动作映射配置（已有占位按钮，后续迭代）
- 不做自定义宠物上传/管理
- 不改变 `ModelRegistry` 的前端注册机制（保持前端作为渲染层的模型描述源）

## Decisions

### Decision 1: 种子数据在 Rust 后端 `initialize_mock_data` 中插入

**选择**: 在 `mod.rs` 的 `initialize_mock_data` 函数中插入所有 7 个内置宠物的 DB 记录

**替代方案**:
- A) 前端 `onMounted` 时批量 `invoke('insert_model')` — 需要新命令，且首次启动后 DB 才有数据
- B) 在 SQL migration 中硬编码 — 不够灵活，且与现有 mock data 模式不一致

**理由**: 复用现有的 `initialize_mock_data` 模式（Jira/Email/Chat 都是这样做的），保持一致性。种子数据与 `ModelRegistry` 中的模型一一对应。

### Decision 2: 跨窗口通信用 Tauri event `pet-model-changed`

**选择**: 设置窗口 `set_active_model` 命令执行后，通过 `app.emit("pet-model-changed", { model_id })` 广播事件；宠物窗口在 `App.vue` 中 `listen` 该事件，回调中调用 `petStore.setCurrentModel()`

**替代方案**:
- A) 设置窗口直接操作 PetStore — 两个窗口是独立 webview，不共享 JS 运行时，不可行
- B) 使用 Tauri 的 `invoke` 轮询 — 延迟高，浪费资源
- C) SharedArrayBuffer — 跨 webview 不支持

**理由**: Tauri event 是官方推荐的跨窗口通信方式，轻量且实时。

### Decision 3: 宠物窗口启动时从 DB 同步 active 模型

**选择**: 在 `App.vue`（非 settings 路由时）的 `onMounted` 中调用 `invoke('get_active_model')` 获取当前 active 模型 ID，然后从 `modelRegistry.getById()` 找到对应的 `PetModelConfig`，调用 `petStore.setCurrentModel()`

**替代方案**:
- A) 依赖 `ModelRegistry.setDefault()` 硬编码 — 无法记住用户选择
- B) 用 `app_settings` KV 表存 active model ID — 已有 `models` 表的 `status` 字段，不需要额外的存储

**理由**: 利用已有的 DB `status='active'` 字段作为持久化源，ModelRegistry 作为渲染配置源，两者通过 model ID 关联。

### Decision 4: Model ID 关联策略

DB 中的 model `id` 与 `ModelRegistry` 中的 `id` 保持一致（haru, hiyori, mao, natori, pixel-cat, arisa, panda）。前端收到 active model ID 后直接从 ModelRegistry 查找完整配置。

## Risks / Trade-offs

- **[Risk] DB 与 ModelRegistry 不同步** → 种子数据中的 ID/名称与 ModelRegistry 硬编码绑定，修改 ModelRegistry 时需要同步更新种子数据。Mitigation: 在 ModelRegistry.ts 顶部加注释提醒。
- **[Risk] 设置窗口打开时宠物窗口已关闭事件** → 如果设置窗口切换模型时宠物窗口还未完成 listen 注册，可能漏接事件。Mitigation: 宠物窗口在 `onMounted` 中先 listen 再初始化渲染器。
- **[Trade-off] 种子数据方式意味着每次新建 DB 都会插入固定模型列表** → 后续添加新宠物需要同时更新 ModelRegistry 和种子数据。当前阶段可接受，未来需要模型管理系统。
