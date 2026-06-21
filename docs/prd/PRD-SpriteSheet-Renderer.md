# PRD: SpriteSheet 帧动画渲染器

> 版本: 1.0
> 日期: 2026-06-20
> 状态: 草案

---

## 1. 概述 (Overview)

CoreAIpet 当前仅支持 Live2D 渲染器（`Live2DRenderer` + `Live2DAvatar`）。本 PRD 定义一种全新的 **SpriteSheet 帧动画渲染器**，作为与 Live2D 并列的第二种渲染管线。

该渲染器消费传统的精灵表（Sprite Sheet）图片——一张包含所有动画帧的 PNG，配合 `manifest.json` 描述帧布局、方向与状态映射。目标是让用户无需 Live2D 建模工具，即可使用像素画、手绘逐帧动画等形式制作桌面宠物。

**核心定位**：遵循现有 `Avatar + Renderer` 二层架构（见 `src/core/avatar/types.ts` 与 `src/core/renderer/types.ts`），通过 `AvatarFactory`（`src/core/avatar/factory.ts`）按模型类型分发，实现零侵入式扩展。

---

## 2. 目标 (Goals)

| # | 目标 | 验证标准 |
|---|------|----------|
| G1 | 实现 `SpriteSheetRenderer`，在 HTML Canvas 上逐帧绘制精灵表 | 能在屏幕上看到动画播放，无闪烁/撕裂 |
| G2 | 实现 `SpriteSheetAvatar`，适配现有 `IAvatar` 接口 | `load()`, `playMotion()`, `setExpression()`, `setState()` 全部可用 |
| G3 | 支持 **8 方向**精灵（N/NE/E/SE/S/SW/W/NW）| 方向切换时平滑过渡，方向角度与帧索引映射正确 |
| G4 | 支持 **状态驱动动画切换**（Idle / Walking / Sleeping / Alert 等 8 种 PetState）| 状态变更事件触发后，动画在 1 帧内切换 |
| G5 | 定义 `manifest.json` 规范，兼容未来可能的新增状态 | 规范文档完整，可被第三方工具生成 |
| G6 | 通过 `AvatarFactory` 集成，与 Live2D 共存 | `factory.create({ type: "sprite", ... })` 返回 SpriteSheetAvatar 实例 |

---

## 3. 非目标 (Non-Goals)

| # | 非目标 | 原因 |
|---|--------|------|
| NG1 | 不实现 Spine/DragonBones 骨骼动画 | 超出帧动画范围，后续独立 PRD |
| NG2 | 不实现精灵表编辑器 | 使用外部工具（如 Aseprite）制作 |
| NG3 | 不替换或修改 Live2D 渲染器 | 二者并存，互不影响 |
| NG4 | 不实现 3D 模型渲染 | 仅 2D 帧动画 |
| NG5 | 不实现物理模拟（布料/头发摆动）| 逐帧动画天然不含物理 |

---

## 4. 功能需求 (Functional Requirements)

### 4.1 精灵表格式规范 (`manifest.json`)

