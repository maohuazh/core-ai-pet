## Context

CoreAIpet桌面宠物应用需要实现动作与表情映射配置系统，允许用户为每个模型自定义在不同业务场景下的视觉表现。当前系统已有：
- ModelRegistry和PetStore管理模型状态
- Live2DRenderer和SpriteSheetRenderer提供playMotion()和playExpression()接口
- SQLite数据库存储模型配置（models表）
- 设置面板（SettingsPanel）展示模型列表
- 事件系统（eventBus）用于组件通信

现有限制：
- 模型的动作/表情资源列表仅在运行时可获取（通过Renderer接口）
- 缺少持久化的映射配置机制
- 缺少根据外部事件触发特定表现的运行时逻辑

## Goals / Non-Goals

**Goals:**
- 实现10种预定义触发场景的映射配置（daily_1-3, new_message, new_task, new_email, task_in_progress, task_completed, task_approaching_deadline, task_overdue）
- 每个场景支持配置Motion（动作分组+动作名）、Expression（表情名）、Effect（特效名+持续时间+位置）
- 从Live2D的.model3.json和SpriteSheet的manifest.json提取可用资源列表
- 配置持久化到SQLite，支持实时预览
- daily_1必填约束（必须配置或使用默认值）

**Non-Goals:**
- 不实现自定义触发场景（固定10种，后续可扩展）
- 不实现时间轴/关键帧编辑器（仅映射到预定义动作）
- 不实现条件组合触发器（每个场景独立触发）
- 不实现动作混合/过渡动画（场景切换直接替换）
- 不实现特效编辑器（特效为系统预定义列表）

## Decisions

### Decision 1: 数据库表结构设计

**选择**: 使用单一`model_action_mappings`表，每条记录对应一个(model_id, trigger_key)组合。

```sql
CREATE TABLE model_action_mappings (
    id TEXT PRIMARY KEY,
    model_id TEXT NOT NULL,
    trigger_key TEXT NOT NULL,
    motion_group TEXT,
    motion_name TEXT,
    expression_name TEXT,
    effect_name TEXT,
    effect_duration INTEGER,
    effect_position TEXT DEFAULT 'center',
    use_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(model_id, trigger_key)
);
```

**理由**: 
- `UNIQUE(model_id, trigger_key)`确保每个场景只有一条配置，简化查询
- 扁平化设计，避免嵌套JSON，便于SQL查询和索引
- 与PRD中的数据模型一致

**替代方案**: 
- 使用JSON字段存储motion/expression/effect对象 → 增加查询复杂度，不利于索引
- 拆分为多张表（motions, expressions, effects）→ 增加JOIN复杂度，场景数固定为10，单一表更简单

### Decision 2: 资源提取时机

**选择**: 在Rust后端实现资源提取函数，前端通过Tauri IPC调用。

```rust
#[tauri::command]
async fn get_available_motions(model_id: String) -> Result<Vec<MotionInfo>, String>

#[tauri::command]
async fn get_available_expressions(model_id: String) -> Result<Vec<ExpressionInfo>, String>
```

**理由**:
- Live2D的.model3.json和SpriteSheet的manifest.json都是本地文件，Rust直接读取更高效
- 前端已有Live2DRenderer/SpriteSheetRenderer的getMotionGroups()/getExpressions()接口，但仅在模型加载后可用
- 后端提取可在模型未加载时获取资源列表，支持配置面板独立于宠物窗口工作
- 避免前端重复解析模型文件

**替代方案**:
- 前端从已加载的Renderer获取资源列表 → 需要宠物窗口和设置窗口共享状态，增加复杂度
- 数据库缓存资源列表 → 增加同步复杂度，资源列表应实时从模型文件读取

### Decision 3: 前端组件结构

**选择**: 
- `ActionMappingPanel.vue` - 全屏面板或弹窗容器
- `MappingRow.vue` - 单行场景配置（复用10次）
- `MotionSelector.vue` / `ExpressionSelector.vue` / `EffectSelector.vue` - 独立选择器组件
- `actionMappingService.ts` - 服务层封装IPC调用

**理由**:
- MappingRow复用10次，避免代码重复
- 选择器组件独立，便于测试和复用
- 服务层封装IPC，组件不直接调用invoke()，降低耦合

