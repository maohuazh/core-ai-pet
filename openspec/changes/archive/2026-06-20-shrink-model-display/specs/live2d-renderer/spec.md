## MODIFIED Requirements

### Requirement: Load Live2D model
系统 SHALL 加载指定的 Live2D 模型文件并在窗口中渲染。

#### Scenario: Load model from file path
- **WHEN** 应用启动并指定模型路径
- **THEN** 系统 SHALL 使用 pixi-live2d-display 加载 .model3.json 文件
- **THEN** 模型 SHALL 渲染在窗口的中心位置
- **THEN** 模型 SHALL 自动缩放以适应 240x240 的渲染区域，缩放因子为 0.75