```jsonc
{
  // 规范版本
  "version": "1.0",

  // 模型元数据
  "meta": {
    "name": "PixelCat",
    "author": "ArtistName",
    "description": "一只像素猫",
    "version": "1.0.0",
    "thumbnail": "thumbnail.png",       // 可选，缩略图
    "license": "CC-BY-4.0"             // 可选
  },

  // 精灵表图片配置
  "spritesheet": {
    "image": "spritesheet.png",         // 精灵表文件名
    "frameWidth": 64,                   // 单帧宽度 (px)
    "frameHeight": 64,                  // 单帧高度 (px)
    "columns": 8,                       // 精灵表列数
    "rows": 16,                         // 精灵表行数
    "padding": 0,                       // 帧间距 (px)
    "spacing": 0                        // 外边距 (px)
  },

  // 方向配置（可选；不提供则视为无方向）
  "directions": {
    "enabled": true,
    "count": 8,                         // 8 方向
    "mapping": {
      "N":  0,                          // 北 = 第 0 行
      "NE": 1,
      "E":  2,
      "SE": 3,
      "S":  4,
      "SW": 5,
      "W":  6,
      "NW": 7
    }
    // 每个方向的动画帧分布在对应行中
  },

  // 状态 → 动画 映射
  "states": {
    "Idle": {
      "frames": { "start": 0, "count": 4 },   // 帧范围（相对于当前方向行）
      "fps": 8,
      "loop": true
    },
    "Walking": {
      "frames": { "start": 4, "count": 8 },
      "fps": 12,
      "loop": true
    },
    "Thinking": {
      "frames": { "start": 12, "count": 4 },
      "fps": 6,
      "loop": true
    },
    "Talking": {
      "frames": { "start": 12, "count": 4 },
      "fps": 10,
      "loop": true
    },
    "Working": {
      "frames": { "start": 16, "count": 6 },
      "fps": 10,
      "loop": true
    },
    "Meeting": {
      "frames": { "start": 16, "count": 6 },
      "fps": 10,
      "loop": true
    },
    "Sleeping": {
      "frames": { "start": 22, "count": 4 },
      "fps": 4,
      "loop": true
    },
    "Alert": {
      "frames": { "start": 26, "count": 4 },
      "fps": 12,
      "loop": false                             // 播放一次后回到 Idle
    }
  },

  // 动作列表（对应 IAvatar.playMotion）
  "motions": {
    "idle":     { "state": "Idle",     "group": "default" },
    "walk":     { "state": "Walking",  "group": "default" },
    "think":    { "state": "Thinking", "group": "default" },
    "talk":     { "state": "Talking",  "group": "default" },
    "work":     { "state": "Working",  "group": "default" },
    "sleep":    { "state": "Sleeping", "group": "default" },
    "alert":    { "state": "Alert",    "group": "default" }
  },

  // 表情列表（SpriteSheet 可选支持，通过覆盖层实现）
  "expressions": {
    "default":  { "overlay": null },
    "happy":    { "overlay": "expressions/happy.png" },
    "sad":      { "overlay": "expressions/sad.png" },
    "angry":    { "overlay": "expressions/angry.png" },
    "surprised":{ "overlay": "expressions/surprised.png" }
  },

  // 默认配置
  "defaults": {
    "state": "Idle",
    "direction": "S",
    "motion": "idle",
    "expression": "default"
  }
}
```

### 4.2 帧动画播放引擎

| 功能 | 描述 |
|------|------|
| 帧调度 | 使用 `requestAnimationFrame` 驱动，按 `fps` 控制帧间隔 |
| 循环播放 | `loop: true` 时帧到末尾后回绕；`loop: false` 播放完毕后触发 `onComplete` 回调并切回 Idle |
| 方向切换 | 当 direction 变化时，保持当前帧索引不变，仅切换行偏移（row offset） |
| 状态切换 | 当 state 变化时，重置帧索引为 0，按新 state 的 fps 播放 |
| 表情覆盖 | 通过 `globalCompositeOperation` 叠加表情 PNG 到主帧之上 |
| 渲染尺寸 | 按 `frameWidth × frameHeight` 裁剪源图，缩放至 `scale` 参数后绘制到 Canvas |

### 4.3 IAvatar 接口适配

`SpriteSheetAvatar` 需实现 `IAvatar`（`src/core/avatar/types.ts`）的所有方法：

```typescript
class SpriteSheetAvatar implements IAvatar {
  // 加载 manifest.json 并预加载精灵表图片
  async load(config: AvatarConfig): Promise<void>

  // 更新（由渲染循环调用），推进帧动画
  update(deltaTime: number): void

  // 播放指定动作，可选参数覆盖
  playMotion(motion: string, group?: string, callback?: () => void): void

  // 停止当前动作
  stopMotion(): void

  // 设置表情（叠加表情层）
  setExpression(name: string): Promise<void>

  // 获取可用表情列表
  getExpressions(): string[]

  // 获取指定分组的可用动作列表
  getMotions(group?: string): string[]

  // 设置内部状态（触发状态对应动画切换）
  setState(state: PetState): void

  // 设置朝向角度（0-360），自动映射到 8 方向
  setDirection(degree: number): void

  // 设置缩放
  setScale(scale: number): void

  // 设置模型内部参数（帧动画无此概念，空实现）
  setParam(id: string, value: number): void

  // 销毁并释放资源
  dispose(): void
}
```

