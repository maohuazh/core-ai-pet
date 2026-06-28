## ADDED Requirements

### Requirement: Model metadata structure
系统 SHALL 提供 `ModelInfo` 接口，包含以下字段：`id`(string)、`name`(string)、`type`("live2d" | "sprite")、`path`(string)、`manifestPath`(string, 可选)、`thumbnail`(string, 可选)、`source`("builtin" | "cdn" | "custom")、`status`("active" | "inactive")。

#### Scenario: Define a Live2D model config
- **WHEN** 开发者定义一个 Live2D 类型的 `ModelInfo` 对象
- **THEN** 对象 MUST 包含 `id`、`name`、`type: "live2d"`、`path`（模型文件路径） 四个必填字段
- **THEN** `manifestPath` 为可选字段（Live2D 不需要）

#### Scenario: Define a Sprite model config
- **WHEN** 开发者定义一个 Sprite 类型的 `ModelInfo` 对象
- **THEN** 对象 MUST 包含 `id`、`name`、`type: "sprite"`、`path`（模型目录路径）、`manifestPath`（manifest.json 相对路径） 为必填字段

#### Scenario: Model with thumbnail
- **WHEN** 开发者定义包含缩略图的模型
- **THEN** `thumbnail` 字段 MUST 为可选的 string 类型

#### Scenario: Model from different sources
- **WHEN** 开发者定义来自 builtin/cdn/custom 的模型
- **THEN** `source` 字段 MUST 为 "builtin" | "cdn" | "custom" 之一

### Requirement: Static model registry
系统 SHALL 提供 `ModelRegistry` 类，在应用启动时注册所有可用模型。注册表 MUST 提供 `getAll()` 返回全部模型列表、`getById(id)` 按 ID 查询单个模型、`register(config)` 注册新模型。

#### Scenario: Get all registered models
- **WHEN** 调用 `registry.getAll()`
- **THEN** 返回包含 Hiyori、Mao、Natori 至少 3 个已注册模型的数组

#### Scenario: Get model by ID
- **WHEN** 调用 `registry.getById("hiyori")`
- **THEN** 返回对应的 `PetModelConfig` 对象，`modelUrl` 指向 `./models/Hiyori/Hiyori.model3.json`

#### Scenario: Query non-existent model
- **WHEN** 调用 `registry.getById("nonexistent")`
- **THEN** 返回 `null`

### Requirement: Default model selection
注册表 SHALL 指定一个默认模型（`getDefault()`），应用首次启动时 MUST 使用该模型。

#### Scenario: Get default model
- **WHEN** 调用 `registry.getDefault()`
- **THEN** 返回一个有效的 `PetModelConfig` 对象，不为 null
