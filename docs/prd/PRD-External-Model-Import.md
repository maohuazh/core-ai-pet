# PRD: 外部模型导入 (External Model Import)

> 版本: 1.0
> 日期: 2026-06-20
> 状态: 草案

---

## 1. 概述 (Overview)

为 CoreAIpet 实现**用户自定义模型导入**功能，支持导入 Live2D（`.model3.json`）和 SpriteSheet（`manifest.json`）两种格式的模型。用户通过文件选择对话框选取本地模型目录，应用自动校验、复制到应用数据目录、注册到 SQLite 数据库，并在设置面板的模型列表中显示。

本功能依赖 Tauri 的文件系统插件（`@tauri-apps/plugin-fs`）和对话框插件（`@tauri-apps/plugin-dialog`），遵循现有 `ModelRegistry`（`src/core/model/ModelRegistry.ts`）的模型管理模式。

---

## 2. 目标 (Goals)

| # | 目标 | 验证标准 |
|---|------|----------|
| G1 | 用户可选择本地文件夹导入 Live2D 模型 | 选择含 `.model3.json` 的文件夹后，模型出现在列表中 |
| G2 | 用户可选择本地文件夹导入 SpriteSheet 模型 | 选择含 `manifest.json` 的文件夹后，模型出现在列表中 |
| G3 | 导入时自动校验模型文件完整性 | 不合法模型被拒绝，显示错误信息 |
| G4 | 模型文件复制到应用数据目录 | 原文件删除后模型仍可使用 |
| G5 | 模型信息持久化到 SQLite | 重启后模型列表保留 |
| G6 | 支持模型元数据（作者、版本等）| 列表显示元数据信息 |
| G7 | 支持删除已导入的自定义模型 | 删除后文件清理 + 数据库移除 |

---

## 3. 非目标 (Non-Goals)

| # | 非目标 | 原因 |
|---|--------|------|
| NG1 | 不实现从网络 URL 下载模型 | 仅支持本地文件导入 |
| NG2 | 不实现模型在线商店/市场 | 后续独立功能 |
| NG3 | 不实现模型编辑/预览功能 | 导入即使用，不满意可删除 |
| NG4 | 不实现 ZIP/RAR 压缩包导入 | 仅支持已解压的文件夹 |
| NG5 | 不实现模型加密/DRM | 用户自行负责模型版权 |

---

## 4. 功能需求 (Functional Requirements)

### 4.1 导入流程

```
用户点击「+ 导入」按钮
  → 打开文件夹选择对话框（Tauri dialog plugin）
  → 用户选择模型文件夹
  → 扫描文件夹内容，识别模型类型
  → 校验模型文件完整性
  → 显示模型预览（名称、类型、缩略图）
  → 用户确认导入
  → 复制文件到应用数据目录
  → 写入 SQLite 数据库
  → 刷新模型列表
```

### 4.2 模型类型自动识别

```typescript
type DetectedModelType = "live2d" | "sprite" | "unknown";

interface DetectedModel {
  type: DetectedModelType;
  entryFile: string;         // 入口文件相对路径
  name: string;              // 从元数据或文件名提取
  directory: string;         // 源文件夹绝对路径
}

function detectModelType(files: string[]): DetectedModel {
  // 优先级：
  // 1. 查找 *.model3.json → Live2D
  // 2. 查找 manifest.json → SpriteSheet
  // 3. 返回 "unknown"

  const model3 = files.find(f => f.endsWith(".model3.json"));
  if (model3) {
    return {
      type: "live2d",
      entryFile: model3,
      name: extractModelName(model3),
      directory: currentDir
    };
  }

  const manifest = files.find(f => f === "manifest.json");
  if (manifest) {
    const manifestData = JSON.parse(readFileSync(manifest));
    return {
      type: "sprite",
      entryFile: "manifest.json",
      name: manifestData.meta?.name || extractModelName(currentDir),
      directory: currentDir
    };
  }

  return { type: "unknown", entryFile: "", name: "", directory: currentDir };
}
```

### 4.3 模型校验规则

#### 4.3.1 Live2D 模型校验

