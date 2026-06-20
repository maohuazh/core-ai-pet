## ADDED Requirements

### Requirement: Transparent area click-through
系统 SHALL 在窗口透明区域（无 Live2D 模型像素的区域）实现鼠标点击穿透，使点击事件传递到下层的其他应用程序窗口。

#### Scenario: Click on transparent area passes through
- **WHEN** 用户点击窗口中完全透明的区域
- **THEN** 点击事件 SHALL 传递到下层的其他应用程序窗口
- **THEN** 宠物窗口 SHALL 不响应此点击

#### Scenario: Click on model area is captured
- **WHEN** 用户点击窗口中 Live2D 模型所在的区域
- **THEN** 宠物窗口 SHALL 捕获此点击事件
- **THEN** 下层窗口 SHALL 不接收此点击

### Requirement: Smart click-through toggle
系统 SHALL 根据鼠标位置智能切换穿透状态：鼠标离开角色区域时自动启用穿透，进入时自动关闭穿透。

#### Scenario: Mouse leaves model area
- **WHEN** 鼠标离开 pet-container 区域
- **THEN** 系统 SHALL 自动启用点击穿透
- **THEN** 透明区域的鼠标事件 SHALL 穿透到下层窗口

#### Scenario: Mouse enters model area
- **WHEN** 鼠标进入 pet-container 区域
- **THEN** 系统 SHALL 自动关闭点击穿透
- **THEN** 整个窗口 SHALL 可以接收鼠标事件

### Requirement: Windows native implementation
系统 SHALL 通过 Windows 原生窗口扩展样式实现点击穿透。

#### Scenario: Set layered and transparent styles
- **WHEN** 启用点击穿透
- **THEN** 系统 SHALL 设置 WS_EX_LAYERED 和 WS_EX_TRANSPARENT 扩展窗口样式
- **THEN** 透明区域 SHALL 完全忽略鼠标事件

#### Scenario: Remove transparent style
- **WHEN** 关闭点击穿透
- **THEN** 系统 SHALL 移除 WS_EX_TRANSPARENT 扩展窗口样式
- **THEN** 窗口 SHALL 恢复正常的鼠标事件响应
