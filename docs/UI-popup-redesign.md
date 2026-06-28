# 弹窗 UI 调整方案

> 基线风格：`src/modules/chat/ChatWindow.vue`（Catppuccin Mocha 深色）
> 长期规则：见 [`docs/UI-style-rules.md`](./UI-style-rules.md)（本方案落地后即作为项目级铁律）

---

## 0. 目标与非目标

**目标**
1. 把设置窗 UI 重构到与聊天窗一致的深色风格（**仅 UI，不动数据层**）
2. 把规则写成项目级文件，新弹窗自动遵循
3. 替换设置页里 `...` 三点菜单触发的浏览器原生 `prompt/alert`，改成依附锚点的小弹层
4. 宠物窗口新增右键唤出功能列表，复用现有 6 个悬浮按钮的内容，并采用统一风格

**非目标**
- 不改后端 Tauri 命令、不改 storage schema、不改 LLM/Jira/Email 业务逻辑
- 不把聊天窗自己再重构一次（它是基线，照搬即可）
- 不引入新的 UI 库（保持零依赖）

**成功判定**
- 现有所有 `invoke()` 调用与本地存储 key 都保持不变；用现有数据库启动应用，Jira/Email/Chat/Model/LLM 五个设置 tab 的数据完整可见
- 全工程 `grep -nE "alert\(|confirm\(|prompt\(|<select"` 在 `src/` 下无业务用法（白名单：测试代码）
- 设置页所有 `...` 菜单展开 = 小弹层（依附按钮）；不再有窗口居中遮罩或浏览器原生样式
- 在宠物窗右键 = 出现深色风格的 6 项功能菜单；点击行为与当前悬浮按钮一致

---

## 1. 总体结构

```
1. 落 tokens & 公共组件   ──→ 验证：组件 demo 页面截图比对聊天窗
2. 重构 Settings 外壳     ──→ 验证：能正常切 tab，数据完整
3. 重构 5 个 Settings 模块 ──→ 验证：每模块 invoke 链路不变
4. 替换设置页 ... 弹层    ──→ 验证：grep 无 alert/prompt
5. 宠物窗右键菜单         ──→ 验证：右键触发 + 6 项行为与悬浮按钮一致
6. 文档与索引收尾         ──→ 验证：CLAUDE.md 引用 rules
```

每步都能独立合入；3、4 可并行。

---

## 2. 阶段 1：基础设施（设计 token + 公共组件）