| 检查项 | 规则 | 错误消息 |
|--------|------|----------|
| 入口文件 | 必须存在 `*.model3.json` | "未找到 Live2D 模型文件 (.model3.json)" |
| JSON 格式 | 入口文件必须是有效 JSON | "模型文件格式错误" |
| 版本兼容 | FileReferences 中的 .moc3 文件存在 | "缺少必要的 .moc3 文件" |
| 纹理文件 | Textures 数组引用的图片文件存在 | "缺少纹理文件: {filename}" |
| 物理文件 | Physics3.json（可选）若引用则必须存在 | "物理文件缺失: {filename}" |
| 目录结构 | 所有引用文件的相对路径不超过上级目录 | "模型文件路径不合法" |

#### 4.3.2 SpriteSheet 模型校验

| 检查项 | 规则 | 错误消息 |
|--------|------|----------|
| 入口文件 | 必须存在 `manifest.json` | "未找到精灵表配置 (manifest.json)" |
| JSON Schema | manifest 通过 schema 校验 | "manifest.json 格式错误: {detail}" |
| 精灵表图片 | `spritesheet.image` 引用的文件存在 | "缺少精灵表图片: {filename}" |
| 图片尺寸 | 图片实际尺寸 ≥ columns×frameWidth, rows×frameHeight | "精灵表图片尺寸不匹配" |
| 表情文件 | expressions 中引用的 overlay 文件存在 | "缺少表情图片: {filename}" |
| 缩略图 | `meta.thumbnail`（可选）若引用则必须存在 | "缩略图文件缺失" |

#### 4.3.3 通用校验

| 检查项 | 规则 | 错误消息 |
|--------|------|----------|
| 目录大小 | 总大小 ≤ 200MB | "模型目录过大 ({size}MB)，上限 200MB" |
| 文件数量 | 文件数 ≤ 500 | "文件数量过多 ({count})，上限 500" |
| 重复导入 | 同 name + type 的模型已存在 | "模型 '{name}' 已存在，是否覆盖？" |
| 文件名安全 | 不含 `..`、`/`、`\` 等路径穿越字符 | "文件名包含非法字符" |

### 4.4 文件复制策略

```typescript
interface ImportPaths {
  sourceDir: string;          // 用户选择的源目录
  targetDir: string;          // 应用数据目录下的目标路径
  modelId: string;            // 生成的唯一 ID
}

function computeTargetPath(modelId: string, modelType: string): string {
  // 应用数据目录结构：
  // Windows: %APPDATA%/core-ai-pet/models/custom/{modelId}/
  // macOS:   ~/Library/Application Support/core-ai-pet/models/custom/{modelId}/
  // Linux:   ~/.local/share/core-ai-pet/models/custom/{modelId}/
  return path.join(getAppDataDir(), "models", "custom", modelId);
}

async function copyModelFiles(source: string, target: string): Promise<void> {
  // 使用 Tauri fs plugin 递归复制
  // 保留目录结构
  // 跳过隐藏文件（.开头的文件）和系统文件（Thumbs.db 等）
  await invoke("copy_directory", { source, target });
}
```

### 4.5 模型 ID 生成

```typescript
function generateModelId(name: string, type: string): string {
  // 格式: {type}-{sanitized-name}-{timestamp}
  // 示例: live2d-hiyori-20260620, sprite-pixelcat-20260620
  const sanitized = name.toLowerCase()
    .replace(/[^a-z0-9一-鿿]/g, "-")  // 保留中文
    .replace(/-+/g, "-")
    .replace(/^-|-$/g, "")
    .substring(0, 32);
  const timestamp = new Date().toISOString().slice(0, 10).replace(/-/g, "");
  return `${type}-${sanitized}-${timestamp}`;
}
```

### 4.6 删除模型

```typescript
interface DeleteModelOptions {
  modelId: string;
  deleteFiles: boolean;    // 是否同时删除文件（默认 true）
}

