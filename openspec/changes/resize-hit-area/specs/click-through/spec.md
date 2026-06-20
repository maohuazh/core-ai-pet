## MODIFIED Requirements

### Requirement: Smart click-through toggle
系统 SHALL 根据鼠标位置智能切换穿透状态：鼠标离开命中区域时自动启用穿透，进入时自动关闭穿透。命中区域 SHALL 为以窗口中心为原点的 75×100 像素矩形区域（宽 75px，高 100px）。

#### Scenario: Mouse leaves hit area
- **WHEN** 鼠标离开以窗口中心为原点的 75×100 像素矩形区域
- **THEN** 系统 SHALL 自动启用点击穿透
- **THEN** 鼠标事件 SHALL 穿透到下层窗口

#### Scenario: Mouse enters hit area
- **WHEN** 鼠标进入以窗口中心为原点的 75×100 像素矩形区域
- **THEN** 系统 SHALL 自动关闭点击穿透
- **THEN** 整个窗口 SHALL 可以接收鼠标事件
