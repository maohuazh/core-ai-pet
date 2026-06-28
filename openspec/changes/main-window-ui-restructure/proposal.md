## Why

The current application has three separate Tauri windows (pet, settings, chat) with no unified UI architecture. Each window has its own layout, styling approach, and component structure. The user wants a full-featured main application window with sidebar navigation, a shared layout system, and route-based content switching — transforming the app from a floating pet + utility windows into a cohesive desktop application. This also requires restructuring the settings and chat windows to follow the same design language.

## What Changes

- **New main application window** (`/app` route) with left sidebar navigation and right content area
- **Shared layout component system**: `AppLayout`, `AppSidebar`, `AppTopBar` — used across all windows
- **Sidebar navigation** with items: Home, Tasks, Chat, Jira, Email, Message — each switching the right content area
- **Top bar** with Logo (left), search/notification/avatar buttons (right), login state handling
- **Theme toggle** (dark/light) fixed at sidebar bottom
- **New placeholder pages**: HomePage, TasksPage, SchedulePage, JiraPage, EmailPage, MessagePage, ProfilePage
- **Restructured Settings window** — new sidebar with items: Jira, Email, IM, Pet, AI Model, Shortcuts; existing settings modules migrated into new layout
- **Redesigned Chat window** — new left sidebar layout (session list + search) with existing chat message area preserved
- **Tauri Rust changes** — new `open_main_window` command, updated window config

## Capabilities

### New Capabilities

- `main-window-layout`: Main application window with AppLayout shell (top bar + sidebar + content area), route-based content switching, collapsible sidebar
- `app-sidebar-navigation`: Left sidebar with nav items (Home, Tasks, Chat, Jira, Email, Message), active state, collapsible, theme toggle at bottom
- `app-top-bar`: Top bar with logo, search button, notification bell, avatar/login state, dropdown menu for logged-in users
- `shared-layout-components`: Reusable AppLayout, AppSidebar, AppTopBar, AppContentArea components used across all windows
- `placeholder-pages`: HomePage, TasksPage, SchedulePage, JiraPage, EmailPage, MessagePage, ProfilePage — placeholder text
- `main-window-routing`: Route-based content switching within the main window (`/app`, `/app/home`, `/app/tasks`, etc.)
- `settings-window-restructure`: Settings window rebuilt with new shared layout; existing Jira/Email/Chat/Model modules migrated; new Shortcuts module

### Modified Capabilities

- `settings-window`: Layout and navigation restructured — sidebar items change from [Jira, Email, Chat, Pet, AI Model] to [Jira, Email, IM, Pet, AI Model, Shortcuts]; uses shared AppLayout
- `chat-window`: Redesigned with new left sidebar layout (chat sessions list, search, new chat button); existing chat message area, streaming, workspace selector preserved

## Impact

- **New Tauri window**: `main` window type added to `tauri.conf.json` with larger dimensions (1200×800)
- **Rust backend**: New `open_main_window` command in `commands/settings.rs`
- **Vue routing**: Currently no vue-router — adding pathname-based routing for `/app/*` paths in `App.vue`
- **Existing components**: `SettingsPanel.vue`, `SettingsSidebar.vue`, `ChatWindow.vue` significantly restructured or replaced
- **Design tokens**: All new components use existing Catppuccin Mocha tokens from `tokens.css`
- **No breaking API changes**: All Tauri IPC commands remain unchanged; existing settings modules are migrated, not rewritten
