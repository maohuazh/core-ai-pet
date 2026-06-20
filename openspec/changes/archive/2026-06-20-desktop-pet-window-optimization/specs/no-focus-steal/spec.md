## ADDED Requirements

### Requirement: No focus stealing on click
系统 SHALL 在用户点击宠物窗口时不抢夺当前活跃窗口的输入焦点。

#### Scenario: Click pet while IDE is active
- **WHEN** 用户在 IDE 中编辑代码时点击宠物窗口
- **THEN** IDE SHALL 保持输入焦点
- **THEN** 宠物窗口 SHALL 可以响应点击事件
- **THEN** 输入光标 SHALL 仍然在 IDE 中

#### Scenario: Click pet while browser is active
- **WHEN** 用户在浏览器中输入文字时点击宠物窗口
- **THEN** 浏览器 SHALL 保持输入焦点
- **THEN** 输入光标 SHALL 仍然在浏览器输入框中

### Requirement: Windows no-activate style
系统 SHALL 通过设置 WS_EX_NOACTIVATE 扩展窗口样式实现不抢焦点。

#### Scenario: Set no-activate style on startup
- **WHEN** 应用启动
- **THEN** 系统 SHALL 设置 WS_EX_NOACTIVATE 扩展窗口样式
- **THEN** 窗口 SHALL 不会因用户点击而被系统激活