### 4.4 IRenderer 接口适配

`SpriteSheetRenderer` 需实现 `IRenderer`（`src/core/renderer/types.ts`）：

```typescript
class SpriteSheetRenderer implements IRenderer {
  constructor(private avatar: SpriteSheetAvatar)

  // 绑定到 DOM 容器
  bind(container: HTMLElement): void

  // 初始化 Canvas 上下文
  init(): Promise<void>

  // 每帧渲染：清除画布 → 绘制当前帧 → 绘制表情覆盖层
  render(): void

  // 响应窗口尺寸变化
  resize(width: number, height: number): void

  // 销毁渲染器，移除 Canvas
  dispose(): void
}
```

### 4.5 AvatarFactory 集成

修改 `src/core/avatar/factory.ts` 中的 `AvatarFactory.create()`：

```typescript
static create(config: AvatarConfig): IAvatar {
  switch (config.type) {
    case "live2d":
      return new Live2DAvatar();
    case "sprite":
      return new SpriteSheetAvatar();
    default:
      throw new Error(`Unknown avatar type: ${config.type}`);
  }
}
```

### 4.6 ModelRegistry 扩展

修改 `src/core/model/ModelRegistry.ts` 中的 `ModelInfo`：

```typescript
export interface ModelInfo {
  id: string;
  name: string;
  type: "live2d" | "sprite";     // ← 新增 "sprite"
  path: string;
  thumbnail?: string;
  source: "builtin" | "cdn" | "custom";
  status: "active" | "inactive";
  metadata?: Record<string, unknown>;
}
```

---

## 5. 技术设计 (Technical Design)

### 5.1 架构总览

```
┌─────────────────────────────────────────────────────┐
│                  App.vue                             │
│  ┌──────────────────┐  ┌──────────────────────────┐ │
│  │  PetHoverMenu     │  │  WindowCloseButton       │ │
│  └──────────────────┘  └──────────────────────────┘ │
│  ┌──────────────────────────────────────────────────┐│
│  │         AvatarHost (挂载容器)                     ││
│  │  ┌────────────────────────────────────────────┐  ││
│  │  │  SpriteSheetRenderer                       │  ││
│  │  │  ┌──────────────────────────────────────┐  │  ││
│  │  │  │  <canvas> 逐帧绘制                    │  │  ││
│  │  │  │  - 主精灵帧                           │  │  ││
│  │  │  │  - 表情覆盖层                         │  │  ││
│  │  │  └──────────────────────────────────────┘  │  ││
│  │  └────────────────────────────────────────────┘  ││
│  └──────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────┘

核心层：
  AvatarFactory.create({ type: "sprite", ... })
       │
       ▼
  SpriteSheetAvatar (implements IAvatar)
       │  持有
       ▼
  SpriteSheetAnimationEngine
       │  消费
       ▼
  manifest.json → SpriteSheetManifest (解析后的 TS 对象)
```

### 5.2 新增文件清单

```
src/core/avatar/
  └── SpriteSheetAvatar.ts        ← SpriteSheetAvatar 实现

src/core/renderer/sprite/
  ├── SpriteSheetRenderer.ts      ← SpriteSheetRenderer 实现
  └── index.ts                    ← 导出

src/core/sprite/
  ├── types.ts                    ← SpriteSheetManifest 类型定义
  ├── manifestParser.ts           ← manifest.json 解析与校验
  ├── animationEngine.ts          ← 帧调度引擎
  ├── directionMapper.ts          ← 角度 → 8 方向映射
  └── index.ts                    ← 导出
```

