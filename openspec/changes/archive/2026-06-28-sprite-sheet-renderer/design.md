## Context

CoreAIpet 当前采用 `Avatar + Renderer` 二层架构，仅支持 Live2D 渲染管线。`IAvatar` 接口定义在 `src/core/avatar/types.ts`，`IRenderer` 接口定义在 `src/core/renderer/live2d/Live2DRenderer.ts`。`AvatarFactory` 根据 `AvatarType` 分发创建具体 Avatar 实例。

PRD 要求新增 SpriteSheet 帧动画渲染器作为第二种渲染管线，与 Live2D 共存，通过工厂模式零侵入扩展。

## Goals / Non-Goals

**Goals:**

- 实现 SpriteSheet 帧动画渲染器，支持 8 方向精灵、状态驱动动画切换
- 遵循现有 `Avatar + Renderer` 架构，通过 `IAvatar`/`IRenderer` 接口集成
- 定义 `manifest.json` 规范，支持精灵表配置、状态映射、表情覆盖层
- 扩展 `AvatarFactory` 和 `ModelRegistry` 支持 sprite 类型
- 数据库 `models` 表增加 `type` 和 `manifest_path` 字段

**Non-Goals:**

- 不实现 Spine/DragonBones 骨骼动画（后续独立 PRD）
- 不实现精灵表编辑器（使用外部工具如 Aseprite）
- 不修改或替换 Live2D 渲染器
- 不实现 3D 模型渲染
- 不实现物理模拟（逐帧动画天然不含物理）

## Decisions

### Decision 1: 渲染后端选择 HTML Canvas 2D

**选择**: 使用浏览器原生 Canvas 2D API，而非引入 PixiJS 或 WebGL。

**理由**: 
- 精灵表渲染逻辑简单（`drawImage` 裁剪 + 缩放），无需复杂渲染管线
- Canvas 2D 内存占用低，适合小型逐帧动画
- 与 Tauri webview 兼容性最佳，无需额外依赖

**替代方案**: 
- PixiJS：功能强大但引入额外依赖，SpriteSheet 场景下性能优势不明显
- WebGL：实现复杂度高，Canvas 2D 已满足需求

### Decision 2: 帧调度引擎独立于 Renderer

**选择**: 将帧调度逻辑抽取为独立的 `SpriteSheetAnimationEngine`，不耦合 Canvas 渲染。

**理由**: 
- 引擎纯逻辑层，可脱离 Canvas 独立测试（单元测试友好）
- 未来可复用于其他渲染后端（如 WebGL）
- 职责分离：引擎负责"播什么帧"，渲染器负责"怎么画"

### Decision 3: manifest.json 采用 Zod 校验

**选择**: 使用 Zod 库进行 manifest.json 的运行时校验。

**理由**: 
- 项目已使用 TypeScript，Zod 提供类型安全的校验结果
- 错误信息清晰，便于用户调试 manifest 格式问题
- 轻量级，无运行时性能开销

**替代方案**: 
- 手动校验：代码冗长，易出错
- JSON Schema：需要额外依赖，校验逻辑不如 Zod 灵活

### Decision 4: 8 方向采用角度分区映射

**选择**: 将 0-360° 分为 8 个 45° 扇区，每个扇区对应一个方向（N/NE/E/SE/S/SW/W/NW）。

**理由**: 
- 与现有 `setDirection(degree: number)` 接口一致
- 角度计算简单（`Math.round(degree / 45) % 8`），性能开销低
- 支持任意角度输入，自动吸附到最近方向

### Decision 5: 表情覆盖层采用 PNG 叠加

**选择**: 通过 `globalCompositeOperation = "source-over"` 将表情 PNG 叠加到主帧之上。

**理由**: 
- 实现简单，无需修改主精灵表
- 表情可独立制作，灵活度高
- 支持透明通道，与背景融合良好

**替代方案**: 
- 将表情帧嵌入主精灵表：增加精灵表体积，制作复杂
- WebGL shader：实现复杂，Canvas 2D 已满足需求

### Decision 6: 数据库 migration 采用 ALTER TABLE

**选择**: 通过 SQLite `ALTER TABLE` 语句为 `models` 表增加 `type` 和 `manifest_path` 列。

**理由**: 
- SQLite 支持 ALTER TABLE ADD COLUMN，无需重建表
- 向后兼容：已有 Live2D 模型的 `type` 默认为 `'live2d'`
- Migration 脚本在应用启动时自动执行

## Risks / Trade-offs

### Risk 1: 大型精灵表性能问题

**风险**: 高分辨率精灵表（>4096×4096）在部分 GPU 上渲染性能差。

**缓解**: 
- 文档建议最大尺寸 2048×2048
- 支持分片精灵表（多文件）作为可选扩展
- 提供性能监控日志，便于排查问题

### Risk 2: manifest.json 格式设计不当

**风险**: 后续扩展时可能引入破坏性变更。

**缓解**: 
- manifest 包含 `version` 字段，支持向后兼容解析
- 使用 Zod 校验，版本升级时可提供清晰错误提示
- 文档化规范，便于第三方工具生成

### Risk 3: 表情覆盖层与主帧尺寸不匹配

**风险**: 表情 PNG 与主帧尺寸不一致导致视觉错位。

**缓解**: 
- 强制覆盖层与主帧同尺寸（manifest 中声明）
- 或在 manifest 中允许 `offset` 字段声明偏移量
- 渲染时自动缩放到主帧尺寸

### Risk 4: 8 方向素材制作门槛高

**风险**: 用户难以制作 8 方向精灵，导致模型数量少。

**缓解**: 
- 支持可选的 4 方向 / 无方向模式（manifest 中 `directions.enabled: false`）
- 提供示例精灵表与 manifest 模板
- 文档化制作流程与工具推荐（如 Aseprite）

### Trade-off: Canvas 2D vs PixiJS

**权衡**: Canvas 2D 实现简单但功能有限，PixiJS 功能强大但引入依赖。

**决策**: 选择 Canvas 2D，因 SpriteSheet 渲染逻辑简单，无需复杂渲染管线。未来如需高级特效（如粒子系统），可考虑引入 PixiJS 作为可选渲染后端。
