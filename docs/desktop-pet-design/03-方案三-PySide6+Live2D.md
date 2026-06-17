# 方案三：PyQt/PySide + Live2D 架构设计

## 技术选型

| 层次 | 技术 | 版本 |
|------|------|------|
| 桌面框架 | PySide6 (Qt 6) | 6.6+ |
| 编程语言 | Python | 3.11+ |
| 渲染引擎 | OpenGL 4.x | Qt 内置 |
| Live2D | live2d-py 或 自定义引擎 | - |
| 异步框架 | asyncio | 内置 |
| HTTP 客户端 | aiohttp | 3.9+ |
| 数据库 | SQLite | via aiosqlite |
| UI 组件 | PyQt-Fluent-Widgets | 1.5+ |
| 打包工具 | PyInstaller / cx_Freeze | - |

---

## 架构概览

```
┌──────────────────────────────────────────────────────────────┐
│                  Python Application                           │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Multi-Process Architecture                  │ │
│  │                                                          │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │ │
│  │  │ Main Process │  │ Pet Process  │  │ Chat Process │  │ │
│  │  │              │  │  (N个实例)    │  │              │  │ │
│  │  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────┐ │  │ │
│  │  │ │ Tray     │ │  │ │ PetWindow│ │  │ │ ChatView │ │  │ │
│  │  │ │ Manager  │ │  │ │          │ │  │ │          │ │  │ │
│  │  │ ├──────────┤ │  │ │┌────────┐│ │  │ │┌────────┐│ │  │ │
│  │  │ │ Config   │ │  │ ││ Live2D ││ │  │ ││Message ││ │  │ │
│  │  │ │ Manager  │ │  │ ││ Widget ││ │  │ ││ List   ││ │  │ │
│  │  │ ├──────────┤ │  │ │└────────┘│ │  │ │└────────┘│ │  │ │
│  │  │ │ IPC      │ │  │ │┌────────┐│ │  │ │┌────────┐│ │  │ │
│  │  │ │ Server   │ │  │ ││Behavior││ │  │ ││Input   ││ │  │ │
│  │  │ └──────────┘ │  │ ││ Engine ││ │  │ ││ Area   ││ │  │ │
│  │  └──────────────┘  │ │└────────┘│ │  │ └────────┘ │  │ │
│  │         │           │ └──────────┘ │  └──────────────┘  │ │
│  │         │           └──────────────┘         │           │ │
│  │         └────────────────┬───────────────────┘           │ │
│  │              Shared Memory IPC                            │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              Shared Services (Daemon)                    │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ AI       │ │ Database │ │ MCP      │ │ TTS      │  │ │
│  │  │ Service  │ │ Manager  │ │ Client   │ │ Service  │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

---

## 文件结构设计

```
pyside-desktop-pet/
│
├── pyproject.toml                  # 项目配置 (PEP 621)
├── requirements.txt                # 依赖列表
├── setup.py                        # 安装脚本
├── README.md                       # 项目说明
├── .env.example                    # 环境变量
│
├── src/
│   └── desktop_pet/
│       ├── __init__.py
│       ├── __main__.py             # 入口点
│       │
│       ├── app/                    # 应用层
│       │   ├── __init__.py
│       │   ├── application.py      # 主应用类
│       │   ├── process_manager.py  # 进程管理
│       │   └── signal_bus.py       # 信号总线
│       │
│       ├── processes/              # 进程定义
│       │   ├── __init__.py
│       │   ├── main_process.py     # 主进程
│       │   ├── pet_process.py      # 宠物进程
│       │   ├── chat_process.py     # 聊天进程
│       │   └── settings_process.py # 设置进程
│       │
│       ├── windows/                # 窗口组件
│       │   ├── __init__.py
│       │   ├── pet_window.py       # 宠物窗口 (核心)
│       │   ├── chat_window.py      # 聊天窗口
│       │   ├── settings_window.py  # 设置窗口
│       │   ├── menu_window.py      # 右键菜单窗口
│       │   └── bubble_window.py    # 对话气泡窗口
│       │
│       ├── widgets/                # 自定义控件
│       │   ├── __init__.py
│       │   ├── live2d_widget.py    # Live2D OpenGL 控件
│       │   ├── sprite_widget.py    # 精灵图控件
│       │   ├── chat_widget.py      # 聊天控件
│       │   ├── settings_widget.py  # 设置控件
│       │   └── tray_widget.py      # 系统托盘
│       │
│       ├── renderers/              # 渲染引擎
│       │   ├── __init__.py
│       │   ├── base_renderer.py    # 渲染器基类
│       │   ├── live2d/             # Live2D 渲染
│       │   │   ├── __init__.py
│       │   │   ├── renderer.py     # Live2D 渲染器
│       │   │   ├── model.py        # 模型管理
│       │   │   ├── motion.py       # 动作管理
│       │   │   ├── expression.py   # 表情管理
│       │   │   ├── hit_test.py     # 碰撞检测
│       │   │   ├── lip_sync.py     # 口型同步
│       │   │   └── head_track.py   # 头部追踪
│       │   └── sprite/             # 精灵图渲染
│       │       ├── __init__.py
│       │       ├── renderer.py     # 精灵图渲染器
│       │       ├── sheet.py        # 图集管理
│       │       └── animator.py     # 帧动画
│       │
│       ├── behavior/               # 行为系统
│       │   ├── __init__.py
│       │   ├── state_machine.py    # 状态机
│       │   ├── states/             # 状态定义
│       │   │   ├── __init__.py
│       │   │   ├── base_state.py
│       │   │   ├── idle_state.py
│       │   │   ├── walk_state.py
│       │   │   ├── talk_state.py
│       │   │   ├── sleep_state.py
│       │   │   ├── play_state.py
│       │   │   └── react_state.py
│       │   ├── transitions.py      # 转换规则
│       │   ├── triggers/           # 触发器
│       │   │   ├── __init__.py
│       │   │   ├── time_trigger.py
│       │   │   ├── mouse_trigger.py
│       │   │   ├── keyboard_trigger.py
│       │   │   └── system_trigger.py
│       │   └── scheduler.py        # 行为调度器
│       │
│       ├── ai/                     # AI 集成
│       │   ├── __init__.py
│       │   ├── base_provider.py    # 提供商基类
│       │   ├── providers/          # 提供商实现
│       │   │   ├── __init__.py
│       │   │   ├── openai_provider.py
│       │   │   ├── claude_provider.py
│       │   │   ├── gemini_provider.py
│       │   │   └── ollama_provider.py
│       │   ├── chat_service.py     # 聊天服务
│       │   ├── message_history.py  # 消息历史
│       │   ├── streaming.py        # 流式响应
│       │   └── tool_use.py         # 工具调用
│       │
│       ├── mcp/                    # MCP 集成
│       │   ├── __init__.py
│       │   ├── client.py           # MCP 客户端
│       │   ├── transport.py        # 传输层
│       │   ├── registry.py         # 工具注册
│       │   └── executor.py         # 工具执行
│       │
│       ├── tts/                    # 语音合成
│       │   ├── __init__.py
│       │   ├── service.py          # TTS 服务
│       │   ├── edge_tts.py         # Edge TTS
│       │   ├── local_tts.py        # 本地 TTS
│       │   └── lip_sync.py         # 口型同步
│       │
│       ├── asr/                    # 语音识别
│       │   ├── __init__.py
│       │   ├── service.py          # ASR 服务
│       │   └── whisper.py          # Whisper 集成
│       │
│       ├── db/                     # 数据库
│       │   ├── __init__.py
│       │   ├── manager.py          # 数据库管理器
│       │   ├── models/             # 数据模型
│       │   │   ├── __init__.py
│       │   │   ├── chat_message.py
│       │   │   ├── pet_state.py
│       │   │   ├── user_prefs.py
│       │   │   └── memory.py
│       │   └── migrations/         # 数据库迁移
│       │       └── 001_init.sql
│       │
│       ├── ipc/                    # 进程间通信
│       │   ├── __init__.py
│       │   ├── server.py           # IPC 服务器
│       │   ├── client.py           # IPC 客户端
│       │   ├── shared_memory.py    # 共享内存
│       │   ├── message_queue.py    # 消息队列
│       │   └── protocol.py         # 通信协议
│       │
│       ├── config/                 # 配置管理
│       │   ├── __init__.py
│       │   ├── manager.py          # 配置管理器
│       │   ├── schema.py           # 配置模式
│       │   └── defaults.py         # 默认配置
│       │
│       ├── services/               # 系统服务
│       │   ├── __init__.py
│       │   ├── window_manager.py   # 窗口管理
│       │   ├── tray_manager.py     # 托盘管理
│       │   ├── shortcut_manager.py # 快捷键管理
│       │   ├── auto_launch.py      # 开机自启
│       │   ├── screen_capture.py   # 屏幕截图
│       │   └── updater.py          # 自动更新
│       │
│       ├── models/                 # 数据类
│       │   ├── __init__.py
│       │   ├── pet.py              # 宠物模型
│       │   ├── chat.py             # 聊天模型
│       │   ├── config.py           # 配置模型
│       │   └── events.py           # 事件模型
│       │
│       ├── utils/                  # 工具函数
│       │   ├── __init__.py
│       │   ├── platform.py         # 平台检测
│       │   ├── logger.py           # 日志
│       │   ├── async_utils.py      # 异步工具
│       │   └── image_utils.py      # 图片处理
│       │
│       └── assets/                 # 资源文件
│           ├── models/             # Live2D 模型
│           │   └── .gitkeep
│           ├── sprites/            # 精灵图
│           │   └── .gitkeep
│           ├── audio/              # 音频
│           │   └── .gitkeep
│           ├── themes/             # 主题
│           │   ├── light.qss
│           │   └── dark.qss
│           ├── icons/              # 图标
│           │   ├── app.ico
│           │   └── tray.png
│           └── fonts/              # 字体
│               └── .gitkeep
│
├── tests/                          # 测试
│   ├── __init__.py
│   ├── test_behavior/
│   ├── test_ai/
│   ├── test_ipc/
│   └── test_renderers/
│
├── scripts/                        # 工具脚本
│   ├── build.py                    # 构建脚本
│   ├── package.py                  # 打包脚本
│   └── generate_sprites.py         # 精灵图生成
│
├── docs/                           # 文档
│   ├── architecture.md
│   ├── development.md
│   └── api.md
│
└── dist/                           # 打包输出
    └── .gitkeep
