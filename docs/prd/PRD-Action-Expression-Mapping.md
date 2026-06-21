# PRD: 动作与表情映射 (Action & Expression Mapping)

> 版本: 1.0
> 日期: 2026-06-20
> 状态: 草案

---

## 1. 概述 (Overview)

为 CoreAIpet 实现**动作与表情映射配置系统**，允许用户为每个已注册的模型自定义在不同业务场景下触发的动作（Motion）、表情（Expression）和特效（Effect）。

映射配置通过设置面板模型列表中的「⚙ 动作映射」按钮进入，提供直观的可视化配置界面。系统为每个模型预定义 10 种触发场景（日常 1-3、新消息、新任务等），每个场景可独立配置动作/表情/特效组合，也可选择使用模型默认值。

本功能是桌面宠物「智能表现」的核心——让宠物根据 Jira 任务状态、邮件到达、聊天消息等外部事件做出恰当的视觉反馈。

---

## 2. 目标 (Goals)

| # | 目标 | 验证标准 |
|---|------|----------|
| G1 | 模型列表中每个模型旁显示「动作映射」按钮 | 按钮可见，点击打开配置面板 |
| G2 | 配置面板展示全部 10 种触发场景 | 所有场景行均可见，标注必填/选填 |
| G3 | 每个场景可配置 Motion + Expression + Effect | 下拉选择从模型可用资源中加载 |
| G4 | Daily 1 为必填，使用模型默认值作为初始配置 | `use_default = 1`，其余场景初始为空 |
| G5 | 配置实时预览 | 修改后桌面宠物立即展示对应动作/表情 |
| G6 | 配置持久化到 SQLite | 重启后配置保留 |
| G7 | 支持动作/表情在不同场景间复用 | 同一 motion/expression 可被多个场景引用 |
| G8 | 支持 Live2D 和 SpriteSheet 两种模型类型 | 资源列表按模型类型动态生成 |

---

## 3. 非目标 (Non-Goals)

| # | 非目标 | 原因 |
|---|--------|------|
| NG1 | 不实现自定义触发场景 | 固定 10 种场景，后续可扩展 |
| NG2 | 不实现时间轴/关键帧编辑器 | 映射只到预定义动作，不创建新动画 |
| NG3 | 不实现条件组合触发器 | 每个场景独立触发，不支持 AND/OR 逻辑 |
| NG4 | 不实现动作混合/过渡动画 | 场景切换直接替换，不插值 |
| NG5 | 不实现特效编辑器 | 特效为预定义列表，仅做选择 |

---

## 4. 功能需求 (Functional Requirements)

### 4.1 触发场景定义

系统预定义以下 10 种触发场景：

| # | 触发键 (trigger_key) | 显示名 | 必填 | 默认值 | 说明 |
|---|----------------------|--------|------|--------|------|
| 1 | `daily_1` | 日常1 | ✅ 是 | 使用模型默认 | 宠物的基础待机状态，始终需要 |
| 2 | `daily_2` | 日常2 | ❌ 否 | 空（不触发） | 第二日常状态，随机切换 |
| 3 | `daily_3` | 日常3 | ❌ 否 | 空（不触发） | 第三日常状态，随机切换 |
| 4 | `new_message` | 新消息 | ❌ 否 | 空（不触发） | 收到聊天消息时触发 |
| 5 | `new_task` | 新任务 | ❌ 否 | 空（不触发） | Jira 新任务分配时触发 |
| 6 | `new_email` | 新邮件 | ❌ 否 | 空（不触发） | 收到新邮件时触发 |
| 7 | `task_in_progress` | 任务进行中 | ❌ 否 | 空（不触发） | Jira 任务状态变为进行中 |
| 8 | `task_completed` | 任务已完成 | ❌ 否 | 空（不触发） | Jira 任务完成时触发 |
| 9 | `task_approaching_deadline` | 任务将到期 | ❌ 否 | 空（不触发） | 任务截止时间临近（< 24h） |
| 10 | `task_overdue` | 任务超时 | ❌ 否 | 空（不触发） | 任务超过截止时间 |

### 4.2 配置项结构

每个触发场景可配置以下三项：

```typescript
interface ActionMappingConfig {
  // 动作/动画
  motion?: {
    group: string;       // 动作分组（如 "Idle", "Walk", "Tap"）
    name: string;        // 具体动作名（如 "idle_01", "walk_03"）
  };

  // 表情
  expression?: {
    name: string;        // 表情名（如 "happy", "sad", "angry"）
  };

  // 特效（可选）
  effect?: {
    name: string;        // 特效名（如 "sparkle", "heart", "sweat_drop"）
    duration?: number;   // 特效持续时间（ms），默认 2000
    position?: "center" | "above" | "below";  // 特效位置
  };

  // 是否使用模型默认值
  useDefault: boolean;
}
```

### 4.3 动作/表情资源列表

#### 4.3.1 Live2D 模型

从 `.model3.json` 及关联文件中提取可用资源：

