# CoreAIpet 真正桌宠级窗口设计方案

Version: 1.0
Date: 2026-06-20

---

# 1. 目标

当前实现：

```text
Tauri Window
  └─ Live2D
```

已经可以实现：

* 透明窗口
* 无边框
* 置顶显示
* 桌面宠物

但仍然存在：

* 细边框
* 阴影
* 窗口感明显
* 占用矩形区域
* 透明区域阻挡鼠标

本方案目标：

实现类似：

* QQ宠物
* Desktop Goose
* Bongo Cat
* 桌面精灵

级别的桌宠窗口效果。

最终达到：

```text
用户只能看到角色
完全感知不到窗口存在
```

---

# 2. 当前方案问题

## 当前窗口

```text
┌──────────────┐
│              │
│    Live2D    │
│              │
└──────────────┘
```

实际上：

```text
整个400x400区域
都属于窗口
```

即使：

```text
背景透明
```

仍然：

```text
会拦截鼠标事件
```

---

# 3. 真正桌宠级窗口标准

## P0要求

支持：

* 无边框
* 无阴影
* 无任务栏图标
* 透明背景

---

## P1要求

支持：

* 非矩形窗口
* 点击穿透透明区域

---

## P2要求

支持：

* 不抢焦点
* 不影响工作流

---

## P3要求

支持：

* 多显示器
* DPI自适应

---

# 4. 技术实现方案

## 第一层：彻底去除边框与阴影

### tauri.conf.json

```json
{
  "windows": [
    {
      "transparent": true,
      "decorations": false,
      "shadow": false,
      "alwaysOnTop": true,
      "skipTaskbar": true,
      "resizable": false
    }
  ]
}
```

---

### Rust

```rust
let window = app.get_webview_window("main").unwrap();

let _ = window.set_shadow(false);
```

---

## 第二层：WebView完全透明

### App.vue

```css
html,
body,
#app {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: transparent;
}
```

---

### PixiJS

```typescript
const app = new PIXI.Application({
    backgroundAlpha: 0,
    antialias: false
});
```

---

# 5. 第三层：点击穿透

这是桌宠体验提升最大的部分。

---

## 普通窗口

```text
┌──────────────┐
│              │
│   宠物       │
│              │
└──────────────┘
```

整个窗口：

```text
都可点击
```

---

问题：

透明区域挡住桌面。

---

## 点击穿透窗口

```text
┌──────────────┐
│              │
│   ○宠物○     │
│              │
└──────────────┘
```

只有：

```text
角色本体
```

可点击。

---

其余：

```text
鼠标事件穿透到底层窗口
```

---

## Windows实现

增加扩展窗口样式：

```cpp
WS_EX_LAYERED
WS_EX_TRANSPARENT
```

---

效果：

```text
透明区域完全忽略鼠标
```

---

# 6. Rust实现

## 获取原生窗口

```rust
use windows::Win32::UI::WindowsAndMessaging::*;
```

---

获取 HWND

```rust
let hwnd = window.hwnd()?;
```

---

修改窗口样式

```rust
unsafe {
    let ex_style = GetWindowLongPtrW(
        hwnd,
        GWL_EXSTYLE
    );

    SetWindowLongPtrW(
        hwnd,
        GWL_EXSTYLE,
        ex_style
            | WS_EX_LAYERED.0 as isize
            | WS_EX_TRANSPARENT.0 as isize
    );
}
```

---

# 7. 智能穿透模式

完全穿透有个问题：

```text
用户点不到宠物
```

---

建议：

平时：

```text
点击穿透
```

悬停：

```text
取消穿透
```

---

状态切换：

```text
Normal
    ↓
Hover
    ↓
Interactive
```

---

逻辑：

```text
鼠标靠近角色
    ↓
关闭穿透
    ↓
允许交互

鼠标离开
    ↓
恢复穿透
```

---

# 8. 不抢焦点模式

目标：

点击宠物时：

```text
当前IDE
浏览器
VSCode
保持焦点
```

---

Windows样式：

```cpp
WS_EX_NOACTIVATE
```

---

效果：

```text
宠物响应点击

但不会抢夺输入焦点
```

---

# 9. 多窗口架构

推荐拆分：

```text
Pet Window
```

和

```text
Workspace Window
```

---

Pet Window

```text
180x220
```

职责：

* 动作
* 表情
* 通知
* 快捷入口

---

Workspace Window

```text
900x700
```

职责：

* Chat
* Jira
* Email
* Memory
* Settings

---

架构：

```text
┌───────┐
│ Pet   │
└───┬───┘
    │ Click
    ▼
┌─────────────────────┐
│ Workspace           │
├─────────────────────┤
│ Chat                │
│ Jira                │
│ Calendar            │
│ Email               │
└─────────────────────┘
```

---

# 10. 高级桌宠能力

## 角色吸附屏幕边缘

效果：

```text
拖到边缘
    ↓
自动吸附
```

类似：

```text
QQ宠物
```

---

## 自动避让窗口

检测：

```text
当前激活窗口
```

如果遮挡宠物：

```text
自动移动位置
```

---

## 多显示器支持

保存：

```json
{
  "monitor": 2,
  "x": 1800,
  "y": 600
}
```

恢复启动位置。

---

# 11. 推荐最终架构

```text
CoreAIpet

├── Pet Window
│   ├ Live2D
│   ├ Pixel Avatar
│   ├ Click Through
│   ├ Edge Snap
│   └ Notification
│
├── Workspace Window
│   ├ Chat
│   ├ Jira
│   ├ Email
│   ├ Calendar
│   └ Settings
│
└── Rust Runtime
    ├ Agent Runtime
    ├ Memory
    ├ EventBus
    ├ Plugin Manager
    └ Scheduler
```

---

# 12. 实施优先级

## P0

* 去阴影
* 去边框
* 完全透明

---

## P1

* 点击穿透
* 智能穿透切换

---

## P2

* 不抢焦点
* 系统托盘

---

## P3

* 多窗口架构
* Workspace Window

---

## P4

* 边缘吸附
* 多显示器支持
* 自动避让

---

# 最终效果

用户最终看到：

```text
桌面上只有一个Live2D角色
```

而不是：

```text
一个透明窗口里面放着Live2D角色
```

这就是桌宠产品与普通透明窗口应用之间的核心差异。
