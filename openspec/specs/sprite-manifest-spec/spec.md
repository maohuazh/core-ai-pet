# sprite-manifest-spec Specification

## Purpose

定义精灵表模型的配置文件（`manifest.json`）格式、字段语义以及运行时校验规则，确保 sprite 类型模型可以被 `SpriteSheetAvatar` / `SpriteSheetAnimationEngine` 正确加载与解析。

## Requirements

### Requirement: manifest.json 文件结构
系统 SHALL 提供 `manifest.json` 文件作为精灵表模型的配置文件，定义帧布局、方向、状态与表情映射。

#### Scenario: manifest.json 基本字段
- **WHEN** 解析一个有效的 manifest.json
- **THEN** 文件 MUST 包含 `version`（string）字段
- **THEN** 文件 MUST 包含 `meta` 对象，含 `name`（string，必填）、`author`（string，可选）、`description`（string，可选）、`thumbnail`（string，可选）
- **THEN** 文件 MUST 包含 `spritesheet` 对象，含 `image`（string）、`frameWidth`（number, >0）、`frameHeight`（number, >0）、`columns`（number, >0）、`rows`（number, >0）
- **THEN** 文件 MUST 包含 `states` 对象，每个状态含 `frames.start`（number, >=0）、`frames.count`（number, >0）、`fps`（number, >0, <=60）、`loop`（boolean）
- **THEN** 文件 MUST 包含 `defaults` 对象，含 `state`（string）

#### Scenario: spritesheet padding 和 spacing
- **WHEN** manifest.json 中 `spritesheet.padding` 或 `spritesheet.spacing` 未提供
- **THEN** 系统 SHALL 默认值为 0

#### Scenario: directions 字段可选
- **WHEN** manifest.json 中 `directions` 字段不存在
- **THEN** 系统 SHALL 视为无方向精灵，方向切换功能 MUST 静默失败

#### Scenario: expressions 字段可选
- **WHEN** manifest.json 中 `expressions` 字段不存在
- **THEN** 系统 SHALL 返回空表情列表

### Requirement: manifest.json 校验
系统 SHALL 使用 Zod 对 manifest.json 进行运行时校验，确保格式正确。

#### Scenario: 校验通过
- **WHEN** 传入一个符合规范的 manifest 对象
- **THEN** 系统 SHALL 返回解析后的 TypeScript 对象（类型安全）

#### Scenario: 校验失败
- **WHEN** 传入一个不符合规范的 manifest 对象
- **THEN** 系统 SHALL 抛出 `Error`，消息包含 Zod 错误详情
- **THEN** 错误消息 MUST 指明具体哪个字段不合法

#### Scenario: 缺失必填字段
- **WHEN** manifest.json 缺少 `version`、`meta`、`spritesheet` 或 `states` 字段
- **THEN** 系统 SHALL 抛出 `Error`，消息指明缺失的字段名

#### Scenario: 非法数值
- **WHEN** manifest.json 中 `frameWidth` <= 0 或 `fps` > 60
- **THEN** 系统 SHALL 抛出 `Error`，消息指明非法的字段与值

### Requirement: manifest.json 版本兼容
manifest.json SHALL 通过 `version` 字段支持向后兼容解析。

#### Scenario: 版本字段存在
- **WHEN** 解析 manifest.json
- **THEN** 系统 SHALL 读取 `version` 字段（如 "1.0"）

#### Scenario: 未来版本升级
- **WHEN** manifest.json 的 `version` 大于当前支持版本
- **THEN** 系统 SHALL 尝试解析，未知字段 MUST 被忽略
- **THEN** 若缺失必填字段，MUST 抛出校验错误