```typescript
interface Live2DAvailableResources {
  // 从 .model3.json 的 FileReferences.Motions 提取
  motions: {
    group: string;        // 如 "Idle", "TapBody", "TapHead"
    files: string[];      // .motion3.json 文件列表
  }[];

  // 从 .model3.json 的 FileReferences.Expressions 提取
  expressions: {
    name: string;         // 表情名
    file: string;         // .exp3.json 文件路径
  }[];
}
```

#### 4.3.2 SpriteSheet 模型

从 `manifest.json` 的 `motions` 和 `expressions` 字段提取：

```typescript
interface SpriteAvailableResources {
  // 从 manifest.motions 提取
  motions: {
    name: string;         // 动作名（即 motion key，如 "idle", "walk"）
    state: string;        // 映射到的状态
    group: string;        // 分组
  }[];

  // 从 manifest.expressions 提取
  expressions: {
    name: string;         // 表情名（即 expression key，如 "happy", "sad"）
    hasOverlay: boolean;  // 是否有覆盖层图片
  }[];
}
```

### 4.4 特效列表

特效为系统预定义，与具体模型无关：

```typescript
interface AvailableEffect {
  id: string;
  name: string;            // 显示名
  icon: string;            // 图标 emoji 或图片路径
  description: string;     // 描述
  defaultDuration: number; // 默认持续时间 (ms)
}

const AVAILABLE_EFFECTS: AvailableEffect[] = [
  { id: "sparkle",       name: "闪光",   icon: "✨", description: "闪烁的星光效果",     defaultDuration: 1500 },
  { id: "heart",         name: "爱心",   icon: "❤️", description: "飘浮的爱心",         defaultDuration: 2000 },
  { id: "sweat_drop",    name: "汗滴",   icon: "💧", description: "额头汗滴效果",       defaultDuration: 2500 },
  { id: "exclamation",   name: "感叹号", icon: "❗", description: "头顶感叹号",         defaultDuration: 1500 },
  { id: "question",      name: "问号",   icon: "❓", description: "头顶问号",           defaultDuration: 2000 },
  { id: "music_note",    name: "音符",   icon: "🎵", description: "跳动的音符",         defaultDuration: 2000 },
  { id: "zzz",           name: "睡眠",   icon: "💤", description: "睡眠 ZZZ 效果",      defaultDuration: 3000 },
  { id: "anger",         name: "怒气",   icon: "💢", description: "愤怒符号",           defaultDuration: 1500 },
  { id: "blush",         name: "脸红",   icon: "😊", description: "脸颊红晕",           defaultDuration: 2000 },
  { id: "star",          name: "星星",   icon: "⭐", description: "闪烁的星星",         defaultDuration: 1500 },
  { id: "check_mark",    name: "对勾",   icon: "✅", description: "完成对勾",           defaultDuration: 1500 },
  { id: "warning",       name: "警告",   icon: "⚠️", description: "警告标志",           defaultDuration: 2500 },
];
```

### 4.5 业务规则

| 规则 | 说明 |
|------|------|
| Daily 1 必填 | `daily_1` 的 action 不可为空，必须配置或使用默认值 |
| 默认值行为 | `use_default = 1` 时，使用模型内置的默认动作/表情/特效 |
| 空配置行为 | `use_default = 0` 且未配置任何项时，该场景不触发任何表现 |
| 部分配置 | 可以只配置 motion 不配置 expression，或反之 |
| 复用允许 | 同一个 motion/expression 可被多个场景引用 |
| 模型切换 | 切换模型后，新模型使用自己的映射配置（与模型绑定） |
| 实时预览 | 修改配置后，桌面宠物立即更新表现（通过事件系统触发） |

### 4.6 事件触发机制

```typescript
// 当外部事件发生时，查找对应场景的映射配置并触发

interface TriggerEvent {
  triggerKey: string;         // "new_message" / "task_overdue" 等
  payload?: Record<string, unknown>;  // 事件附加数据
}

async function handleTriggerEvent(event: TriggerEvent): Promise<void> {
  const modelId = petStore.activeModelId;

  // 从数据库查询映射配置
  const mapping = await db.getActionMapping(modelId, event.triggerKey);

  if (!mapping) {
    // 未配置，不触发
    return;
  }

  if (mapping.use_default) {
    // 使用模型默认表现
    avatar.playMotion("idle");  // 或模型的默认动作
    return;
  }

  // 应用配置
  if (mapping.motion_group && mapping.motion_name) {
    avatar.playMotion(mapping.motion_name, mapping.motion_group);
  }

  if (mapping.expression_name) {
    await avatar.setExpression(mapping.expression_name);
  }

  if (mapping.effect_name) {
    effectManager.play(mapping.effect_name);
  }
}
```

---

## 5. 技术设计 (Technical Design)

### 5.1 数据模型

