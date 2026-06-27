## ADDED Requirements

### Requirement: AI 模型设置面板
系统 SHALL 在设置面板中提供 `🤖 AI 模型` tab，展示所有可配置的 role。

#### Scenario: 显示已配置的 role
- **WHEN** 用户打开 AI 模型 tab
- **THEN** MUST 列出 `chat_assistant` role 的配置（provider / model / base_url / secret_ref 状态 / params）

#### Scenario: 未配置的 role
- **WHEN** role 配置段缺失
- **THEN** MUST 显示"未配置"占位，引导用户填写

### Requirement: 槽位编辑表单
系统 SHALL 提供表单控件编辑 LLMConfig 各字段。

#### Scenario: provider 下拉
- **WHEN** 用户点击 provider 下拉
- **THEN** MUST 显示可选 provider 列表（M1 仅 'anthropic'）

#### Scenario: model 输入
- **WHEN** 用户输入 model
- **THEN** MUST 接受任意字符串（不限制枚举，支持用户自部署 model 名）

#### Scenario: base_url 输入
- **WHEN** 用户输入 base_url
- **THEN** MUST 校验为合法 URL（非空时）

#### Scenario: secret 输入
- **WHEN** 用户输入新的 API key
- **THEN** MUST 调用 `llm_save_secret` 存为 secret_ref，更新配置中 secret_ref

### Requirement: 测试连接按钮
系统 SHALL 在 role 配置行提供"测试连接"按钮。

#### Scenario: 点击测试连接
- **WHEN** 用户点击"测试连接"
- **THEN** MUST 调用 `invoke('llm_test_connection', { role })`，UI 显示 loading → 成功 / 失败

#### Scenario: 测试成功
- **WHEN** Rust 端 ping 返回 `{ ok: true }`
- **THEN** UI MUST 显示绿色勾 + "连接正常"

#### Scenario: 测试失败
- **WHEN** ping 返回 `{ ok: false, reason }`
- **THEN** UI MUST 显示红叉 + reason（如 "unauthorized"、"network_error: timeout"）

### Requirement: 保存配置到 TOML
系统 SHALL 将用户在 UI 中的修改写回 config.toml。

#### Scenario: 保存
- **WHEN** 用户点击"保存"
- **THEN** MUST 原子写 config.toml 的对应段，不丢失其他段

#### Scenario: 写失败
- **WHEN** 写文件失败（权限/磁盘满）
- **THEN** MUST toast 提示 + 回滚 UI 状态
