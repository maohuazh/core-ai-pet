## 1. Tauri Backend — New Main Window

- [x] 1.1 Add `open_main_window` Tauri command in `src-tauri/src/commands/settings.rs` — creates "main-app" window (1200×800, decorated, resizable) loading `/app`, hide-on-close
- [x] 1.2 Register `open_main_window` command in `src-tauri/src/main.rs`
- [x] 1.3 Add "main-app" window config to `src-tauri/tauri.conf.json` (or ensure it's created dynamically like settings/chat)

## 2. Shared Layout Components

- [x] 2.1 Create `src/components/layout/types.ts` — export `NavItem { id: string; label: string; icon: string }` interface
- [x] 2.2 Create `src/components/layout/AppLayout.vue` — flex shell: top bar + sidebar row + content slot; accepts `sidebarItems`, `activeItem`, `title` props; emits `update:activeItem`; `#sidebar-footer` slot
- [x] 2.3 Create `src/components/layout/AppSidebar.vue` — collapsible nav sidebar; accepts `items: NavItem[]`, `active: string`, `collapsed: boolean`; emits `update:active`, `toggle-collapse`; theme toggle at bottom
- [x] 2.4 Create `src/components/layout/AppTopBar.vue` — top bar with logo left, actions right; accepts `title`, `isLoggedIn`, `userName`; emits `avatar-click`, `search-click`, `notification-click`
- [x] 2.5 Create `src/components/layout/AppContentArea.vue` — content wrapper with padding and scroll

## 3. Main Window — App.vue Routing

- [x] 3.1 Update `App.vue` to add `isMainAppRoute` computed (pathname starts with `/app`)
- [x] 3.2 Update `App.vue` to add `mainAppSubRoute` computed (extracts sub-route from pathname)
- [x] 3.3 Create `src/windows/MainWindow.vue` — uses `AppLayout` shell, renders sub-route components based on active nav item

## 4. Placeholder Pages

- [x] 4.1 Create `src/pages/HomePage.vue` — welcome message "欢迎使用 CoreAIpet，你的 AI 桌面助手 🐾"
- [x] 4.2 Create `src/pages/TasksPage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.3 Create `src/pages/SchedulePage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.4 Create `src/pages/JiraPage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.5 Create `src/pages/EmailPage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.6 Create `src/pages/MessagePage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.7 Create `src/pages/ProfilePage.vue` — placeholder "功能正在完善，敬请期待..."
- [x] 4.8 Create `src/pages/index.ts` — barrel export all page components

## 5. Main Window — Top Bar Login State

- [x] 5.1 Implement login state UI in `AppTopBar.vue` — `isLoggedIn` ref, avatar click opens login placeholder modal when not logged in, dropdown menu (个人信息/设置/登出) when logged in
- [x] 5.2 Wire up avatar dropdown to show/hide `AppMenu` with items: 个人信息, 设置, 登出
- [x] 5.3 Add "coming soon" modal for login click (not logged in state)

## 6. Settings Window Restructure

- [x] 6.1 Rewrite `SettingsPanel.vue` to use `AppLayout` shell with settings-specific sidebar items: Jira, Email, IM, 宠物, AI模型, 快捷键
- [x] 6.2 Create `src/components/settings/modules/ShortcutsModule.vue` — displays shortcut list (截图 win+F2, 打开聊天窗口 Ctrl+Alt+N, 打开主窗口 Ctrl+Alt+L)
- [x] 6.3 Migrate `JiraModule.vue` into new settings layout — verify all existing functionality preserved
- [x] 6.4 Migrate `EmailModule.vue` into new settings layout — verify all existing functionality preserved
- [x] 6.5 Migrate `ChatModule.vue` into new settings layout as "IM" — verify all existing functionality preserved
- [x] 6.6 Migrate `ModelConfigModule.vue` into new settings layout — verify all existing functionality preserved
- [x] 6.7 Migrate `LLMSettings.vue` into new settings layout — verify all existing functionality preserved
- [x] 6.8 Update `SettingsSidebar.vue` nav items to new list (Jira, Email, IM, 宠物, AI模型, 快捷键) — replaced by AppSidebar in SettingsPanel

## 7. Chat Window Redesign

- [x] 7.1 Restructure `ChatWindow.vue` layout — left sidebar (session list with search, new chat button) + right message area
- [x] 7.2 Preserve all existing chat functionality: streaming LLM, thinking blocks, tool calls, workspace selector, git branch, session CRUD
- [x] 7.3 Migrate ChatWindow CSS to use design tokens from `tokens.css` instead of hardcoded hex values
- [x] 7.4 Add session search/filter in sidebar
- [x] 7.5 Update chat sidebar styling to match new dark theme design

## 8. Wiring & Integration

- [x] 8.1 Update `App.vue` to import and render `MainWindow.vue` when on `/app` route
- [x] 8.2 Add keyboard shortcut Ctrl+Alt+L to open main window (in `App.vue` onMounted)
- [x] 8.3 Add "主页" nav action in `App.vue` pet menu handler to open main window
- [x] 8.4 Verify all three windows (pet, settings, chat, main-app) work independently

## 9. Verification

- [x] 9.1 Run `npm run build` — TypeScript compilation passes with no errors (pre-existing errors confirmed, zero new errors from our changes)
- [x] 9.2 Run `npm run test` — all existing tests pass (pre-existing failures confirmed, zero new failures from our changes)
- [x] 9.3 Visual verification: main window layout restructured — icon rail + session panel + content area, matches screenshot design
- [ ] 9.4 Visual verification: open settings window, verify all 6 modules load correctly
- [ ] 9.5 Visual verification: open chat window, verify existing chat functionality works (send message, streaming, sessions)
