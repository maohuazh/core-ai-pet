## ADDED Requirements

### Requirement: Load Live2D model
系统 SHALL 加载指定的 Live2D 模型文件并在窗口中渲染。

#### Scenario: Load model from file path
- **WHEN** 应用启动并指定模型路径
- **THEN** 系统 SHALL 使用 pixi-live2d-display 加载 .model3.json 文件
- **THEN** 模型 SHALL 渲染在窗口的中心位置
- **THEN** 模型 SHALL 自动缩放以适应实际渲染区域（由 canvas 元素尺寸决定），缩放因子为 0.75
- **THEN** PixiJS Application 的宽高 SHALL 与 canvas 元素的 clientWidth/clientHeight 一致

#### Scenario: Canvas dimensions match window
- **WHEN** Live2D 渲染器初始化时
- **THEN** 系统 SHALL 读取 canvas DOM 元素的实际像素尺寸（clientWidth × clientHeight）
- **THEN** PixiJS Application SHALL 使用该尺寸作为渲染分辨率
- **THEN** 渲染区域 SHALL 与 Tauri 窗口尺寸完全匹配（200×200 像素）

#### Scenario: Model loading progress
- **WHEN** 模型正在加载中
- **THEN** 系统 SHALL 在控制台输出加载进度
- **THEN** 加载完成后 SHALL 触发 onModelLoaded 事件

#### Scenario: Model loading error
- **WHEN** 模型文件不存在或格式错误
- **THEN** 系统 SHALL 在控制台输出错误信息
- **THEN** 窗口 SHALL 显示一个占位图标表示加载失败
