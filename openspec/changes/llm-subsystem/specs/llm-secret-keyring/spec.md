## ADDED Requirements

### Requirement: OS Keyring 存取
系统 SHALL 通过 `keyring` crate 在 OS Keychain 中存储/读取 API key 明文。

#### Scenario: 存 secret
- **WHEN** 调用 `invoke('llm_save_secret', { role, plaintext })`
- **THEN** Rust MUST 生成 UUID 作为 `secret_ref`，存入 Keyring service=`coreai-llm` 用户名=secret_ref 的条目，返回 `{ secret_ref }`

#### Scenario: 取 secret
- **WHEN** 调用 `invoke('llm_get_secret', { secret_ref })`
- **THEN** MUST 从 Keyring 读取并返回 `{ plaintext }`

#### Scenario: 取不存在的 secret
- **WHEN** secret_ref 在 Keyring 中找不到
- **THEN** MUST 返回错误 `{ error: 'secret_not_found' }`

### Requirement: DPAPI fallback（Windows / 无 Keyring 环境）
系统 SHALL 在 Keyring 不可用时回落到 `~/.core-ai-pet/.secrets/<secret_ref>` + DPAPI 加密（Windows）/ 文件权限 0600（Unix）。

#### Scenario: Keyring 失败
- **WHEN** Keyring 调用抛错（无守护进程 / 权限拒绝）
- **THEN** MUST 自动 fallback 到文件加密存储，并打 WARN 日志

#### Scenario: 文件权限（Unix）
- **WHEN** 创建 `.secrets/<secret_ref>` 文件
- **THEN** MUST 设置 mode 0600（仅 owner 可读写）

#### Scenario: 删除 secret
- **WHEN** 调用 `invoke('llm_delete_secret', { secret_ref })`
- **THEN** MUST 从 Keyring 和/或 fallback 文件删除对应条目

### Requirement: 明文不出 Rust 进程
系统 SHALL 保证 API key 明文只在 Rust 进程内使用，Renderer 只持有 secret_ref。

#### Scenario: Provider 请求
- **WHEN** Facade 发起 Provider 请求
- **THEN** Rust MUST 在 Rust 端注入 Authorization / x-api-key header，emit_to 给 Renderer 的 payload 中 MUST 不含任何 key

#### Scenario: Renderer 侧审计
- **WHEN** grep Renderer 端代码
- **THEN** MUST 找不到任何直接拼接 API key 的代码路径
