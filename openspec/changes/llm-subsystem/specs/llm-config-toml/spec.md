## ADDED Requirements

### Requirement: TOML 配置文件路径
系统 SHALL 读取 `~/.core-ai-pet/config.toml` 作为主配置。

#### Scenario: 文件不存在
- **WHEN** 启动时 config.toml 不存在
- **THEN** MUST 创建默认空配置（无 [llm.*] 段），并打 INFO 日志

#### Scenario: 文件存在但解析失败
- **WHEN** TOML 语法错误
- **THEN** MUST fail-closed：不启动 LLM 子系统，并 toast 提示用户修正配置

### Requirement: [llm.<role>] 槽位解析
系统 SHALL 解析 `[llm.chat_assistant]` 等嵌套段。

#### Scenario: 完整配置段
- **WHEN** 配置段含 provider/model/secret_ref/params
- **THEN** MUST 解析为 `LLMConfig` 对象并注册到 Harness

#### Scenario: 缺失必填字段
- **WHEN** 缺失 provider / model / secret_ref 任一
- **THEN** MUST fail-closed：该 role 不可用，并 toast 提示

#### Scenario: params 段可选
- **WHEN** 无 [llm.<role>.params] 段
- **THEN** MUST 使用默认值：`temperature=0.7, max_tokens=4096`

### Requirement: 配置热更新（M1 仅启动时加载）
M1 SHALL 不实现热更新：配置仅在启动时加载一次。

#### Scenario: 运行时修改 TOML
- **WHEN** 用户手动编辑 config.toml
- **THEN** M1 MUST 不感知变化，需重启应用才生效

#### Scenario: M2+ 热更新
- **WHEN** 未来版本
- **THEN** SHOULD 通过 file watch 触发 reload（M1 不实现）