```typescript
// 数据库记录类型
interface ActionMappingRecord {
  id: string;
  model_id: string;
  trigger_key: TriggerKey;
  motion_group: string | null;
  motion_name: string | null;
  expression_name: string | null;
  effect_name: string | null;
  effect_duration: number | null;     // ms
  effect_position: "center" | "above" | "below" | null;
  use_default: number;                // 0 or 1
  created_at: string;
  updated_at: string;
}

type TriggerKey =
  | "daily_1" | "daily_2" | "daily_3"
  | "new_message" | "new_task" | "new_email"
  | "task_in_progress" | "task_completed"
  | "task_approaching_deadline" | "task_overdue";

// 前端表单状态
interface MappingFormData {
  triggerKey: TriggerKey;
  useDefault: boolean;

  motion: {
    enabled: boolean;
    group: string;
    name: string;
  };

  expression: {
    enabled: boolean;
    name: string;
  };

  effect: {
    enabled: boolean;
    name: string;
    duration: number;
    position: "center" | "above" | "below";
  };
}
```

### 5.2 Tauri IPC 接口

```rust
// src-tauri/src/action_mapping_commands.rs

/// 获取模型的所有动作映射
#[tauri::command]
async fn get_action_mappings(
    db: State<'_, Database>,
    model_id: String,
) -> Result<Vec<ActionMappingRecord>, String> {
    db.get_action_mappings_by_model(&model_id)
        .map_err(|e| e.to_string())
}

/// 获取单个触发场景的映射
#[tauri::command]
async fn get_action_mapping(
    db: State<'_, Database>,
    model_id: String,
    trigger_key: String,
) -> Result<Option<ActionMappingRecord>, String> {
    db.get_action_mapping(&model_id, &trigger_key)
        .map_err(|e| e.to_string())
}

/// 创建或更新动作映射（upsert）
#[tauri::command]
async fn upsert_action_mapping(
    db: State<'_, Database>,
    model_id: String,
    trigger_key: String,
    motion_group: Option<String>,
    motion_name: Option<String>,
    expression_name: Option<String>,
    effect_name: Option<String>,
    effect_duration: Option<i64>,
    effect_position: Option<String>,
    use_default: i32,
) -> Result<ActionMappingRecord, String> {
    // 校验：daily_1 必须有动作或使用默认值
    if trigger_key == "daily_1" && use_default == 0 {
        if motion_name.is_none() && expression_name.is_none() {
            return Err("日常1 必须配置动作或表情，或选择使用默认值".to_string());
        }
    }

    db.upsert_action_mapping(&UpsertMappingParams {
        model_id: &model_id,
        trigger_key: &trigger_key,
        motion_group: motion_group.as_deref(),
        motion_name: motion_name.as_deref(),
        expression_name: expression_name.as_deref(),
        effect_name: effect_name.as_deref(),
        effect_duration,
        effect_position: effect_position.as_deref(),
        use_default,
    }).map_err(|e| e.to_string())
}

/// 批量更新动作映射
#[tauri::command]
async fn batch_upsert_action_mappings(
    db: State<'_, Database>,
    model_id: String,
    mappings: Vec<UpsertMappingParams>,
) -> Result<(), String> {
    // 事务性批量更新
    db.batch_upsert_action_mappings(&model_id, &mappings)
        .map_err(|e| e.to_string())
}

/// 获取模型的可用动作列表
#[tauri::command]
async fn get_available_motions(
    model_id: String,
) -> Result<Vec<MotionInfo>, String> {
    let model = get_model_info(&model_id)?;
    match model.model_type.as_str() {
        "live2d" => get_live2d_motions(&model.path),
        "sprite" => get_sprite_motions(&model.path),
        _ => Err("未知模型类型".to_string()),
    }
}

/// 获取模型的可用表情列表
#[tauri::command]
async fn get_available_expressions(
    model_id: String,
) -> Result<Vec<ExpressionInfo>, String> {
    let model = get_model_info(&model_id)?;
    match model.model_type.as_str() {
        "live2d" => get_live2d_expressions(&model.path),
        "sprite" => get_sprite_expressions(&model.path),
        _ => Err("未知模型类型".to_string()),
    }
}

// ---- 辅助数据结构 ----

#[derive(serde::Serialize, serde::Deserialize)]
struct MotionInfo {
    group: String,
    name: String,
    display_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ExpressionInfo {
    name: String,
    display_name: String,
    file: Option<String>,
}

#[derive(serde::Deserialize)]
struct UpsertMappingParams {
    model_id: String,
    trigger_key: String,
    motion_group: Option<String>,
    motion_name: Option<String>,
    expression_name: Option<String>,
    effect_name: Option<String>,
    effect_duration: Option<i64>,
    effect_position: Option<String>,
    use_default: i32,
}
```

### 5.3 前端组件设计

```
src/components/settings/
  └── modules/
      └── ActionMappingPanel.vue           ← 动作映射配置面板（全屏或弹窗）

src/components/action-mapping/
  ├── MappingRow.vue                       ← 单行场景配置（10 行之一）
  ├── MotionSelector.vue                   ← 动作下拉选择器
  ├── ExpressionSelector.vue               ← 表情下拉选择器
  ├── EffectSelector.vue                   ← 特效下拉选择器
  ├── PreviewArea.vue                      ← 实时预览区域
  ├── MappingToolbar.vue                   ← 工具栏（保存/重置/预览）
  └── types.ts                             ← 类型定义
```