```

---

## 核心模块设计

### 1. Live2D OpenGL 控件

```python
# src/desktop_pet/widgets/live2d_widget.py

from PySide6.QtWidgets import QOpenGLWidget
from PySide6.QtCore import QTimer, Signal, Qt
from PySide6.QtGui import QMouseEvent
import OpenGL.GL as gl

class Live2DWidget(QOpenGLWidget):
    """Live2D 渲染控件"""
    
    # 信号
    clicked = Signal(str, int, int)  # hit_area, x, y
    drag_started = Signal(int, int)
    drag_moved = Signal(int, int)
    drag_ended = Signal()
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self._model = None
        self._renderer = None
        self._timer = QTimer(self)
        self._timer.timeout.connect(self._on_frame)
        self._fps = 60
        self._dragging = False
        self._drag_start_pos = None
        
    def initializeGL(self):
        """OpenGL 初始化"""
        gl.glClearColor(0.0, 0.0, 0.0, 0.0)  # 透明背景
        gl.glEnable(gl.GL_BLEND)
        gl.glBlendFunc(gl.GL_SRC_ALPHA, gl.GL_ONE_MINUS_SRC_ALPHA)
        
        # 初始化 Live2D 渲染器
        self._renderer = Live2DRenderer()
        self._renderer.initialize()
        
    def resizeGL(self, w: int, h: int):
        """窗口大小改变"""
        gl.glViewport(0, 0, w, h)
        if self._renderer:
            self._renderer.resize(w, h)
            
    def paintGL(self):
        """渲染帧"""
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)
        if self._renderer and self._model:
            self._renderer.render(self._model)
            
    def _on_frame(self):
        """帧更新"""
        self.update()
        
    def load_model(self, model_path: str):
        """加载 Live2D 模型"""
        if self._renderer:
            self._model = self._renderer.load_model(model_path)
            self._start_animation()
            
    def _start_animation(self):
        """启动动画"""
        interval = int(1000 / self._fps)
        self._timer.start(interval)
        
    def play_motion(self, group: str, index: int = 0, priority: int = 0):
        """播放动作"""
        if self._renderer and self._model:
            self._renderer.play_motion(self._model, group, index, priority)
            
    def set_expression(self, name: str):
        """设置表情"""
        if self._renderer and self._model:
            self._renderer.set_expression(self._model, name)
            
    def hit_test(self, x: int, y: int) -> str | None:
        """碰撞检测"""
        if self._renderer and self._model:
            return self._renderer.hit_test(self._model, x, y)
        return None
        
    # ---- 鼠标事件 ----
    
    def mousePressEvent(self, event: QMouseEvent):
        if event.button() == Qt.LeftButton:
            hit_area = self.hit_test(event.x(), event.y())
            if hit_area:
                self.clicked.emit(hit_area, event.x(), event.y())
            self._dragging = True
            self._drag_start_pos = event.globalPosition().toPoint()
            self.drag_started.emit(event.x(), event.y())
            
    def mouseMoveEvent(self, event: QMouseEvent):
        if self._dragging:
            current_pos = event.globalPosition().toPoint()
            delta = current_pos - self._drag_start_pos
            self._drag_start_pos = current_pos
            self.drag_moved.emit(delta.x(), delta.y())
            
    def mouseReleaseEvent(self, event: QMouseEvent):
        if event.button() == Qt.LeftButton:
            self._dragging = False
            self.drag_ended.emit()
            
    # ---- 口型同步 ----
    
    def set_mouth_open_y(self, value: float):
        """设置嘴部张开程度 (0.0 - 1.0)"""
        if self._renderer and self._model:
            self._renderer.set_parameter(self._model, "ParamMouthOpenY", value)
            
    # ---- 头部追踪 ----
    
    def look_at(self, x: float, y: float):
        """注视指定位置"""
        if self._renderer and self._model:
            self._renderer.look_at(self._model, x, y)
