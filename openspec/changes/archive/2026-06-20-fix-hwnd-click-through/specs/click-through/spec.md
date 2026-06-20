## MODIFIED Requirements

### Requirement: Windows native implementation
系统 SHALL 通过 Windows 原生窗口扩展样式实现点击穿透。WS_EX_TRANSPARENT 和 WS_EX_LAYERED 样式 SHALL 设置在顶层窗口（top-level HWND）上，而非 WebView2 子控件 HWND 上。系统 SHALL 使用 `GetAncestor(hwnd, GA_ROOT)` 获取顶层窗口的 HWND。

#### Scenario: Set layered and transparent styles on root HWND
- **WHEN** 启用点击穿透
- **THEN** 系统 SHALL 通过 `GetAncestor(hwnd, GA_ROOT)` 获取顶层窗口 HWND
- **THEN** 系统 SHALL 在顶层 HWND 上设置 WS_EX_LAYERED 和 WS_EX_TRANSPARENT 扩展窗口样式
- **THEN** 透明区域 SHALL 完全忽略鼠标事件

#### Scenario: Remove transparent style on root HWND
- **WHEN** 关闭点击穿透
- **THEN** 系统 SHALL 在顶层 HWND 上移除 WS_EX_TRANSPARENT 扩展窗口样式
- **THEN** 窗口 SHALL 恢复正常的鼠标事件响应

### Requirement: Smart click-through toggle
系统 SHALL 根据鼠标位置智能切换穿透状态：鼠标离开命中区域时自动启用穿透，进入时自动关闭穿透。命中区域 SHALL 为以窗口中心为原点的 160×160 像素矩形区域（而非整个 200×200 窗口区域）。

#### Scenario: Mouse leaves hit area
- **WHEN** 鼠标离开以窗口中心为原点的 160×160 像素矩形区域
- **THEN** 系统 SHALL 自动启用点击穿透
- **THEN** 鼠标事件 SHALL 穿透到下层窗口

#### Scenario: Mouse enters hit area
- **WHEN** 鼠标进入以窗口中心为原点的 160×160 像素矩形区域
- **THEN** 系统 SHALL 自动关闭点击穿透
- **THEN** 整个窗口 SHALL 可以接收鼠标事件
