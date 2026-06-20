## MODIFIED Requirements

### Requirement: Transparent borderless window
系统 SHALL 创建一个无边框、无阴影、透明背景的窗口，用于显示 Live2D 宠物。

#### Scenario: Window creation with transparency
- **WHEN** 应用启动
- **THEN** 系统创建一个无边框（decorations: false）、无阴影（shadow: false）、透明背景（transparent: true）的窗口
- **THEN** 窗口尺寸 SHALL 为 200x200 像素
- **THEN** 窗口初始位置 SHALL 居中显示在屏幕中央