async function deleteModel(options: DeleteModelOptions): Promise<void> {
  const model = await modelRegistry.getModel(options.modelId);

  // 安全检查：不允许删除内置模型
  if (model.source === "builtin") {
    throw new Error("不允许删除内置模型");
  }

  // 如果当前活跃，先切换到默认模型
  if (petStore.activeModelId === options.modelId) {
    await petStore.setActiveModel(getDefaultModelId());
  }

  // 删除数据库记录
  await modelRegistry.removeModel(options.modelId);

  // 删除动作映射
  await db.deleteActionMappings(options.modelId);

  // 删除文件
  if (options.deleteFiles) {
    const modelPath = resolveModelPath(model.path);
    await invoke("remove_directory", { path: modelPath });
  }
}
```

### 4.7 模型元数据提取

```typescript
interface ModelMetadata {
  name: string;
  author?: string;
  version?: string;
  description?: string;
  license?: string;
  thumbnail?: string;       // 缩略图文件的相对路径
}

// Live2D 元数据从 .model3.json 提取
function extractLive2DMetadata(model3Path: string): ModelMetadata {
  const model3 = JSON.parse(readFileSync(model3Path));
  return {
    name: path.basename(model3Path, ".model3.json"),
    // Live2D .model3.json 标准格式无 author/version 字段
    // 尝试从自定义字段读取
    author: model3.CustomData?.Author,
    version: model3.Version,
    description: model3.CustomData?.Description,
    thumbnail: model3.CustomData?.Thumbnail,
  };
}

// SpriteSheet 元数据从 manifest.json 提取
function extractSpriteMetadata(manifestPath: string): ModelMetadata {
  const manifest = JSON.parse(readFileSync(manifestPath));
  return {
    name: manifest.meta?.name || "Unnamed",
    author: manifest.meta?.author,
    version: manifest.meta?.version,
    description: manifest.meta?.description,
    license: manifest.meta?.license,
    thumbnail: manifest.meta?.thumbnail,
  };
}
```

---

## 5. 技术设计 (Technical Design)

### 5.1 Tauri 插件依赖

```toml
# src-tauri/Cargo.toml

[dependencies]
tauri-plugin-fs = "2"       # 文件系统操作
tauri-plugin-dialog = "2"   # 文件选择对话框
```

```json
// src-tauri/tauri.conf.json (plugins 配置)
{
  "plugins": {
    "fs": {
      "scope": {
        "allow": ["**"],
        "deny": []
      }
    },
    "dialog": {
      "all": true
    }
  }
}
```

### 5.2 Rust 后端命令

```rust
// src-tauri/src/model_import.rs

use std::path::PathBuf;
use tauri::command;

/// 扫描目录，检测模型类型
#[command]
async fn detect_model_type(directory: String) -> Result<DetectedModelInfo, String> {
    let dir = PathBuf::from(&directory);

    if !dir.is_dir() {
        return Err("所选路径不是文件夹".to_string());
    }

    // 列出所有文件
    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("无法读取目录: {}", e))?;

    let files: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    // 检测类型
    if let Some(f) = files.iter().find(|f| f.ends_with(".model3.json")) {
        Ok(DetectedModelInfo {
            model_type: "live2d".to_string(),
            entry_file: f.clone(),
            name: f.trim_end_matches(".model3.json").to_string(),
        })
    } else if files.contains(&"manifest.json".to_string()) {
        // 读取 manifest 获取名称
        let manifest_path = dir.join("manifest.json");
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("无法读取 manifest.json: {}", e))?;
        let manifest: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("manifest.json 解析失败: {}", e))?;
        let name = manifest["meta"]["name"]
            .as_str()
            .unwrap_or("Unnamed")
            .to_string();
        Ok(DetectedModelInfo {
            model_type: "sprite".to_string(),
            entry_file: "manifest.json".to_string(),
            name,
        })
    } else {
        Err("未识别的模型格式。请确保文件夹包含 .model3.json 或 manifest.json".to_string())
    }
}

/// 校验模型文件完整性
#[command]
async fn validate_model(directory: String, model_type: String) -> Result<ValidationResult, String> {
    // 实现 4.3 节定义的校验规则
    // 返回校验结果 + 错误列表
    match model_type.as_str() {
        "live2d" => validate_live2d_model(&directory),
        "sprite" => validate_sprite_model(&directory),
        _ => Err("未知模型类型".to_string()),
    }
}

