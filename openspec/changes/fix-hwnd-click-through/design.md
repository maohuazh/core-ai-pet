## Context

当前 `window_native.rs` 通过 `raw-window-handle` 获取 HWND，得到的是 WebView2 子控件的 HWND（HWND_B），而非 Tauri 顶层窗口（HWND_A）。`WS_EX_TRANSPARENT` 设在 HWND_B 上不影响 OS 对顶层窗口的命中测试，导致点击穿透完全无效。

同时，cursor monitor 使用 `GetWindowRect` 获取整个 200×200 窗口矩形作为命中区域，导致鼠标还未接近模型就触发悬浮菜单。

## Goals / Non-Goals

**Goals:**
- 点击穿透在 Windows 上正常工作（光标在窗口外时鼠标事件穿透到下层窗口）
- 命中检测区域从 200×200 缩小到 160×160，减少无效触发距离
- 不引入新的外部依赖

**Non-Goals:**
- 不做像素级非矩形命中测试（需要 `WM_NCHITTEST` 子类化或 `SetWindowRgn`，超出本次修复范围）
- 不修改前端代码
- 不修改模型渲染逻辑

## Decisions

**决策 1：使用 `GetAncestor(hwnd, GA_ROOT)` 获取顶层 HWND**

替代方案：
- `GetParent()` — 只向上一层，如果层级更深则不够
- `GetAncestor(hwnd, GA_ROOTOWNER)` — 沿 owner 链而非 parent 链，不适用

选 `GA_ROOT`：直接到达顶层窗口，一层解决。

**决策 2：命中区域使用居中 160×160 矩形**

计算依据：6 个悬浮按钮分布在半径 80px 的圆周上，bounding box 为 160×160。命中区域至少需要覆盖按钮可达范围，否则按钮在穿透模式下无法点击。

替代方案：
- 80×80：按钮在区域外，点击按钮时穿透到桌面，不可用
- 200×200（当前）：鼠标距模型 50px 就触发，体验差
- 像素级命中：实现复杂度高，不在本次范围

**决策 3：统一封装 `get_root_hwnd` 函数**

替换所有 `get_hwnd` 调用点，确保 `apply_no_activate`、`apply_layered`、`set_click_through`、cursor monitor 都操作同一个顶层 HWND。

## Risks / Trade-offs

- **HWND 查找失败** → 返回错误日志，不 panic；穿透功能降级但不影响主流程
- **160×160 仍比模型大** → 窗口四角（160×160 与 200×200 之间的 L 形区域）仍有约 20px 空白会触发菜单，比当前 50px 显著改善
- **`GetAncestor` 依赖窗口层级已建立** → 在 `.setup()` 中调用时窗口已创建完成，安全
