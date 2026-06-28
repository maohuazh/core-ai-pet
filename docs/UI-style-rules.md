# UI 风格规则（项目级，长期生效）

本项目所有窗口、弹窗、菜单、对话框，**必须**遵循本文件的设计 token 与组件规范。
风格基线取自 `src/modules/chat/ChatWindow.vue`（Catppuccin Mocha 风深色主题）。

> 修改本文件前，请先在 PR 描述里贴出受影响的弹窗清单。

---

## 1. Design Tokens

> 所有数值都已在 ChatWindow.vue 中验证过，落地时优先以 CSS 变量形式声明在
> `src/assets/styles/tokens.css`，并在 `main.ts` 顶层导入。

### 1.1 颜色

| Token              | 值          | 用途                                         |
|--------------------|-------------|----------------------------------------------|
| `--bg-base`        | `#1e1e2e`   | 窗口主底色 / 输入框底色                      |
| `--bg-surface`     | `#181825`   | 侧栏 / 标题栏 / 底栏 / 弹层底                |
| `--bg-elevated`    | `#313244`   | 卡片 / 助手气泡 / hover 高亮                 |
| `--bg-hover`       | `#45475a`   | 二级 hover / 选中态                          |
| `--bg-hover-2`     | `#585b70`   | 按钮 hover                                   |
| `--border`         | `#313244`   | 主分隔线                                     |
| `--border-strong`  | `#45475a`   | 输入边框 / 块边框                            |
| `--text`           | `#cdd6f4`   | 主文本                                       |
| `--text-muted`     | `#a6adc8`   | 次级文本 / 节标题                            |
| `--text-dim`       | `#6c7086`   | 占位 / 空态 / 元信息                         |
| `--text-faint`     | `#7f849c`   | 思考块 / 工具调用参数                        |
| `--accent`         | `#89b4fa`   | 主强调色（按钮、focus、用户气泡）            |
| `--accent-hover`   | `#74c7ec`   | accent hover                                 |
| `--success`        | `#a6e3a1`   | 成功 / 工具名 / git 分支                     |
| `--danger`         | `#f38ba8`   | 删除 / 关闭红 / 错误                         |
| `--warning`        | `#f9e2af`   | 警告（新增；当前 UI 未用过，保留语义）       |

**禁止**：纯白底（`rgba(255,255,255,*)` glassmorphism）、`#1f2937` / `#6366f1` 系列浅色 token——这些是旧 SettingsPanel 的遗产，全部迁移。

### 1.2 字体

```
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
font-family-mono: 'Cascadia Code', 'Fira Code', monospace;   /* 仅代码 / 工具参数 */
```

字号阶梯（px）：`10 / 11 / 12 / 13 / 14 / 15 / 16`。
- 11：徽章 / 工具参数
- 12：副标题 / 元信息 / 输入下方提示
- 13：**正文与按钮默认**
- 14：模块标题 / 会话标题
- 15–16：窗口标题 / 空态主文案

### 1.3 圆角

| 名称   | 值     | 用途                       |
|--------|--------|----------------------------|
| `sm`   | `4px`  | 小标签 / 下拉            |
| `md`   | `6px`  | 列表项 / 小按钮          |
| `lg`   | `8px`  | 常规按钮 / 输入框 / 块   |
| `xl`   | `10px` | 徽章胶囊                 |
| `2xl`  | `14px` | 气泡 / 大卡片            |

气泡用非对称圆角（用户：`14 14 4 14`，助手：`14 14 14 4`），是基线规则，不要改。

### 1.4 间距 & 过渡

- 内边距标准刻度：`2 / 4 / 6 / 8 / 10 / 12 / 14 / 16 / 18 / 20 / 24`
- 过渡：`transition: <prop> 0.15s` 是基线（用于 hover/focus）。仅对话框/抽屉打开用 `0.2~0.25s` 的 `ease` 或 `ease-in-out`
- **滚动条**：宽度 `5px`，轨道透明，滑块 `--border-strong`，圆角 `3px`（已在 ChatWindow 验证）

### 1.5 阴影 & 模糊

- 弹层 / 浮层：`box-shadow: 0 8px 24px rgba(0,0,0,0.35)`（深色底需更深的影）
- **不使用 `backdrop-filter: blur()`**：当前聊天窗口无毛玻璃；旧设置窗的毛玻璃也要移除以保持一致