/// 复制模型到应用数据目录
#[command]
async fn import_model(
    source_dir: String,
    model_id: String,
    model_type: String,
    entry_file: String,
    db: tauri::State<'_, Database>,
) -> Result<ModelRecord, String> {
    let app_data = tauri::api::path::app_data_dir(&tauri::Config::default())
        .ok_or("无法获取应用数据目录")?;
    let target_dir = app_data.join("models").join("custom").join(&model_id);

    // 递归复制
    copy_dir_recursive(&source_dir, &target_dir)?;

    // 写入数据库
    let record = ModelRecord {
        id: model_id,
        name: /* 从参数或文件提取 */,
        model_type,
        path: target_dir.to_string_lossy().to_string(),
        entry_file,
        source: "custom".to_string(),
        status: "active".to_string(),
        // ...
    };
    db.insert_model(&record)?;

    Ok(record)
}

/// 删除已导入的模型
#[command]
async fn delete_imported_model(
    model_id: String,
    delete_files: bool,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    // 获取模型信息
    let model = db.get_model(&model_id)?;

    // 安全检查
    if model.source == "builtin" {
        return Err("不允许删除内置模型".to_string());
    }

    // 删除文件
    if delete_files {
        let path = PathBuf::from(&model.path);
        if path.exists() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| format!("删除文件失败: {}", e))?;
        }
    }

    // 删除数据库记录（CASCADE 会同时删除 action_mappings）
    db.delete_model(&model_id)?;

    Ok(())
}

// ---- 辅助数据结构 ----

#[derive(serde::Serialize)]
struct DetectedModelInfo {
    model_type: String,
    entry_file: String,
    name: String,
}

#[derive(serde::Serialize)]
struct ValidationResult {
    valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
    metadata: Option<ModelMetadata>,
}

#[derive(serde::Serialize)]
struct ModelMetadata {
    name: String,
    author: Option<String>,
    version: Option<String>,
    description: Option<String>,
    thumbnail: Option<String>,
}

#[derive(serde::Serialize)]
struct ModelRecord {
    id: String,
    name: String,
    model_type: String,
    path: String,
    entry_file: String,
    source: String,
    status: String,
    author: Option<String>,
    version: Option<String>,
    description: Option<String>,
    thumbnail: Option<String>,
}
```

### 5.3 前端导入服务

```typescript
// src/core/model/modelImportService.ts

import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

interface ImportResult {
  success: boolean;
  modelId?: string;
  error?: string;
}

class ModelImportService {
  /**
   * 启动导入流程（从 UI 调用）
   */
  async startImport(): Promise<ImportResult> {
    // 1. 打开文件夹选择对话框
    const selected = await open({
      directory: true,
      multiple: false,
      title: "选择模型文件夹",
    });

    if (!selected) {
      return { success: false, error: "用户取消选择" };
    }

    const directory = selected as string;

    try {
      // 2. 检测模型类型
      const detected = await invoke<DetectedModelInfo>("detect_model_type", {
        directory,
      });

      // 3. 校验模型
      const validation = await invoke<ValidationResult>("validate_model", {
        directory,
        modelType: detected.model_type,
      });

      if (!validation.valid) {
        return {
          success: false,
          error: `模型校验失败:\n${validation.errors.join("\n")}`,
        };
      }

      // 4. 生成模型 ID
      const modelId = generateModelId(detected.name, detected.model_type);

      // 5. 检查重复
      const existing = await modelRegistry.getModelByName(detected.name);
      if (existing) {
        const overwrite = await showConfirmDialog(
          `模型 "${detected.name}" 已存在，是否覆盖？`,
          "覆盖将替换原有模型及其配置。"
        );
        if (!overwrite) {
          return { success: false, error: "用户取消导入" };
        }
        // 删除旧模型
        await invoke("delete_imported_model", {
          modelId: existing.id,
          deleteFiles: true,
        });
      }

      // 6. 执行导入（复制文件 + 写入数据库）
      const record = await invoke<ModelRecord>("import_model", {
        sourceDir: directory,
        modelId,
        modelType: detected.model_type,
        entryFile: detected.entry_file,
      });

      // 7. 初始化默认动作映射
      await this.initDefaultActionMappings(modelId, detected.model_type);

      return { success: true, modelId };
    } catch (err) {
      return { success: false, error: String(err) };
    }
  }