### 5.4 前端服务层

```typescript
// src/core/action/actionMappingService.ts

class ActionMappingService {
  /**
   * 加载模型的所有映射配置
   */
  async loadMappings(modelId: string): Promise<MappingFormData[]> {
    const records = await invoke<ActionMappingRecord[]>(
      "get_action_mappings",
      { modelId }
    );

    // 如果无记录（新导入模型），返回默认配置
    if (records.length === 0) {
      return this.createDefaultMappings();
    }

    return records.map(r => this.recordToFormData(r));
  }

  /**
   * 保存映射配置
   */
  async saveMappings(
    modelId: string,
    formData: MappingFormData[]
  ): Promise<void> {
    // 校验
    const daily1 = formData.find(f => f.triggerKey === "daily_1");
    if (daily1 && !daily1.useDefault && !daily1.motion.enabled && !daily1.expression.enabled) {
      throw new Error("日常1 必须配置动作或表情，或选择使用默认值");
    }

    // 批量保存
    const params = formData.map(f => this.formDataToParams(f, modelId));
    await invoke("batch_upsert_action_mappings", {
      modelId,
      mappings: params,
    });

    // 如果是当前活跃模型，触发实时更新
    if (petStore.activeModelId === modelId) {
      eventBus.emit("action-mappings-updated", { modelId });
    }
  }

  /**
   * 加载模型的可用动作列表
   */
  async getAvailableMotions(modelId: string): Promise<MotionInfo[]> {
    return invoke<MotionInfo[]>("get_available_motions", { modelId });
  }

  /**
   * 加载模型的可用表情列表
   */
  async getAvailableExpressions(modelId: string): Promise<ExpressionInfo[]> {
    return invoke<ExpressionInfo[]>("get_available_expressions", { modelId });
  }

  /**
   * 预览指定场景的表现
   */
  async previewMapping(modelId: string, formData: MappingFormData): Promise<void> {
    if (formData.useDefault) {
      // 播放模型默认动作
      await petStore.previewDefaultMotion();
      return;
    }

    if (formData.motion.enabled) {
      await petStore.previewMotion(formData.motion.name, formData.motion.group);
    }

    if (formData.expression.enabled) {
      await petStore.previewExpression(formData.expression.name);
    }

    if (formData.effect.enabled) {
      effectManager.preview(formData.effect.name, formData.effect.duration);
    }
  }

  /**
   * 创建默认映射配置
   */
  private createDefaultMappings(): MappingFormData[] {
    const triggers = [
      "daily_1", "daily_2", "daily_3",
      "new_message", "new_task", "new_email",
      "task_in_progress", "task_completed",
      "task_approaching_deadline", "task_overdue",
    ];

    return triggers.map(key => ({
      triggerKey: key as TriggerKey,
      useDefault: key === "daily_1",  // 仅 daily_1 默认使用模型默认值
      motion: { enabled: false, group: "", name: "" },
      expression: { enabled: false, name: "" },
      effect: { enabled: false, name: "", duration: 2000, position: "center" as const },
    }));
  }

  private recordToFormData(record: ActionMappingRecord): MappingFormData {
    return {
      triggerKey: record.trigger_key as TriggerKey,
      useDefault: record.use_default === 1,
      motion: {
        enabled: !!record.motion_name,
        group: record.motion_group || "",
        name: record.motion_name || "",
      },
      expression: {
        enabled: !!record.expression_name,
        name: record.expression_name || "",
      },
      effect: {
        enabled: !!record.effect_name,
        name: record.effect_name || "",
        duration: record.effect_duration || 2000,
        position: (record.effect_position as "center" | "above" | "below") || "center",
      },
    };
  }

  private formDataToParams(
    form: MappingFormData,
    modelId: string
  ): UpsertMappingParams {
    return {
      model_id: modelId,
      trigger_key: form.triggerKey,
      motion_group: form.motion.enabled ? form.motion.group : null,
      motion_name: form.motion.enabled ? form.motion.name : null,
      expression_name: form.expression.enabled ? form.expression.name : null,
      effect_name: form.effect.enabled ? form.effect.name : null,
      effect_duration: form.effect.enabled ? form.effect.duration : null,
      effect_position: form.effect.enabled ? form.effect.position : null,
      use_default: form.useDefault ? 1 : 0,
    };
  }
}

export const actionMappingService = new ActionMappingService();
```

### 5.5 Live2D 资源提取

