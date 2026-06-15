# Windows AI Desktop Companion PRD

Version: V1.0 MVP
Platform: Windows 10 / Windows 11
Priority: High

---

# 1. 产品概述

## 1.1 产品名称

AI Desktop Companion

---

## 1.2 产品定位

一个常驻于Windows桌面的AI虚拟助手。

助手以Live2D角色形式存在于桌面，支持：

* 桌宠交互
* AI对话
* 办公入口聚合
* 插件扩展

帮助用户快速访问工作系统和AI能力。

---

## 1.3 产品目标

构建轻量级桌面AI助手，实现：

* Live2D角色展示
* 桌面交互
* AI聊天
* 快捷功能入口
* 可扩展插件框架

---

# 2. 用户场景

## 场景1：桌面陪伴

用户打开电脑后。

桌面右下角出现AI助手。

助手持续播放待机动画。

---

## 场景2：快速访问工作系统

用户鼠标移动到角色上。

出现：

* Task
* Jira
* Email
* Message
* Setting
* Debug

用户点击后进入对应功能。

---

## 场景3：AI问答

用户点击角色。

弹出输入框。

输入：

"帮我总结今天的会议内容"

AI返回结果。

---

## 场景4：调整位置

用户按住角色拖动。

将角色移动到任意屏幕区域。

系统自动保存位置。

---

# 3. 功能需求

# 模块一：Live2D角色系统

## FR-001 角色展示

### 功能描述

系统显示Live2D角色。

---

### 要求

窗口：

* 无边框
* 无标题栏
* 背景透明
* 桌面置顶

仅显示角色内容。

---

## FR-002 多状态动画

角色支持以下状态：

| 状态       | 描述    |
| -------- | ----- |
| Idle     | 待机    |
| Happy    | Hover |
| Thinking | AI处理中 |
| Talking  | AI回复中 |

---

## FR-003 待机动画

包含：

* 呼吸
* 眨眼
* 轻微摆动

循环播放。

---

## FR-004 视线跟随

角色眼睛跟随鼠标移动。

限制范围：

水平 ±30°

垂直 ±15°

避免模型变形。

---

# 模块二：Hover快捷菜单

## FR-101 Hover触发

鼠标进入角色区域。

显示快捷菜单。

---

### 消失条件

鼠标离开：

1秒后自动隐藏。

---

## FR-102 菜单布局

以角色为中心环形排列。

布局：

```
      Task

Jira        Email

     [角色]
```

Message      Setting

```
      Debug
```

共6个入口。

---

## FR-103 功能按钮

### Task

任务中心。

---

### Jira

Jira系统入口。

---

### Email

邮件入口。

---

### Message

消息中心。

---

### Setting

系统设置。

---

### Debug

调试工具。

---

## FR-104 菜单动画

显示：

* Fade In
* Scale Up

隐藏：

* Fade Out

动画时长：

200~300ms

---

## FR-105 Hover反馈

鼠标悬停图标：

* 放大110%
* 显示Tooltip

---

# 模块三：AI聊天系统

## FR-201 打开聊天

触发方式：

* 点击角色
* 点击Task
* Alt + Space

---

## FR-202 聊天气泡

显示于角色上方。

布局：

┌────────────────┐
│ 输入问题...     │
└────────────────┘

[发送]

---

### 输入要求

最大长度：

4000字符

支持：

* 中文
* 英文
* 多行

---

### 快捷键

Enter：

发送

Shift + Enter：

换行

---

## FR-203 AI回复

回复显示在角色附近。

形式：

┌────────────────────┐
│ AI回复内容          │
└────────────────────┘

---

## FR-204 动画联动

发送问题：

Thinking

生成完成：

Talking

3秒后：

Idle

---

## FR-205 长文本处理

超过300字符：

显示：

查看更多

支持展开。

---

# 模块四：拖动与桌面交互

## FR-301 拖动角色

鼠标左键按住角色。

进入拖动状态。

---

## FR-302 拖动反馈

角色跟随鼠标移动。

刷新率：

60 FPS

---

## FR-303 保存位置

释放鼠标后：

保存：

Position(X,Y)

---

## FR-304 恢复位置

应用重启时：

恢复上次位置。

---

## FR-305 点击穿透

支持桌面穿透模式。

开启后：

* 角色可见
* 不阻挡桌面操作

快捷键：

Ctrl + Alt + P

---

# 模块五：系统托盘

## FR-401 托盘图标

应用驻留系统托盘。

---

## FR-402 托盘菜单

右键菜单：

* 显示助手
* 隐藏助手
* 设置
* 重启
* 退出

---

## FR-403 双击行为

双击托盘图标：

恢复显示。

---

# 模块六：插件系统

## FR-501 插件架构

支持扩展业务能力。

插件标准：

Plugin.json

示例：

{
"id": "jira",
"name": "Jira",
"icon": "jira.png"
}

---

## 生命周期

* Load
* Activate
* Execute
* Unload

---

# 模块七：办公插件

## FR-601 Jira插件

展示：

* Assigned To Me
* In Progress
* Blocked

数据来源：

Jira REST API

---

## FR-602 Email插件

展示：

* 未读邮件数量
* 最新邮件列表

点击：

打开Outlook

---

## FR-603 Message插件

展示：

* Teams消息
* Outlook通知

统一消息中心。

---

# 模块八：设置中心

## FR-701 外观设置

角色大小：

50% ~ 200%

透明度：

20% ~ 100%

主题：

* Light
* Dark

---

## FR-702 系统设置

* 开机启动
* 是否置顶
* 点击穿透

---

## FR-703 AI设置

支持：

* OpenAI
* Azure OpenAI
* Ollama

配置项：

* Endpoint
* API Key
* Model

---

# 模块九：Debug中心

## FR-801 调试面板

展示：

* FPS
* CPU
* Memory
* 插件状态
* 日志

供开发和测试使用。

---

# 4. 非功能需求

## 性能

启动时间：

< 5秒

---

## 内存

待机：

< 500MB

目标：

< 300MB

---

## CPU

待机：

< 5%

---

## 动画

推荐：

60 FPS

最低：

30 FPS

---

# 5. 数据存储

config.json

{
"position": {
"x": 1600,
"y": 820
},
"scale": 1.0,
"theme": "dark",
"alwaysOnTop": true,
"clickThrough": false
}

---

# 6. 技术方案

开发框架：

* .NET 8
* WPF

渲染：

* Live2D Cubism SDK

系统能力：

* Win32 API

数据存储：

* JSON

AI能力：

* OpenAI API
* Azure OpenAI
* Ollama

---

# 7. MVP验收标准

AC-001

显示Live2D角色。

AC-002

支持Idle、Happy、Thinking、Talking状态。

AC-003

支持视线跟随。

AC-004

支持拖动。

AC-005

支持位置保存。

AC-006

Hover显示6个功能入口。

AC-007

支持聊天输入框。

AC-008

支持AI回复。

AC-009

支持系统托盘。

AC-010

支持点击穿透。

AC-011

支持插件加载。

AC-012

Jira、Email、Message插件可正常展示数据。
