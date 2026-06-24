## MODIFIED Requirements

### Requirement: Settings window close behavior
设置窗口关闭时 SHALL 隐藏而非销毁。

#### Scenario: Close settings window via X button
- **WHEN** 用户点击设置窗口标题栏的 X 关闭按钮
- **THEN** 窗口 MUST 隐藏（hide）而非销毁
- **THEN** 窗口状态和数据 MUST 保留
- **THEN** 窗口 MUST 从屏幕消失（用户可见窗口消失）

#### Scenario: Reopen settings window
- **WHEN** 设置窗口已隐藏且用户再次打开
- **THEN** 窗口 MUST 显示（show）已有实例
- **THEN** 窗口 MUST 保留之前的状态和数据

### Requirement: Custom title bar
设置窗口 SHALL 提供自定义标题栏支持拖拽和关闭。

#### Scenario: Title bar drag
- **WHEN** 用户在标题栏区域按下鼠标并拖动
- **THEN** 窗口 MUST 跟随鼠标移动

#### Scenario: Title bar close button
- **WHEN** 用户点击标题栏的关闭按钮（X）
- **THEN** 窗口 MUST 立即隐藏
- **THEN** 应用 MUST 不退出

#### Scenario: Title bar minimize button
- **WHEN** 用户点击标题栏的最小化按钮
- **THEN** 窗口 MUST 最小化到任务栏