**替代方案**:
- 单一巨型组件包含所有逻辑 → 难以维护和测试
- 每个场景独立组件 → 10个相似组件，代码重复

### Decision 4: 实时预览机制

**选择**: 通过Tauri事件系统，设置窗口发送`preview-action-mapping`事件到宠物窗口，宠物窗口调用Renderer接口播放。

```typescript
// 设置窗口
await emit("preview-action-mapping", {
  modelId: string,
  motionGroup?: string,
  motionName?: string,
  expressionName?: string,
  effectName?: string,
});

// 宠物窗口
listen("preview-action-mapping", (event) => {
  if (event.payload.motionGroup) {
    renderer.playMotion(event.payload.motionGroup, event.payload.motionName);
  }
  if (event.payload.expressionName) {
    renderer.playExpression(event.payload.expressionName);
  }
  // TODO: effectManager.play(event.payload.effectName);
});
```

**理由**:
- 复用现有跨窗口通信模式（与pet-model-changed一致）
- 预览不影响数据库状态，仅触发播放
- 宠物窗口已有Renderer实例，直接调用接口

**替代方案**:
- 设置窗口内嵌小型预览Canvas → 需要加载第二个Renderer实例，内存开销大
- 通过主进程转发 → 增加不必要的中间层

### Decision 5: 运行时触发集成

**选择**: 新增`triggerHandler.ts`，监听eventBus的`external-event`，根据事件类型映射到trigger_key，查询数据库获取配置，调用Renderer接口执行。

```typescript
eventBus.on("external-event", async (event) => {
  const triggerKey = eventTypeToTriggerKey(event.type);
  if (!triggerKey) return;
  
  const mapping = await actionMappingService.getMapping(activeModelId, triggerKey);
  if (!mapping || mapping.useDefault) return;
  
  if (mapping.motionName) {
    renderer.playMotion(mapping.motionGroup, mapping.motionName);
  }
  if (mapping.expressionName) {
    renderer.playExpression(mapping.expressionName);
  }
});
```

**理由**:
- 与现有eventBus解耦，triggerHandler独立模块
- 查询数据库获取配置，确保使用最新配置
- 支持daily-tick定时器随机触发daily_1/2/3

**替代方案**:
- 在eventBus中直接处理映射逻辑 → 违反单一职责原则
- 缓存所有映射配置到内存 → 配置更新后需要同步缓存，增加复杂度

### Decision 6: daily_1默认值处理

**选择**: 
- 新模型首次打开配置面板时，如果无任何映射记录，自动创建10条默认记录
- daily_1的`use_default = 1`，其余9个场景`use_default = 0`且所有字段为NULL
- 前端校验：保存时检查daily_1是否有配置或使用默认值

**理由**:
- 自动创建默认记录，避免每次查询都需要处理"无记录"特殊情况
- daily_1必填，使用默认值作为初始配置，降低用户认知负担
- 前端校验提供即时反馈，避免保存无效配置

**替代方案**:
- 不自动创建记录，查询时动态生成默认配置 → 增加查询逻辑复杂度
- daily_1默认不触发任何表现 → 违反"基础待机状态始终需要"的需求

## Risks / Trade-offs

**[Risk] Live2D模型动作分组命名不统一**
→ 提供分组排序和显示名映射，按字母顺序排列分组，统一显示格式`[分组名] 动作名`

**[Risk] 某些模型无表情文件**
→ 表情选择器为空时，禁用表情配置项，显示提示"该模型不支持表情"

**[Risk] 预览与运行时表现不一致**
→ 预览使用同一套Renderer接口（playMotion/playExpression），保证一致性

**[Risk] 特效系统尚未实现**
→ 特效配置项保留，但EffectSelector选择"无"时effect_name为NULL，运行时跳过特效播放

**[Risk] 事件触发过于频繁（如daily-tick）**
→ daily-tick默认5分钟触发一次，可配置触发间隔；用户可通过清空daily_2/daily_3配置禁用随机切换

**[Trade-off] 数据库表扁平化设计**
→ 优点：查询简单，索引友好
→ 缺点：如果未来需要支持更多配置维度（如声音、粒子效果），需要修改表结构

**[Trade-off] Rust后端提取资源列表**
→ 优点：配置面板可独立工作，不依赖宠物窗口
→ 缺点：需要Rust解析JSON文件，增加后端复杂度（但Live2D/SpriteSheet格式已稳定）