```

### 2. 行为状态机

```python
# src/desktop_pet/behavior/state_machine.py

from abc import ABC, abstractmethod
from enum import Enum
from typing import Callable

class PetState(Enum):
    IDLE = "idle"
    WALKING = "walking"
    TALKING = "talking"
    SLEEPING = "sleeping"
    PLAYING = "playing"
    REACTING = "reacting"

class State(ABC):
    """状态基类"""
    
    def __init__(self, name: PetState):
        self.name = name
        
    @abstractmethod
    def enter(self, context: dict):
        """进入状态"""
        pass
        
    @abstractmethod
    def update(self, dt: float, context: dict):
        """更新状态"""
        pass
        
    @abstractmethod
    def exit(self, context: dict):
        """退出状态"""
        pass

class Transition:
    """状态转换"""
    
    def __init__(
        self,
        from_state: PetState,
        to_state: PetState,
        condition: Callable[[dict], bool],
        action: Callable[[dict], None] | None = None
    ):
        self.from_state = from_state
        self.to_state = to_state
        self.condition = condition
        self.action = action

class StateMachine:
    """行为状态机"""
    
    def __init__(self):
        self._states: dict[PetState, State] = {}
        self._transitions: list[Transition] = []
        self._current_state: State | None = None
        self._context: dict = {}
        
    def add_state(self, state: State):
        """添加状态"""
        self._states[state.name] = state
        
    def add_transition(self, transition: Transition):
        """添加转换"""
        self._transitions.append(transition)
        
    def set_state(self, state_name: PetState):
        """设置状态"""
        new_state = self._states.get(state_name)
        if not new_state or new_state == self._current_state:
            return
            
        if self._current_state:
            self._current_state.exit(self._context)
            
        self._current_state = new_state
        self._current_state.enter(self._context)
        
    def update(self, dt: float):
        """更新状态机"""
        if not self._current_state:
            return
            
        # 检查转换条件
        for transition in self._transitions:
            if transition.from_state == self._current_state.name:
                if transition.condition(self._context):
                    if transition.action:
                        transition.action(self._context)
                    self.set_state(transition.to_state)
                    break
                    
        self._current_state.update(dt, self._context)
        
    @property
    def current_state(self) -> PetState | None:
        return self._current_state.name if self._current_state else None
        
    def set_context(self, key: str, value):
        """设置上下文"""
        self._context[key] = value
