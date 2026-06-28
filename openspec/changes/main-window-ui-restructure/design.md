## Context

The current CoreAIpet application has three separate Tauri windows: a 200×200 transparent pet window (`/`), a 680×720 settings window (`/settings`), and a 960×640 chat window (`/chat`). Each window has its own layout system — the settings window uses a flexbox shell with sidebar+content, the chat window uses CSS Grid, and the pet window is just a transparent container. There is no shared layout component system, no navigation framework, and no main application window.

The tech stack is Vue 3.5 (Composition API, `<script setup>`), TypeScript, Tauri 2, Vite 6, with scoped CSS and design tokens (Catppuccin Mocha). There is no vue-router — routing is done via `window.location.pathname` checks in `App.vue`. All state is local `ref()` — no Pinia. All backend communication is via `invoke()` to Tauri Rust commands.

The user wants a full desktop application experience: a main window with sidebar navigation, top bar with user controls, and route-based content switching. The settings and chat windows should be redesigned to match the new visual language.

## Goals / Non-Goals

**Goals:**
- Create a main application window (1200×800) with sidebar navigation and content area
- Introduce shared layout components (AppLayout, AppSidebar, AppTopBar) used across all windows
- Implement route-based content switching using pathname routing (no vue-router dependency)
- Migrate existing settings modules (Jira, Email, Chat, Model) into the new settings layout
- Redesign chat window with new left sidebar while preserving all existing chat functionality
- Provide placeholder pages for all nav items (Home, Tasks, Schedule, Jira, Email, Message, Profile)
- All new UI follows existing Catppuccin Mocha design tokens

**Non-Goals:**
- Adding vue-router or any routing library (pathname-based routing is sufficient)
- Adding a state management library (Pinia/Vuex) — local `ref()` is fine for this scope
- Implementing actual backend functionality for placeholder pages (Home, Tasks, Schedule, etc.)
- User authentication / login backend integration (just UI state for now)
- Changing any Tauri Rust IPC APIs or database schemas
- Changing the pet window (transparent desktop pet) — it remains independent

## Decisions

### Decision 1: Pathname-based routing (no vue-router)

**Choice**: Continue using `window.location.pathname` checks in `App.vue` for route switching.

**Rationale**: The project currently has 3 routes (`/`, `/settings`, `/chat`) all handled via computed properties checking `window.location.pathname`. Adding vue-router introduces a dependency, learning curve, and bundle size for a simple case. The new main window routes (`/app`, `/app/home`, `/app/tasks`, etc.) can be handled the same way — a computed property checks if pathname starts with `/app` and a second computed extracts the sub-route.

**Alternatives considered**: vue-router — rejected because it's overkill for <10 routes and adds ~15kb to bundle. Hash routing — rejected because the app already uses path-based routing and Tauri window URLs are path-based.

### Decision 2: Shared layout component architecture

**Choice**: Create `src/components/layout/` with `AppLayout.vue`, `AppSidebar.vue`, `AppTopBar.vue` as composable shell components.

**Rationale**: Both the main window and the restructured settings window share the same layout pattern (sidebar + content area with top bar). Extracting this into shared components avoids duplication and ensures visual consistency. The chat window will use a variant of this layout.

**Component structure**:
```
src/components/layout/
├── AppLayout.vue        # Outer shell: top bar + flex row (sidebar + content)
── AppSidebar.vue       # Collapsible sidebar with nav items
├── AppTopBar.vue        # Top bar: logo left, actions right
├── AppContentArea.vue   # Content area wrapper with padding
└── types.ts             # Shared types (NavItem, LayoutProps)
```

**Alternatives considered**: Single monolithic layout component — rejected because the sidebar and top bar are used in different configurations across windows. CSS-only layout — rejected because sidebar state (collapse/expand, active item) needs Vue reactivity.

### Decision 3: Main window as separate Tauri window

**Choice**: Add a new Tauri window type `main` (1200×800, decorated, resizable) loaded at `/app` route.

**Rationale**: The main application window is fundamentally different from the pet window (200×200 transparent, always-on-top) and needs its own window properties. It also shouldn't share state with the pet window — the pet is a separate concern.

**Window config**:
- Size: 1200×800
- Min size: 900×600
- Decorations: true (native title bar)
- Transparent: false
- Route: `/app`

### Decision 4: Settings window restructure approach

**Choice**: Rewrite `SettingsPanel.vue` to use the new `AppLayout` shell. Migrate existing module components (`JiraModule.vue`, `EmailModule.vue`, `ChatModule.vue`, `ModelConfigModule.vue`) into the new sidebar structure. Add new `ShortcutsModule.vue`.

**Rationale**: The existing settings modules are well-structured and don't need rewriting — they just need a new shell. The sidebar items change from [Jira, Email, Chat, Pet, AI Model] to [Jira, Email, IM, Pet, AI Model, Shortcuts] — Chat becomes IM (broader concept), and Shortcuts is new.

### Decision 5: Chat window redesign approach

**Choice**: Restructure `ChatWindow.vue` to use the shared sidebar pattern for the session list (left side), while keeping the existing message area, streaming, and workspace selector intact on the right.

**Rationale**: The current chat window already has a session sidebar — it just needs to be styled to match the new design language and use shared components where possible. The message area is complex (streaming, thinking blocks, tool calls) and must not be broken.

### Decision 6: No authentication backend

**Choice**: Login state is UI-only (a `ref<boolean>` in the top bar component). Clicking the avatar when not logged in opens a placeholder "coming soon" modal.

**Rationale**: The user specified login state UI behavior but there's no auth backend yet. Building auth integration is out of scope — the UI should be ready for it when the backend arrives.

## Risks / Trade-offs

- **[Risk] Chat window functionality regression** → Mitigation: The chat message area code is not being rewritten, only the outer layout changes. Existing streaming, session management, and workspace features are preserved by only modifying the layout shell, not the inner message components.

- **[Risk] Pathname routing gets messy with many routes** → Mitigation: A simple `useRoute()` composable can be extracted later if needed. For now, <10 routes is manageable.

- **[Risk] Tauri window management complexity increases** (3 → 4 windows) → Mitigation: The new `open_main_window` command follows the same pattern as `open_settings_window` and `open_chat_window`. Hide-on-close behavior is consistent.

- **[Risk] Design token coverage** — ChatWindow.vue currently uses hardcoded hex values instead of CSS tokens → Mitigation: The redesigned chat window should migrate to using design tokens, following the UI style rules.

- **[Trade-off] No vue-router means no nested routes, guards, or programmatic navigation** → Acceptable for this scope. The app is a desktop app with fixed navigation, not a web app.

- **[Trade-off] Shared layout components couple the windows** → Acceptable because all windows should look consistent. If one window needs a drastically different layout, it can opt out of using `AppLayout`.