  /**
   * 为新导入的模型初始化默认动作映射
   */
  private async initDefaultActionMappings(
    modelId: string,
    modelType: string
  ): Promise<void> {
    const triggers = [
      "daily_1", "daily_2", "daily_3",
      "new_message", "new_task", "new_email",
      "task_in_progress", "task_completed",
      "task_approaching_deadline", "task_overdue",
    ];

    for (const trigger of triggers) {
      await invoke("upsert_action_mapping", {
        modelId,
        triggerKey: trigger,
        motionGroup: null,
        motionName: null,
        expressionName: null,
        effectName: null,
        useDefault: trigger === "daily_1" ? 1 : 0,  // daily_1 默认使用模型默认值
      });
    }
  }
}

export const modelImportService = new ModelImportService();
```

### 5.4 应用数据目录结构

```
{AppDataDir}/
├── core-ai-pet.db                    ← SQLite 数据库
├── models/
│   ├── builtin/                      ← 内置模型（只读）
│   │   ├── Hiyori/
│   │   │   ├── Hiyori.model3.json
│   │   │   ├── Hiyori.moc3
│   │   │   └── ...
│   │   ├── Mao/
│   │   └── Natori/
│   └── custom/                       ← 用户导入的模型
│       ├── sprite-pixelcat-20260620/
│       │   ├── manifest.json
│       │   ├── spritesheet.png
│       │   ├── thumbnail.png
│       │   └── expressions/
│       │       └── happy.png
│       └── live2d-mychara-20260621/
│           ├── mychara.model3.json
│           ├── mychara.moc3
│           └── ...
└── cache/                            ← 缓存（可清理）
    └── thumbnails/
```

### 5.5 安全模型

| 安全措施 | 实现方式 |
|----------|----------|
| 路径穿越防护 | 校验所有文件路径不含 `..`；复制后验证目标在 AppDataDir 下 |
| 文件大小限制 | 单文件 ≤ 50MB；总目录 ≤ 200MB |
| 文件类型白名单 | 仅允许：`.json`, `.png`, `.jpg`, `.moc3`, `.model3.json`, `.physics3.json`, `.pose3.json`, `.cdi3.json` |
| 符号链接检测 | 拒绝包含符号链接的目录 |
| 文件名消毒 | 复制时清理文件名中的特殊字符 |
| 权限隔离 | 自定义模型目录设为应用可写，其他位置只读 |

---

## 6. 数据库设计 (Database Schema)

完整的 `models` 表定义参见 [PRD-Settings-Panel.md](./PRD-Settings-Panel.md#6-数据库设计-database-schema)。

此处补充导入相关的操作 SQL：

### 6.1 插入模型

```sql
INSERT INTO models (
    id, name, type, path, manifest_path, model3_path,
    thumbnail, source, status,
    author, version, description, license,
    sort_order
) VALUES (
    :id, :name, :type, :path, :manifest_path, :model3_path,
    :thumbnail, 'custom', 'active',
    :author, :version, :description, :license,
    :sort_order
);
```

### 6.2 查询所有模型（设置面板用）

```sql
SELECT
    id, name, type, path, thumbnail, source, status,
    author, version, description,
    CASE WHEN status = 'active' THEN 1 ELSE 0 END as is_active,
    sort_order
FROM models
ORDER BY sort_order ASC, created_at ASC;
```

### 6.3 查询自定义模型（管理用）

```sql
SELECT *
FROM models
WHERE source = 'custom'
ORDER BY created_at DESC;
```

### 6.4 删除模型（级联）

```sql
-- 由于 model_action_mappings 定义了 ON DELETE CASCADE
-- 删除 models 记录会自动删除关联的映射
PRAGMA foreign_keys = ON;  -- 确保启用外键约束

DELETE FROM models WHERE id = :model_id AND source != 'builtin';
```

### 6.5 更新模型状态

```sql
-- 停用所有模型
UPDATE models SET status = 'inactive';