### 5.3 动画引擎核心逻辑

```typescript
// src/core/sprite/animationEngine.ts

interface AnimationState {
  currentFrame: number;       // 当前帧索引（相对于状态起始帧）
  elapsed: number;            // 已累积时间 (ms)
  frameInterval: number;      // 帧间隔 (ms) = 1000 / fps
  isPlaying: boolean;
  isComplete: boolean;        // loop=false 播放完毕后为 true
}

class SpriteSheetAnimationEngine {
  private state: AnimationState;
  private manifest: SpriteSheetManifest;
  private currentStateName: string;
  private currentDirection: Direction;

  constructor(manifest: SpriteSheetManifest) { ... }

  // 推进动画，返回当前应显示的帧信息
  update(deltaMs: number): FrameInfo {
    // 1. 累加 elapsed
    // 2. 当 elapsed >= frameInterval 时推进 currentFrame
    // 3. 处理循环 / 播放完毕逻辑
    // 4. 计算源图裁剪坐标 (sx, sy, sw, sh)
    // 5. 返回 FrameInfo
  }

  // 切换状态
  setState(stateName: string): void {
    // 1. 从 manifest.states 查找状态定义
    // 2. 重置 currentFrame = 0, elapsed = 0
    // 3. 更新 frameInterval
  }

  // 切换方向
  setDirection(dir: Direction): void {
    this.currentDirection = dir;
    // 方向切换不改帧索引，仅改行偏移
  }

  // 计算源图裁剪坐标
  private computeSourceRect(): SourceRect {
    const stateDef = this.manifest.states[this.currentStateName];
    const dirRow = this.manifest.directions.mapping[this.currentDirection];
    const absFrame = stateDef.frames.start + this.state.currentFrame;

    // 如果有方向：行 = 方向对应行 + (absFrame / columns) 的偏移
    // 如果无方向：行 = absFrame / columns
    const col = absFrame % this.manifest.spritesheet.columns;
    const row = dirRow + Math.floor(absFrame / this.manifest.spritesheet.columns);

    return {
      sx: col * (this.manifest.spritesheet.frameWidth + this.manifest.spritesheet.padding),
      sy: row * (this.manifest.spritesheet.frameHeight + this.manifest.spritesheet.padding),
      sw: this.manifest.spritesheet.frameWidth,
      sh: this.manifest.spritesheet.frameHeight
    };
  }
}

interface FrameInfo {
  sourceRect: SourceRect;         // 源图裁剪区域
  displayWidth: number;           // 显示宽度 = frameWidth * scale
  displayHeight: number;          // 显示高度 = frameHeight * scale
  expressionOverlay?: string;     // 表情覆盖层图片路径
}
```

### 5.4 方向映射算法

```typescript
// src/core/sprite/directionMapper.ts

type Direction = "N" | "NE" | "E" | "SE" | "S" | "SW" | "W" | "NW";

// 将 0-360 度映射到 8 方向
// 0° = 右(E)，逆时针递增（与现有 setDirection(degree) 一致）
function degreeToDirection(degree: number): Direction {
  // 标准化到 [0, 360)
  const normalized = ((degree % 360) + 360) % 360;
  // 每个方向占 45°，以 E(0°) 为中心
  const sector = Math.round(normalized / 45) % 8;
  const directions: Direction[] = ["E", "NE", "N", "NW", "W", "SW", "S", "SE"];
  return directions[sector];
}
```

### 5.5 manifest.json 校验