```rust
// 从 .model3.json 提取动作和表情列表

fn get_live2d_motions(model_path: &str) -> Result<Vec<MotionInfo>, String> {
    let path = PathBuf::from(model_path);
    let model3_files: Vec<_> = std::fs::read_dir(&path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .ends_with(".model3.json")
        })
        .collect();

    if model3_files.is_empty() {
        return Err("未找到 .model3.json 文件".to_string());
    }

    let content = std::fs::read_to_string(model3_files[0].path())?;
    let model3: serde_json::Value = serde_json::from_str(&content)?;

    let mut motions = Vec::new();

    if let Some(motions_map) = model3["FileReferences"]["Motions"].as_object() {
        for (group_name, motions_value) in motions_map {
            if let Some(motion_array) = motions_value.as_array() {
                for (i, motion) in motion_array.iter().enumerate() {
                    let file = motion["File"].as_str().unwrap_or("");
                    let motion_name = PathBuf::from(file)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| format!("motion_{}", i));

                    motions.push(MotionInfo {
                        group: group_name.clone(),
                        name: motion_name.clone(),
                        display_name: format!("[{}] {}", group_name, motion_name),
                    });
                }
            }
        }
    }

    Ok(motions)
}

fn get_live2d_expressions(model_path: &str) -> Result<Vec<ExpressionInfo>, String> {
    // 类似逻辑，从 FileReferences.Expressions 提取
    // ...
}
```

### 5.6 SpriteSheet 资源提取

```rust
fn get_sprite_motions(model_path: &str) -> Result<Vec<MotionInfo>, String> {
    let manifest_path = PathBuf::from(model_path).join("manifest.json");
    let content = std::fs::read_to_string(&manifest_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;

    let mut motions = Vec::new();

    if let Some(motions_map) = manifest["motions"].as_object() {
        for (name, value) in motions_map {
            let state = value["state"].as_str().unwrap_or("");
            let group = value["group"].as_str().unwrap_or("default");

            motions.push(MotionInfo {
                group: group.to_string(),
                name: name.clone(),
                display_name: format!("{} ({})", name, state),
            });
        }
    }

    Ok(motions)
}

fn get_sprite_expressions(model_path: &str) -> Result<Vec<ExpressionInfo>, String> {
    let manifest_path = PathBuf::from(model_path).join("manifest.json");
    let content = std::fs::read_to_string(&manifest_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;

    let mut expressions = Vec::new();

    if let Some(expr_map) = manifest["expressions"].as_object() {
        for (name, value) in expr_map {
            let has_overlay = value["overlay"].is_string();
            expressions.push(ExpressionInfo {
                name: name.clone(),
                display_name: name.clone(),
                file: value["overlay"].as_str().map(|s| s.to_string()),
            });
        }
    }

    Ok(expressions)
}
```

### 5.7 运行时触发集成

```typescript
// src/core/events/triggerHandler.ts
// 与现有事件系统（src/core/events/index.ts）集成

import { eventBus } from "../events";
import { actionMappingService } from "../action/actionMappingService";
import { petStore } from "../model/PetStore";

// 触发场景到 PetState 的映射关系（可选，用于状态联动）
const TRIGGER_TO_PET_STATE: Partial<Record<TriggerKey, PetState>> = {
  daily_1: "Idle",
  daily_2: "Idle",
  daily_3: "Idle",
  new_message: "Alert",
  new_task: "Alert",
  new_email: "Alert",
  task_in_progress: "Working",
  task_completed: "Idle",
  task_approaching_deadline: "Alert",
  task_overdue: "Alert",
};

class TriggerHandler {
  constructor() {
    // 监听外部事件
    eventBus.on("external-event", this.handleExternalEvent.bind(this));

    // 监听日常状态切换（定时随机触发 daily_1/2/3）
    eventBus.on("daily-tick", this.handleDailyTick.bind(this));
  }

  async handleExternalEvent(event: { type: string; payload: any }): Promise<void> {
    const triggerKey = this.eventTypeToTriggerKey(event.type);
    if (!triggerKey) return;

    await this.fireTrigger(triggerKey);
  }

  async handleDailyTick(): Promise<void> {
    // 随机选择 daily_1/2/3（权重：daily_1 70%, daily_2 20%, daily_3 10%）
    const rand = Math.random();
    let triggerKey: TriggerKey;
    if (rand < 0.7) {
      triggerKey = "daily_1";
    } else if (rand < 0.9) {
      triggerKey = "daily_2";
    } else {
      triggerKey = "daily_3";
    }

    await this.fireTrigger(triggerKey);
  }

  private async fireTrigger(triggerKey: TriggerKey): Promise<void> {
    const modelId = petStore.activeModelId;
    const mapping = await actionMappingService.getMapping(modelId, triggerKey);

    if (!mapping || (mapping.useDefault === 0 && !mapping.hasAnyConfig)) {
      return; // 未配置，不触发
    }

    const avatar = petStore.currentAvatar;
    if (!avatar) return;

    // 1. 切换 PetState（可选）
    const petState = TRIGGER_TO_PET_STATE[triggerKey];
    if (petState) {
      avatar.setState(petState);
    }

    // 2. 播放动作
    if (mapping.useDefault) {
      // 使用模型默认动作
      avatar.playMotion("idle");
    } else if (mapping.motion) {
      avatar.playMotion(mapping.motion.name, mapping.motion.group);
    }

    // 3. 设置表情
    if (mapping.expression) {
      await avatar.setExpression(mapping.expression.name);
    }

    // 4. 播放特效
    if (mapping.effect) {
      effectManager.play(mapping.effect.name, {
        duration: mapping.effect.duration,
        position: mapping.effect.position,
      });
    }
  }

  private eventTypeToTriggerKey(eventType: string): TriggerKey | null {
    const mapping: Record<string, TriggerKey> = {
      "chat-message-received": "new_message",
      "jira-task-assigned": "new_task",
      "email-received": "new_email",
      "jira-task-status-changed": "task_in_progress",
      "jira-task-completed": "task_completed",
      "jira-task-deadline-approaching": "task_approaching_deadline",
      "jira-task-overdue": "task_overdue",
    };
    return mapping[eventType] || null;
  }
}

export const triggerHandler = new TriggerHandler();
```