```

### 3. AI 提供商接口

```python
# src/desktop_pet/ai/base_provider.py

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import AsyncIterator

@dataclass
class ChatMessage:
    role: str  # "system", "user", "assistant"
    content: str
    timestamp: float

@dataclass
class ChatOptions:
    model: str | None = None
    temperature: float = 0.7
    max_tokens: int | None = None
    system_prompt: str | None = None

class AiProvider(ABC):
    """AI 提供商基类"""
    
    @property
    @abstractmethod
    def name(self) -> str:
        """提供商名称"""
        pass
        
    @property
    @abstractmethod
    def supported_models(self) -> list[str]:
        """支持的模型列表"""
        pass
        
    @abstractmethod
    async def chat(
        self,
        messages: list[ChatMessage],
        options: ChatOptions | None = None
    ) -> str:
        """发送聊天请求"""
        pass
        
    @abstractmethod
    async def chat_stream(
        self,
        messages: list[ChatMessage],
        options: ChatOptions | None = None
    ) -> AsyncIterator[str]:
        """流式聊天"""
        pass
        
    async def chat_with_tools(
        self,
        messages: list[ChatMessage],
        tools: list[dict],
        options: ChatOptions | None = None
    ) -> tuple[str, list[dict] | None]:
        """工具调用聊天（可选实现）"""
        return await self.chat(messages, options), None
