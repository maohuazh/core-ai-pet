## MODIFIED Requirements

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