---

## 6. 数据库设计 (Database Schema)

完整的 `model_action_mappings` 表定义参见 [PRD-Settings-Panel.md](./PRD-Settings-Panel.md#6-数据库设计-database-schema)。

此处补充特效相关字段和示例数据：

### 6.1 完整表结构（含特效字段）

```sql
CREATE TABLE IF NOT EXISTS model_action_mappings (
    id              TEXT PRIMARY KEY,
    model_id        TEXT NOT NULL REFERENCES models(id) ON DELETE CASCADE,

    -- 触发场景
    trigger_key     TEXT NOT NULL
                        CHECK (trigger_key IN (
                            'daily_1', 'daily_2', 'daily_3',
                            'new_message', 'new_task', 'new_email',
                            'task_in_progress', 'task_completed',
                            'task_approaching_deadline', 'task_overdue'
                        )),

    -- 动作配置
    motion_group    TEXT,                      -- 动作分组（如 "Idle", "TapBody"）
    motion_name     TEXT,                      -- 具体动作名
    expression_name TEXT,                      -- 表情名

    -- 特效配置
    effect_name     TEXT,                      -- 特效 ID（如 "sparkle", "heart"）
    effect_duration INTEGER,                   -- 特效持续时间 (ms)
    effect_position TEXT DEFAULT 'center'
                        CHECK (effect_position IN ('center', 'above', 'below')),

    -- 默认值标记
    use_default     INTEGER NOT NULL DEFAULT 0,

    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(model_id, trigger_key)
);

CREATE INDEX idx_action_mapping_model ON model_action_mappings(model_id);
CREATE INDEX idx_action_mapping_trigger ON model_action_mappings(trigger_key);
```

### 6.2 示例数据

```sql
-- 为内置模型 Hiyori 配置示例映射
INSERT INTO model_action_mappings (id, model_id, trigger_key, motion_group, motion_name, expression_name, effect_name, effect_duration, use_default) VALUES
-- 日常1：使用模型默认
('map-hiyori-daily1', 'hiyori-001', 'daily_1', NULL, NULL, NULL, NULL, NULL, 1),

-- 日常2：轻微动作
('map-hiyori-daily2', 'hiyori-001', 'daily_2', 'Idle', 'idle_02', 'neutral', NULL, NULL, 0),

-- 新消息：惊喜表情 + 闪光特效
('map-hiyori-newmsg', 'hiyori-001', 'new_message', 'TapBody', 'tap_body_01', 'surprised', 'sparkle', 1500, 0),

-- 新任务：认真表情
('map-hiyori-newtask', 'hiyori-001', 'new_task', 'Idle', 'idle_01', 'serious', 'exclamation', 1500, 0),

-- 任务完成：开心
('map-hiyori-done', 'hiyori-001', 'task_completed', 'Idle', 'idle_03', 'happy', 'check_mark', 1500, 0),

-- 任务超时：焦虑
('map-hiyori-overdue', 'hiyori-001', 'task_overdue', 'Idle', 'idle_01', 'angry', 'warning', 2500, 0);
```

### 6.3 查询示例

```sql
-- 获取模型的所有映射配置
SELECT * FROM model_action_mappings
WHERE model_id = :model_id
ORDER BY
  CASE trigger_key
    WHEN 'daily_1' THEN 1
    WHEN 'daily_2' THEN 2
    WHEN 'daily_3' THEN 3
    WHEN 'new_message' THEN 4
    WHEN 'new_task' THEN 5
    WHEN 'new_email' THEN 6
    WHEN 'task_in_progress' THEN 7
    WHEN 'task_completed' THEN 8
    WHEN 'task_approaching_deadline' THEN 9
    WHEN 'task_overdue' THEN 10
  END;

-- 获取当前活跃模型的指定触发场景映射
SELECT m.* FROM model_action_mappings m
JOIN models mo ON m.model_id = mo.id
WHERE mo.status = 'active'
  AND m.trigger_key = :trigger_key;
```

---

## 7. UI/UX 设计

### 7.1 入口

从设置面板的模型配置模块中，每个模型卡片上的「⚙ 动作映射」按钮进入。

### 7.2 面板布局

```
┌──────────────────────────────────────────────────────────────┐
│  ← 返回模型列表    动作映射: Hiyori          [保存] [重置]  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌── 预览区域 ──────────────────────────────────────────┐    │
│  │  [模型实时预览]   当前: Idle + neutral                │    │
│  │  点击行尾「预览」按钮可预览该场景的表现                │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ─── 日常状态 ─────────────────────────────────────────────  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ☀️ 日常1  ⭐必填                                    │    │
│  │                                                      │    │
│  │  [✓] 使用模型默认值                                  │    │
│  │                                                      │    │
│  │  ┌─ 动作 ────────────  ┌─ 表情 ──────  ┌─ 特效 ──┐  │    │
│  │  │ [分组 ▼] [动作 ▼]  │ [表情 ▼]     │ [特效 ▼] │  │    │
│  │  │  Idle     idle_01   │  neutral      │  (无)    │  │    │
│  │  └────────────────────  └─────────────  └─────────┘  │    │
│  │                                                      │    │
│  │                                      [▶ 预览]       │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ☀️ 日常2                                            │    │
│  │                                                      │    │
│  │  [ ] 使用模型默认值                                  │    │
│  │                                                      │    │
│  │  ┌─ 动作 ────────────  ┌─ 表情 ──────  ┌─ 特效 ──┐  │    │
│  │  │ [✓] [分组 ▼] ...   │ [✓] [...]     │ [ ] ...  │  │    │
│  │  └────────────────────  └─────────────  └─────────┘  │    │
│  │                                                      │    │
│  │                                      [▶ 预览]       │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ☀️ 日常3                                            │    │
│  │  (同上结构)                                          │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ─── 事件触发 ─────────────────────────────────────────────  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  💬 新消息                                           │    │
│  │  (配置区域同上)                                      │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  📋 新任务                                           │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  📧 新邮件                                           │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ⏳ 任务进行中                                       │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ✅ 任务已完成                                       │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ⚠️ 任务将到期                                       │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  🚨 任务超时                                         │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### 7.3 单行配置卡片详细设计

```
┌──────────────────────────────────────────────────────────────┐
│  💬 新消息                                            [▶]   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  [ ] 使用模型默认值                                         │
│                                                              │
│  ┌── 动作 ──────────────────────────────────────────────┐    │
│  │  [✓ 启用]                                            │    │
│  │                                                      │    │
│  │  分组:  [ TapBody         ▼]                         │    │
│  │  动作:  [ tap_body_01     ▼]                         │    │
│  │                                                      │    │
│  │  可用动作:                                            │    │
│  │  ┌──────────┬────────────────────────────────────┐   │    │
│  │  │ Idle     │ idle_01, idle_02, idle_03          │   │    │
│  │  │ TapBody  │ tap_body_01, tap_body_02           │   │    │
│  │  │ TapHead  │ tap_head_01                        │   │    │
│  │  └──────────┴────────────────────────────────────┘   │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌── 表情 ──────────────────────────────────────────────┐    │
│  │  [✓ 启用]                                            │    │
│  │  表情:  [ surprised       ▼]                         │    │
│  │                                                      │    │
│  │  可用: neutral, happy, sad, angry, surprised         │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌── 特效 ──────────────────────────────────────────────┐    │
│  │  [✓ 启用]                                            │    │
│  │  特效:  [✨ 闪光         ▼]                           │    │
│  │  持续:  [1500        ] ms                             │    │
│  │  位置:  (●) 居中  ( ) 上方  ( ) 下方                 │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### 7.4 下拉选择器设计

#### 动作分组+动作 级联选择

```
┌──────────────────────┐
│  分组: [TapBody    ▼]│
├──────────────────────┤
│  ▸ Idle       (3)   │
│  ▾ TapBody    (2)   │
│    ├─ tap_body_01    │
│    ├─ tap_body_02    │
│  ▸ TapHead    (1)   │
└──────────────────────┘

选中分组后，右侧显示该分组下的动作列表
```

#### 表情选择

```
┌──────────────────────┐
│  [surprised       ▼] │
├──────────────────────┤
│  ○ neutral           │
│  ○ happy       😊    │
│  ○ sad         😢    │
│  ○ angry       😠    │
│  ● surprised   😲    │
└──────────────────────┘
```

#### 特效选择

```
┌──────────────────────┐
│  [✨ 闪光         ▼] │
├──────────────────────┤
│  ○ (无)              │
│  ○ ✨ 闪光  (1.5s)  │
│  ○ ❤️ 爱心   (2.0s)  │
│  ○ 💧 汗滴   (2.5s)  │
│  ● ❗ 感叹号 (1.5s)  │
│  ○ ❓ 问号   (2.0s)  │
│  ...                 │
└──────────────────────┘
```

### 7.5 交互细节

| 操作 | 行为 |
|------|------|
| 勾选「使用模型默认值」| 禁用下方的动作/表情/特效配置区域，显示灰色遮罩 |
| 取消「使用模型默认值」| 启用配置区域，恢复上次手动配置（或空） |
| 勾选动作/表情/特效的「启用」| 显示对应配置项 |
| 取消「启用」| 隐藏配置项，清空该维度的选择 |
| 切换动作分组 | 右侧动作列表刷新为该分组下的动作 |
| 点击「▶ 预览」| 桌面宠物立即播放该场景配置的动作/表情/特效 |
| 点击「保存」| 校验 → 批量写入 SQLite → Toast 提示「已保存」|
| 点击「重置」| 放弃修改，从数据库重新加载 |
| 点击「← 返回」| 如有未保存修改，弹出确认对话框 |

### 7.6 校验规则

| 规则 | 消息 |
|------|------|
| daily_1 必须有动作或使用默认值 | "日常1 必须配置动作或表情，或选择使用默认值" |
| 启用动作时，分组和动作名不可为空 | "请选择动作分组和具体动作" |
| 启用表情时，表情名不可为空 | "请选择表情" |
| 启用特效时，特效名不可为空 | "请选择特效" |

### 7.7 用户流程

```
用户在设置面板 → 模型配置 → 点击某模型的「⚙ 动作映射」
  → 打开动作映射面板
  → 加载该模型的所有映射配置
  → 加载该模型的可用动作/表情列表
  → 展示 10 行配置卡片

用户编辑某行配置：
  → 修改动作/表情/特效选择
  → 点击「▶ 预览」→ 桌面宠物即时反馈
  → 重复编辑直到满意

用户点击「保存」：
  → 前端校验
  → 批量 upsert 到 SQLite
  → 发送 "action-mappings-updated" 事件
  → Toast "配置已保存"

用户点击「返回」：
  → 如有未保存修改 → 确认对话框
  → 关闭面板，返回模型列表
```

---

## 8. 实现计划 (Implementation Plan)

### 阶段一：数据层（预计 2 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 数据库表创建/迁移 | PRD-Settings-Panel 的 DB 设计 | migration SQL |
| Rust 端 CRUD 命令实现 | 表结构 | `action_mapping_commands.rs` |
| Live2D 动作/表情提取函数 | 无 | Rust 端解析逻辑 |
| SpriteSheet 动作/表情提取函数 | PRD-SpriteSheet-Renderer | Rust 端解析逻辑 |
| 前端 `ActionMappingService` | 后端命令 | TypeScript 服务层 |

**验证点**：能通过 IPC 读写映射配置，能获取模型的可用动作/表情列表。

### 阶段二：UI 组件（预计 4 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| `ActionMappingPanel.vue` 面板骨架 | 阶段一 | 面板容器 |
| `MappingRow.vue` 单行配置组件 | 面板 | 10 行卡片 |
| `MotionSelector.vue` 级联选择器 | 行组件 | 动作选择 UI |
| `ExpressionSelector.vue` 下拉选择器 | 行组件 | 表情选择 UI |
| `EffectSelector.vue` 下拉选择器 | 行组件 | 特效选择 UI |
| 面板工具栏（保存/重置/返回）| 面板 | 操作按钮 |
| 校验逻辑 | 服务层 | 前端校验 |

**验证点**：完整的配置 UI 可操作，保存/重置功能正常。

### 阶段三：预览与集成（预计 2 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 预览功能（单行预览按钮）| 阶段二 + Avatar 接口 | 实时预览 |
| 预览区域（面板顶部）| 阶段二 | 模型预览窗口 |
| 事件触发集成 | 阶段一 + 事件系统 | `TriggerHandler` |
| 日常随机切换逻辑 | 事件系统 | `daily-tick` 定时器 |
| 与设置面板集成 | 阶段二 | 入口按钮 + 面板切换 |

**验证点**：配置保存后，桌面宠物在对应事件触发时正确执行配置的动作/表情/特效。

### 阶段四：打磨（预计 1 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 动效细节 | 阶段三 | hover/过渡动画 |
| 空状态引导 | 阶段三 | 新模型无配置时的提示 |
| 边界测试 | 阶段三 | 模型无可用动作/表情时的降级 |

**总计：约 9 个工作日**

---

## 9. 风险与约束 (Risks & Constraints)

| # | 风险 | 影响 | 缓解措施 |
|---|------|------|----------|
| R1 | Live2D 模型动作分组命名不统一 | 下拉列表混乱 | 提供分组排序和显示名映射 |
| R2 | 某些模型无表情文件 | 表情选择器为空 | 禁用表情配置，提示「该模型不支持表情」|
| R3 | SpriteSheet 模型动作粒度粗 | 表现力不足 | 文档建议模型提供足够的动作帧 |
| R4 | 预览与运行时表现不一致 | 用户困惑 | 预览使用同一套 Avatar 接口，保证一致性 |
| R5 | 事件触发过于频繁（如 daily-tick）| 用户烦躁 | 可配置触发间隔；默认 5 分钟一次 |
| R6 | 特效系统尚未实现 | 特效配置无法预览 | 特效作为 Phase 2 功能，Phase 1 先实现空操作 |

### 约束

- **配置粒度**：映射到模型预定义的动作和表情，不支持自定义动画参数
- **特效**：本阶段特效为系统预定义列表，不支持用户自定义特效
- **数据库**：使用 `UNIQUE(model_id, trigger_key)` 确保每个场景只有一条配置
- **前端**：Vue 3 Composition API + `<script setup>`
- **复用**：动作和表情可在不同场景间自由复用，无需复制
- **必填规则**：仅 `daily_1` 有必填约束，其余 9 个场景完全可选