```

### 4. IPC 共享内存

```python
# src/desktop_pet/ipc/shared_memory.py

import mmap
import struct
import json
from multiprocessing import shared_memory

class SharedMemoryQueue:
    """基于共享内存的消息队列"""
    
    HEADER_SIZE = 128  # 头部大小
    MESSAGE_SIZE = 4096  # 单条消息最大大小
    
    def __init__(self, name: str, size: int = 65536, create: bool = False):
        self.name = name
        self.size = size
        
        if create:
            self._shm = shared_memory.SharedMemory(create=True, size=size)
        else:
            self._shm = shared_memory.SharedMemory(name=name)
            
        self._buffer = self._shm.buf
        
    def put(self, message: dict):
        """放入消息"""
        data = json.dumps(message).encode('utf-8')
        if len(data) > self.MESSAGE_SIZE:
            raise ValueError("Message too large")
            
        # 读取写指针
        write_pos = struct.unpack_from('I', self._buffer, 0)[0]
        
        # 写入消息长度和数据
        struct.pack_into('I', self._buffer, write_pos, len(data))
        self._buffer[write_pos + 4:write_pos + 4 + len(data)] = data
        
        # 更新写指针
        new_pos = (write_pos + 4 + len(data)) % self.size
        struct.pack_into('I', self._buffer, 0, new_pos)
        
    def get(self) -> dict | None:
        """获取消息"""
        read_pos = struct.unpack_from('I', self._buffer, 4)[0]
        write_pos = struct.unpack_from('I', self._buffer, 0)[0]
        
        if read_pos == write_pos:
            return None  # 队列为空
            
        # 读取消息长度
        msg_len = struct.unpack_from('I', self._buffer, read_pos)[0]
        
        # 读取消息数据
        data = bytes(self._buffer[read_pos + 4:read_pos + 4 + msg_len])
        
        # 更新读指针
        new_pos = (read_pos + 4 + msg_len) % self.size
        struct.pack_into('I', self._buffer, 4, new_pos)
        
        return json.loads(data.decode('utf-8'))
        
    def close(self):
        """关闭共享内存"""
        self._shm.close()
        
    def destroy(self):
        """销毁共享内存"""
        self._shm.close()
        self._shm.unlink()
