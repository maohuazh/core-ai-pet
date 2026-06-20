## MODIFIED Requirements

### Requirement: Circular layout
6 个按钮 SHALL 以环形布局均匀分布在模型外围，半径约 80px，间隔 60°。按钮 MUST 不遮挡模型主体区域。在智能穿透模式下，按钮仅在鼠标进入角色区域（取消穿透）后可见和可交互。

#### Scenario: Buttons arranged in circle
- **WHEN** 悬浮菜单显示
- **THEN** 6 个按钮均匀分布在以模型中心为圆心、半径约 80px 的圆环上

#### Scenario: Menu hidden in click-through mode
- **WHEN** 鼠标离开角色区域，系统进入穿透模式
- **THEN** 悬浮菜单 SHALL 隐藏
- **THEN** 按钮不可交互