```typescript
// src/core/sprite/manifestParser.ts

import { z } from "zod";   // 或手动校验，视项目依赖而定

const ManifestSchema = z.object({
  version: z.string(),
  meta: z.object({
    name: z.string(),
    author: z.string().optional(),
    description: z.string().optional(),
    version: z.string().optional(),
    thumbnail: z.string().optional(),
    license: z.string().optional(),
  }),
  spritesheet: z.object({
    image: z.string(),
    frameWidth: z.number().positive(),
    frameHeight: z.number().positive(),
    columns: z.number().int().positive(),
    rows: z.number().int().positive(),
    padding: z.number().min(0).default(0),
    spacing: z.number().min(0).default(0),
  }),
  directions: z.object({
    enabled: z.boolean(),
    count: z.union([z.literal(1), z.literal(4), z.literal(8)]),
    mapping: z.record(z.string(), z.number().int().nonneg()),
  }).optional(),
  states: z.record(z.string(), z.object({
    frames: z.object({
      start: z.number().int().nonneg(),
      count: z.number().int().positive(),
    }),
    fps: z.number().positive().max(60),
    loop: z.boolean().default(true),
  })),
  motions: z.record(z.string(), z.object({
    state: z.string(),
    group: z.string().default("default"),
  })).optional(),
  expressions: z.record(z.string(), z.object({
    overlay: z.string().nullable(),
  })).optional(),
  defaults: z.object({
    state: z.string(),
    direction: z.string().default("S"),
    motion: z.string().optional(),
    expression: z.string().default("default"),
  }),
});

function parseManifest(raw: unknown): SpriteSheetManifest {
  const result = ManifestSchema.safeParse(raw);
  if (!result.success) {
    throw new Error(`Invalid manifest.json: ${result.error.message}`);
  }
  return result.data;
}
```

### 5.6 渲染管线

```typescript
// SpriteSheetRenderer.render() 核心流程

render(): void {
  const ctx = this.canvas.getContext("2d");
  if (!ctx) return;

  // 1. 清除画布
  ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

  // 2. 从动画引擎获取当前帧信息
  const frame = this.animationEngine.update(this.deltaTime);

  // 3. 绘制主帧（从精灵表裁剪）
  ctx.drawImage(
    this.spriteImage,
    frame.sourceRect.sx, frame.sourceRect.sy,
    frame.sourceRect.sw, frame.sourceRect.sh,
    this.offsetX, this.offsetY,
    frame.displayWidth, frame.displayHeight
  );

  // 4. 绘制表情覆盖层（如果有）
  if (frame.expressionOverlay && this.expressionImage) {
    ctx.globalCompositeOperation = "source-over";
    ctx.drawImage(
      this.expressionImage,
      this.offsetX, this.offsetY,
      frame.displayWidth, frame.displayHeight
    );
  }

  // 5. 重置合成模式
  ctx.globalCompositeOperation = "source-over";
}
```

---

## 6. 数据库设计 (Database Schema)

SpriteSheet 模型信息存储在 `models` 表中，通过 `type` 字段区分：

```sql
-- models 表扩展（已有表增加 type 字段）
-- 如果表已存在，通过 migration 添加:
ALTER TABLE models ADD COLUMN type TEXT NOT NULL DEFAULT 'live2d';
ALTER TABLE models ADD COLUMN manifest_path TEXT;  -- SpriteSheet 专属：manifest.json 相对路径

-- 新增模型时的插入示例：
INSERT INTO models (id, name, type, path, manifest_path, source, status)
VALUES ('pixel-cat-001', 'PixelCat', 'sprite', '/models/custom/pixel-cat/', 'manifest.json', 'custom', 'active');
```

完整的 `models` 表结构参见 [PRD-External-Model-Import.md](./PRD-External-Model-Import.md)。

---

## 7. UI/UX 设计

### 7.1 用户视角

SpriteSheet 模型对终端用户完全透明——用户通过设置面板导入/切换模型时，Live2D 和 SpriteSheet 模型混合显示在模型列表中，通过图标区分类型。

### 7.2 模型列表中的视觉区分

```
┌─────────────────────────────────────┐
│  模型列表                           │
├─────────────────────────────────────┤
│  🎭 Hiyori          [Live2D]  ● 活跃 │
│  🎭 Mao             [Live2D]  ○ 停用 │
│  🖼️ PixelCat        [Sprite]  ● 活跃 │
│  🖼️ DinoRun         [Sprite]  ○ 停用 │
└─────────────────────────────────────┘
```