### 2.1 新建 token 文件
`src/assets/styles/tokens.css`：把 [`UI-style-rules.md` §1](./UI-style-rules.md#1-design-tokens) 全部颜色 / 间距 / 圆角写成 CSS 变量挂在 `:root`。
`src/main.ts` 顶部 `import "./assets/styles/tokens.css"`。

### 2.2 公共浮层组件（新建在 `src/components/ui/`）

| 文件                 | 职责                                                                 |
|----------------------|----------------------------------------------------------------------|
| `AppPopover.vue`     | 依附锚点的浮层基类；props: `anchor`（Element 或 `{x,y}`），`placement`（默认 `bottom-start`），`offset`（默认 6）；自动翻转、Teleport to body、ESC + 外部点击关闭 |
| `AppMenu.vue`        | 在 `AppPopover` 之上包一层 `items: MenuItem[]` API；支持图标、危险态、分隔线、disabled |
| `AppModal.vue`       | 居中模态；标题 + 内容插槽 + 按钮区；统一替代 `ConfirmDialog.vue`     |
| `AppTooltip.vue`     | hover 触发的小 popover；保留 `PetHoverMenu` 现有 tooltip 行为后再统一 |
| `useFloating.ts`     | 内部 composable：处理定位 / flip / outside-click（不引入 floating-ui，自己写 50 行就够） |

**API 草案（AppMenu）**：
```ts
type MenuItem =
  | { id: string; label: string; icon?: string; danger?: boolean; disabled?: boolean; onSelect: () => void }
  | { kind: 'divider' };

<AppMenu :anchor="btnEl" :open="show" :items="items" @update:open="show = $event" />
```

**验证**：
- 写一个临时 `/uikit` 路由（开发调试用，不进 menu），把 4 个组件各放一个 demo，肉眼比对聊天窗
- 完工删除路由

---

## 3. 阶段 2：重构 SettingsPanel 外壳

### 3.1 SettingsPanel.vue
- 整个面板背景：`--bg-base`；去掉 `backdrop-filter: blur`；圆角顶部 14、底部 0（贴满窗口）或 0（无窗口边距时）——按 Tauri 窗口实际外观选择
- `fade` 过渡保留，仅把不透明度过渡时间统一到 0.2s
- 滚动条样式用 `UI-style-rules.md §1.4`

### 3.2 SettingsTitleBar.vue
- 底：`--bg-surface`；标题字 `--text`；分隔线 `--border`
- 关闭按钮 hover：`--danger` 红
- 拖拽行为保留

### 3.3 SettingsSidebar.vue
- 底：`--bg-surface`，右边界 `--border`
- 项默认色 `--text-muted`，hover 加 `--bg-elevated`，active = `--bg-hover` + `--text`（不再用紫色 indigo）
- 左侧 active 指示条：`--accent`，宽度 3px
- 版本字 `--text-dim`

### 3.4 共享组件迁移
| 文件                 | 改动要点                                                              |
|----------------------|-----------------------------------------------------------------------|
| `ConnectionCard.vue` | 卡片底 `--bg-elevated`，副标题 `--text-dim`；`menu-btn`（…）改为触发 `AppMenu`（见 §5） |
| `ConfirmDialog.vue`  | **保留兼容性**：内部改用 `AppModal` 实现，对外 API 不变（避免改所有调用方）|
| `EmptyState.vue`     | 主文案 `--text-muted`，副文案 `--text-dim`，按钮主色 `--accent`           |
| `ToggleSwitch.vue`   | 轨道关 `--bg-hover`，开 `--accent`，滑块白                                |

**验证**：
- 启动应用 → 进设置窗 → 切 5 个 tab；数据可见
- 数据库无新写入（侧栏切换是纯前端状态）

---

## 4. 阶段 3：重构 5 个 Settings 业务模块

> **铁律**：本阶段只改 `<template>` 类名 / `<style>` 内的颜色与间距；
> 不动 `<script>` 里任何 `invoke()`、字段名、emit、props——这样存量配置一定不受影响。

逐文件 checklist：

| 文件                                              | 关注点                                                          |
|---------------------------------------------------|-----------------------------------------------------------------|
| `components/settings/modules/JiraModule.vue`      | 标题、按钮、卡片颜色 → token 化                                  |
| `components/settings/modules/EmailModule.vue`     | 同上                                                            |
| `components/settings/modules/ChatModule.vue`      | 同上 + **替换 `prompt`/`alert`**（见 §5）                       |
| `components/settings/modules/ModelConfigModule.vue` | 同上；如内部有 `<select>` 改 `AppMenu`                           |
| `modules/settings/LLMRoleForm.vue` & `LLMSettings.vue` | 输入框 / 按钮配色；保留所有现有保存逻辑                          |

**验证**（每个模块）：
1. 打开模块 → 字段值与改之前一致
2. 编辑并保存 → 重启应用后值仍正确
3. 控制台无 console.error
4. （LLM）连接测试按钮仍可触发 `llm_test_connection`

---

## 5. 阶段 4：替换设置页 `...` 浏览器原生展开

### 5.1 现状
- `ConnectionCard.vue` 的 `menu-btn`（三个圆点 SVG）emit `menu` 事件
- 各 `*Module.vue` 监听 `@menu` 后调用 `prompt(...)` 或 `alert(...)` —— 这就是用户说的"浏览器默认样式"

### 5.2 新方案
- `ConnectionCard.vue` 内部直接持有 `AppMenu` 状态：点击 `...` 时把按钮元素作为 anchor 打开菜单
- 通过 `prop: menuItems: MenuItem[]` 接收菜单项（业务在父组件定义）
- 这样调用方从 `@menu="handleMenu(p)"` + `prompt` 变为：

```vue
<ConnectionCard
  ...
  :menu-items="[
    { id:'rename', label:'编辑名称', icon:'✏️', onSelect: () => openRenameModal(platform) },
    { id:'delete', label:'删除',     icon:'🗑',  danger:true, onSelect: () => askDelete(platform) },
  ]"
/>
```

- `openRenameModal` 改用 `AppModal` + 输入框（替代 `prompt`）
- `askDelete` 沿用 `ConfirmDialog`（其已迁到 `AppModal` 底座）

### 5.3 同时替换
- `App.vue:247` 的 `alert("功能即将推出")` → 把 `PetHoverMenu` / 右键菜单点击改为打开统一的 "Coming Soon" 小 modal
- `ChatWindow.vue:117` 的 `<select>` workspace → 改成一个 `AppMenu` 触发器按钮（显示当前 workspace 名 + 下拉箭头），点击展开列表 + "📁 添加目录..." 项
- 全局 grep 兜底：

```
grep -rnE "alert\(|confirm\(|prompt\(|<select" src/ --include="*.vue" --include="*.ts"
```

只剩第三方 / 测试代码即通过。

**验证**：
- 在每个设置卡片点 `...` → 出现深色小菜单，且依附按钮，不是窗口居中
- 滚动设置面板时菜单跟随关闭
- ESC 关菜单

---

## 6. 阶段 5：宠物模型右键功能菜单

### 6.1 来源
当前 6 个项定义在 `src/components/PetHoverMenu.vue:35-42`：

```
📋 任务 / 💬 消息 / 🔗 Jira / 📧 邮件 / 🤖 Agent / ⚙️ 设置
```

行为分发在 `src/App.vue:233-248`。

### 6.2 改动
- `src/App.vue` 的 `.pet-container` 上加 `@contextmenu.prevent="onPetContextMenu($event)"`
- `onPetContextMenu(e)`：记录 `{x: e.clientX, y: e.clientY}`，打开 `<AppMenu>` 实例
- 菜单项 = 复用 `PetHoverMenu` 的同一份 `menuItems` 数组——**抽到 `src/core/pet/petMenu.ts`** 作为 single source of truth，hover 圆环与右键菜单都从这里导入
- 点击项调用现有 `handleMenuAction(action)`，行为不变
- 与 hover 圆环互斥：右键菜单打开期间，临时把 `showMenu`（hover 圆环）置 false；菜单关闭后由 hover 监听自然恢复

### 6.3 风格
- 复用 `AppMenu`，无需任何特化
- 透明窗口下的可读性：菜单是 Teleport 到 body，本身有 `--bg-surface` 实底，不需要额外背景

**验证**：
1. 右键宠物 → 在光标处出现 6 项菜单
2. ESC / 点击空白 → 关闭
3. 点击"设置" → 打开设置窗（沿用 `open_settings_window`）
4. 点击"消息" → 打开聊天窗
5. 其余 4 项 → 出现统一 "Coming Soon" modal（深色风格）
6. 左键 hover 圆环行为不受影响

---

## 7. 阶段 6：规则落库

### 7.1 已建文件
- `docs/UI-style-rules.md`（本方案的姊妹文档）

### 7.2 项目级 CLAUDE.md
在仓库根 `CLAUDE.md`（新建）追加：

```md
## UI 规则
所有窗口 / 弹窗 / 菜单必须遵循 docs/UI-style-rules.md。
新弹窗在 PR 描述里贴出 §3 的自查清单。
```

让 Claude Code 在本仓库工作时自动加载——以后所有弹窗都按这套出。

---

## 8. 风险与回退

| 风险                                  | 缓解                                                                 |
|---------------------------------------|----------------------------------------------------------------------|
| 配色迁移过程中漏改某文件               | 阶段 3 完工后 grep 旧 token (`#1f2937` / `#6366f1` / `rgba(255,255,255` 等) 全部清零 |
| `ConfirmDialog` 内部重写导致旧调用炸  | 保留对外 props/emits 完全一致，单元/手测每个调用点                    |
| Popover 定位翻转 bug                  | 加 e2e 不现实，先在 `/uikit` demo 路由放窗口角落 / 边缘验证            |
| 右键菜单与 Live2D 命中检测冲突        | 在 `App.vue` 监听 `contextmenu`，Live2D 不应自己消费右键；若冲突，给 `Live2DCanvas` 上 `pointer-events: none` 测试再决策 |
| `<select>` 改 `AppMenu` 漏掉键盘可达性 | `AppMenu` 必须支持 ↑↓ Enter Esc；阶段 1 demo 时一并验证                |

回退策略：每阶段一个 commit，可独立 revert。

---

## 9. 落地顺序建议（PR 拆分）

1. PR-A：tokens.css + AppPopover/AppMenu/AppModal/AppTooltip + `/uikit` demo（**最小可合**）
2. PR-B：Settings 外壳 + shared 组件迁移（依赖 A）
3. PR-C：5 个 Settings 模块改色（依赖 B；可拆 5 个小 PR）
4. PR-D：`...` 菜单与 `alert/prompt/<select>` 全替换（依赖 A、B）
5. PR-E：宠物右键菜单 + petMenu.ts 抽离（依赖 A）
6. PR-F：删除 `/uikit` demo 路由 + CLAUDE.md 索引 + docs 收尾