---

## 2. 组件规范

### 2.1 浮层分类（重要）

| 类型               | 触发           | 定位                       | 用什么实现                  |
|--------------------|----------------|----------------------------|-----------------------------|
| **Modal**          | 命令式 / 确认 | 视口居中，含遮罩           | `<AppModal>` (重构 ConfirmDialog) |
| **Popover**        | 点击锚点       | 依附锚点，自动翻转，无遮罩 | `<AppPopover>`（新建）      |
| **DropdownMenu**   | 点击 `...` / 选择 | Popover 的语义包装          | `<AppMenu>`（新建，基于 AppPopover） |
| **Tooltip**        | hover          | 依附锚点，纯展示           | `<AppTooltip>`（新建）      |
| **ContextMenu**    | 右键           | 跟随光标 (clientX/Y)       | `<AppMenu :anchor="point">` |

**禁止**使用：`alert()` / `confirm()` / `prompt()` / 浏览器原生 `<select>` 下拉 / 原生 `<dialog>`。
现存使用点（必须替换）：
- `src/components/settings/modules/ChatModule.vue:90` — `prompt()`
- `src/components/settings/modules/ChatModule.vue:85,123` — `alert()`
- `src/App.vue:247` — `alert()`
- `src/modules/chat/ChatWindow.vue:117` — 原生 `<select>` workspace 选择
- 其它 settings 模块里的同类调用，由实施时 grep 一遍补齐

### 2.2 AppPopover / AppMenu 行为契约

- **依附**：`anchor` 可传 `HTMLElement` 或 `{x,y}` 点；默认沿锚点底边对齐，空间不足时自动 flip 到顶 / 镜像翻边
- **关闭**：ESC、点击外部、滚动锚点祖先、路由 / 窗口失焦 → 关闭
- **焦点**：打开后焦点进入 Popover；关闭后回到锚点
- **不要遮罩**：Popover 不带 backdrop；要遮罩就改用 Modal
- **z-index**：Popover `1000`，Tooltip `1100`，Modal `2000`
- **Teleport to body**：所有浮层 Teleport 到 `body`，避免被父级 `overflow:hidden` 裁剪

### 2.3 AppModal 行为契约

- 遮罩：`rgba(0,0,0,0.5)`，**不用 blur**
- 容器：`--bg-surface` 底 + `1px solid --border-strong` + `box-shadow: 0 12px 40px rgba(0,0,0,0.5)`
- 圆角：`14px`
- 标题 16px / 内容 13px / 按钮 13px
- 主按钮 = `--accent`，危险 = `--danger`，取消 = 透明底 + `--text-muted`

### 2.4 表单控件

- `<input>` / `<textarea>`：底 `--bg-base`，边 `--border-strong`，focus 边 `--accent`，placeholder `--text-dim`，圆角 `8`，字号 `13`
- 自定义下拉：用 `AppMenu` 包装，不用 `<select>`
- 开关：沿用 `ToggleSwitch.vue`，但配色迁移到深色（轨道用 `--bg-hover`，开态用 `--accent`）

### 2.5 卡片

- 底：`--bg-elevated`
- 边：`1px solid --border-strong`
- 圆角：`12px`
- hover：边色 `--accent`（透明度 0.4 即可），不要 transform

---

## 3. 强制审查清单（写新弹窗时自查）

- [ ] 没有引入 `alert/confirm/prompt`、原生 `<select>` 下拉、`<dialog>`
- [ ] 所有颜色、间距、圆角都引用 token，没有裸 hex
- [ ] 浮层用 `<Teleport to="body">`
- [ ] Popover 实现了 ESC / 外部点击 / 滚动 关闭
- [ ] 焦点开 / 关时正确转移
- [ ] 没有 `backdrop-filter: blur()`
- [ ] 深色模式下文字对比度 ≥ AA（用 `--text` 或 `--text-muted`，不要自创灰）

---

## 4. 参考实现

- **基线样式**：`src/modules/chat/ChatWindow.vue` 的 `<style scoped>` 段（行 494–896）
- **Token 定义文件**（待新建）：`src/assets/styles/tokens.css`
- **组件**（待新建）：`src/components/ui/AppPopover.vue` / `AppMenu.vue` / `AppModal.vue` / `AppTooltip.vue`