### 7.3 开发调试

提供可选的调试覆盖层（开发模式下）：
- 显示当前帧编号
- 显示当前状态名称
- 显示当前方向
- 显示帧率
- 显示精灵表裁剪区域高亮

---

## 8. 实现计划 (Implementation Plan)

### 阶段一：基础设施（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 定义 `SpriteSheetManifest` TypeScript 类型 | 无 | `src/core/sprite/types.ts` |
| 实现 `manifestParser.ts` 解析与校验 | 类型定义 | 解析器 + 单元测试 |
| 实现 `directionMapper.ts` | 无 | 方向映射函数 + 单元测试 |
| 创建示例精灵表与 manifest.json | 无 | 测试资源 |

**验证点**：能正确解析 manifest.json，方向映射在 8 个扇区均正确。

### 阶段二：动画引擎（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 实现 `SpriteSheetAnimationEngine` | 阶段一 | 帧调度引擎 |
| 支持 8 方向帧定位 | 引擎 | 方向切换功能 |
| 支持状态切换与循环/单次播放 | 引擎 | 状态机逻辑 |
| 支持表情覆盖层 | 引擎 | 叠加渲染 |

**验证点**：引擎纯逻辑层可脱离 Canvas 独立测试，帧序列正确。

### 阶段三：渲染器与 Avatar 集成（预计 4 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 实现 `SpriteSheetRenderer` (IRenderer) | 阶段二 | Canvas 渲染器 |
| 实现 `SpriteSheetAvatar` (IAvatar) | 阶段二 | Avatar 适配器 |
| 修改 `AvatarFactory` 支持 sprite 类型 | Avatar 实现 | 工厂扩展 |
| 修改 `ModelRegistry` 支持 sprite 类型 | 无 | 注册表扩展 |
| 端到端集成测试 | 全部 | 可在 App.vue 中显示 |

**验证点**：`AvatarFactory.create({ type: "sprite" })` → 屏幕显示帧动画 → 状态切换响应正确。

### 阶段四：打磨与文档（预计 2 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 资源加载错误处理与降级 | 阶段三 | 错误处理 |
| 性能优化（帧缓存、离屏 Canvas）| 阶段三 | 优化 |
| manifest.json 规范文档 | 阶段一 | 开发者文档 |
| 示例模型打包与分发 | 阶段三 | 示例资源 |

**总计：约 12 个工作日**

---

## 9. 风险与约束 (Risks & Constraints)

| # | 风险 | 影响 | 缓解措施 |
|---|------|------|----------|
| R1 | 大型精灵表图片（>4096×4096）在部分 GPU 上渲染性能差 | 帧率下降 | 文档建议最大尺寸；支持分片精灵表（多文件） |
| R2 | manifest.json 格式设计不当，后续扩展困难 | 破坏性变更 | 使用 `version` 字段；保持向后兼容解析 |
| R3 | 表情覆盖层与主帧尺寸不匹配 | 视觉错位 | 强制覆盖层与主帧同尺寸，或在 manifest 中声明偏移 |
| R4 | 8 方向的素材制作门槛高 | 模型数量少 | 提供可选的 4 方向 / 无方向模式 |
| R5 | 帧动画文件体积大（高分辨率逐帧）| 下载/存储成本 | 建议使用像素风格小尺寸；可选 WebP 格式 |
| R6 | Tauri 窗口透明模式下 Canvas 性能 | 渲染卡顿 | 使用 `willReadFrequently: false`；测试透明叠加 |

### 约束

- **渲染后端**：必须使用 HTML Canvas 2D API（Tauri webview 兼容）
- **图片格式**：PNG（必须），WebP（可选）
- **帧率上限**：60 FPS（与显示器刷新率对齐）
- **内存预算**：单模型精灵表 + 表情层总内存 < 50MB
- **架构约束**：必须通过 `IAvatar` 接口交互，不允许绕过 Avatar 直接操作 Renderer