```

---

## 扩展性设计

### 1. 渲染器插件化

```python
# src/desktop_pet/renderers/base_renderer.py

from abc import ABC, abstractmethod

class Renderer(ABC):
    """渲染器基类"""
    
    @abstractmethod
    def initialize(self):
        pass
        
    @abstractmethod
    def load_model(self, path: str):
        pass
        
    @abstractmethod
    def render(self, model):
        pass
        
    @abstractmethod
    def play_animation(self, name: str):
        pass
        
    @abstractmethod
    def hit_test(self, x: int, y: int) -> str | None:
        pass
        
    @abstractmethod
    def destroy(self):
        pass

# 注册渲染器
RENDERER_REGISTRY: dict[str, type[Renderer]] = {}

def register_renderer(name: str, renderer_class: type[Renderer]):
    RENDERER_REGISTRY[name] = renderer_class

def create_renderer(name: str) -> Renderer:
    cls = RENDERER_REGISTRY.get(name)
    if not cls:
        raise ValueError(f"Unknown renderer: {name}")
    return cls()
```

### 2. 触发器插件化

```python
# src/desktop_pet/behavior/triggers/base_trigger.py

from abc import ABC, abstractmethod
from typing import Callable

class Trigger(ABC):
    """触发器基类"""
    
    def __init__(self, callback: Callable):
        self.callback = callback
        self.enabled = True
        
    @abstractmethod
    def start(self):
        pass
        
    @abstractmethod
    def stop(self):
        pass
        
    def trigger(self, data: dict):
        if self.enabled:
            self.callback(data)
```

### 3. 宠物配置化

```python
# 宠物配置文件 pet_config.json

{
  "name": "my-pet",
  "version": "1.0.0",
  "renderer": "live2d",
  "model_path": "models/my_pet/model.json",
  
  "behavior": {
    "initial_state": "idle",
    "states": {
      "idle": {
        "animations": ["idle_01", "idle_02"],
        "min_duration": 5.0,
        "max_duration": 30.0,
        "transitions": ["walk", "sleep"]
      },
      "walk": {
        "animations": ["walk_01"],
        "speed": 2.0,
        "max_distance": 300
      }
    },
    "schedule": [
      { "time": "08:00", "state": "greet", "message": "早上好！" },
      { "time": "12:00", "state": "remind", "message": "该吃午饭了！" },
      { "time": "22:00", "state": "sleep" }
    ]
  },
  
  "personality": {
    "system_prompt": "你是一个可爱的桌面宠物...",
    "greeting": "你好呀！",
    "idle_messages": ["有点无聊呢~", "陪我玩一会儿吧！", "..."]
  },
  
  "interaction": {
    "click_head": { "motion": "tap_head", "message": "不要摸头啦！" },
    "click_body": { "motion": "tap_body", "message": "嘿嘿~" },
    "double_click": { "action": "open_chat" }
  }
}
```

---

## 打包发布

```toml
# pyproject.toml

[build-system]
requires = ["setuptools>=68.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "desktop-pet"
version = "1.0.0"
description = "桌面宠物"
requires-python = ">=3.11"
dependencies = [
    "PySide6>=6.6.0",
    "aiohttp>=3.9.0",
    "aiosqlite>=0.19.0",
    "python-dotenv>=1.0.0",
]

[project.optional-dependencies]
live2d = ["live2d-py>=0.5.0"]
tts = ["edge-tts>=0.6.0"]
dev = ["pytest>=7.0.0", "black>=23.0.0"]

[project.scripts]
desktop-pet = "desktop_pet.__main__:main"
```