-- 激活指定模型
UPDATE models SET status = 'active', updated_at = datetime('now')
WHERE id = :model_id;
```

### 6.6 更新排序

```sql
UPDATE models SET sort_order = :order, updated_at = datetime('now')
WHERE id = :model_id;
```

---

## 7. UI/UX 设计

### 7.1 导入对话框流程

```
┌─────────────────────────────────────────────────┐
│           系统文件夹选择对话框                    │
│                                                 │
│  📁 选择模型文件夹                    [选择]    │
│                                                 │
│  当前位置: > 用户 > Documents > Models          │
│                                                 │
│  ┌──────────────────────────────────────────┐   │
│  │  📁 PixelCat/                            │   │
│  │  📁 MyLive2DChara/                       │   │
│  │  📄 readme.txt                           │   │
│  └──────────────────────────────────────────┘   │
│                                                 │
│                          [取消]  [选择文件夹]    │
└─────────────────────────────────────────────────┘
```

### 7.2 导入预览确认

选择文件夹后，弹出预览确认卡片：

```
┌──────────────────────────────────────────────────────┐
│  📦 导入模型                                         │
├──────────────────────────────────────────────────────┤
│                                                      │
│  ┌────────┐                                          │
│  │  [缩略 │  名称: PixelCat                           │
│  │   图]  │  类型: 🖼️ SpriteSheet                    │
│  │        │  作者: ArtistName                         │
│  └────────┘  版本: 1.0.0                             │
│                                                      │
│  ─── 校验结果 ───                                    │
│  ✅ 模型文件完整                                      │
│  ✅ 精灵表图片尺寸正确                                │
│  ⚠️ 未提供缩略图（将使用默认图标）                    │
│                                                      │
│  ─── 存储信息 ───                                    │
│  模型大小: 2.3 MB                                    │
│  目标位置: AppData/models/custom/sprite-pixelcat-... │
│                                                      │
│                        [取消]    [确认导入]           │
└──────────────────────────────────────────────────────┘
```

### 7.3 导入进度

```
┌──────────────────────────────────────────────────────┐
│  📦 正在导入...                                      │
├──────────────────────────────────────────────────────┤
│                                                      │
│  PixelCat                                            │
│  [████████████████████░░░░░░░░░░░] 65%              │
│  正在复制文件...                                     │
│                                                      │
└──────────────────────────────────────────────────────┘
```

### 7.4 导入成功

```
┌──────────────────────────────────────────────────────┐
│  ✅ 导入成功                                         │
├──────────────────────────────────────────────────────┤
│                                                      │
│  模型 "PixelCat" 已成功导入！                        │
│                                                      │
│  [切换到该模型]    [留在当前模型]    [关闭]          │
│                                                      │
└──────────────────────────────────────────────────────┘
```

### 7.5 导入失败

```
┌──────────────────────────────────────────────────────┐
│  ❌ 导入失败                                         │
├──────────────────────────────────────────────────────┤
│                                                      │
│  模型校验失败:                                       │
│  • 缺少纹理文件: textures/texture_00.png            │
│  • .moc3 文件不存在                                 │
│                                                      │
│  请确认选择的文件夹包含完整的模型文件。              │
│                                                      │
│                                      [关闭]         │
└──────────────────────────────────────────────────────┘
```

### 7.6 用户流程图

```
                    ┌─────────────┐
                    │  点击导入   │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │ 选择文件夹  │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
              ┌─────│ 检测类型    │
              │     └──────┬──────┘
              │            │
         未识别      ┌─────▼──────┐
              │     │ 校验完整性  │
              │     └─────┬──────┘
              │           │
              │     ┌─────▼──────┐
              │     │ 校验通过？  │
              │     └──┬─────┬───┘
              │    否  │     │ 是
              │        │     │
              │  ┌─────▼──┐  │
              │  │显示错误 │  │
              │  └────────┘  │
              │              │
              │       ┌──────▼──────┐
              │       │ 检查重复？  │
              │       └──┬─────┬───┘
              │      是  │     │ 否
              │          │     │
              │   ┌──────▼──┐  │
              │   │确认覆盖？│  │
              │   └──┬───┬──┘  │
              │   否 │   │ 是  │
              │      │   │     │
              │  取消 │   └─────┤
              │      │         │
              │      │  ┌──────▼──────┐
              │      │  │ 复制文件    │
              │      │  └──────┬──────┘
              │      │         │
              │      │  ┌──────▼──────┐
              │      │  │ 写入数据库  │
              │      │  └──────┬──────┘
              │      │         │
              │      │  ┌──────▼──────┐
              │      └──│ 初始化映射  │
              │        │ └──────┬──────┘
              │        │        │
              │        │ ┌──────▼──────┐
              │        │ │ 导入成功！  │
              │        │ └─────────────┘
```

---

## 8. 实现计划 (Implementation Plan)

### 阶段一：后端基础（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 配置 Tauri fs + dialog 插件 | 无 | Cargo.toml + tauri.conf.json |
| 实现 `detect_model_type` 命令 | 无 | 类型检测 |
| 实现 `validate_model` 命令（Live2D）| 检测 | Live2D 校验 |
| 实现 `validate_model` 命令（Sprite）| 检测 | Sprite 校验 |
| 实现目录复制辅助函数 | 无 | `copy_dir_recursive` |
| 安全校验（路径穿越/文件类型/大小）| 复制函数 | 安全检查层 |

**验证点**：能检测到测试用 Live2D/SpriteSheet 文件夹的类型和校验通过。

### 阶段二：导入流程（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 实现 `import_model` 命令 | 阶段一 | 文件复制 + DB 写入 |
| 实现 `delete_imported_model` 命令 | 无 | 删除 + 清理 |
| 前端 `ModelImportService` | 后端命令 | 导入服务层 |
| 前端导入 UI（预览/进度/结果对话框）| 服务层 | 对话框组件 |
| 集成到设置面板「+ 导入」按钮 | UI | 端到端流程 |

**验证点**：完整导入流程从选择文件夹到模型出现在列表。

### 阶段三：删除与错误处理（预计 2 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 模型删除 UI（确认对话框）| 阶段二 | 删除流程 |
| 文件清理验证 | 阶段二 | 磁盘空间回收 |
| 错误场景覆盖（中断恢复/磁盘满等）| 阶段二 | 健壮性 |
| 重复导入处理 | 阶段二 | 覆盖逻辑 |

**验证点**：删除后文件与数据库均清理；重复导入正确提示。

### 阶段四：打磨（预计 1 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 导入动效（进度条/过渡）| 阶段三 | UI 体验 |
| 边界测试（超大目录/特殊字符名）| 阶段三 | 稳定性 |
| 日志与错误上报 | 阶段三 | 可观测性 |

**总计：约 9 个工作日**

---

## 9. 风险与约束 (Risks & Constraints)

| # | 风险 | 影响 | 缓解措施 |
|---|------|------|----------|
| R1 | Live2D 模型格式版本差异（Cubism 2/3/4/5）| 导入的模型无法渲染 | 仅支持 Cubism 4+（.model3.json）；文档说明 |
| R2 | 大模型导入耗时过长 | 用户以为卡死 | 进度回调 + 可取消 |
| R3 | 磁盘空间不足 | 导入失败 | 复制前检查可用空间 |
| R4 | 恶意文件（伪装为模型的可执行文件）| 安全风险 | 文件类型白名单；不执行任何导入的文件 |
| R5 | 跨平台路径差异（Windows/macOS/Linux）| 路径解析错误 | 使用 Tauri 的 `path` API 统一处理 |
| R6 | 导入中断（用户强制关闭）导致半成品 | 数据库与文件不一致 | 事务性导入：先复制到临时目录，完成后原子移动 |

### 约束

- **Tauri 插件**：必须使用 `tauri-plugin-fs` 和 `tauri-plugin-dialog`（不使用 Web File API）
- **存储位置**：模型文件必须复制到应用数据目录（不直接引用用户目录）
- **文件格式**：Live2D 仅支持 Cubism 4+（`.model3.json`）；SpriteSheet 遵循 manifest.json 规范
- **大小限制**：单模型目录 ≤ 200MB
- **安全性**：禁止路径穿越、禁止执行文件、文件类型白名单
- **数据库**：外键约束必须启用（`PRAGMA foreign_keys = ON`）
