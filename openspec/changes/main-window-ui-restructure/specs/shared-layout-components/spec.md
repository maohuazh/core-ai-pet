## ADDED Requirements

### Requirement: Shared layout components
系统 SHALL 在 `src/components/layout/` 目录下提供可复用的布局组件。

#### Scenario: AppLayout component
- **WHEN** 导入 `AppLayout.vue`
- **THEN** MUST 接受 `sidebarItems: NavItem[]` prop 用于侧边栏导航
- **THEN** MUST 接受 `activeItem: string` prop 用于高亮当前导航项
- **THEN** MUST 发射 `update:activeItem` 事件当用户切换导航
- **THEN** MUST 提供默认 slot 作为右侧内容区
- **THEN** MUST 提供 `#sidebar-footer` slot 用于侧边栏底部（如主题切换）

#### Scenario: AppSidebar component
- **WHEN** 导入 `AppSidebar.vue`
- **THEN** MUST 接受 `items: NavItem[]` 和 `active: string` props
- **THEN** MUST 支持 `collapsed: boolean` prop 控制折叠
- **THEN** MUST 发射 `update:active` 和 `toggle-collapse` 事件

#### Scenario: AppTopBar component
- **WHEN** 导入 `AppTopBar.vue`
- **THEN** MUST 接受 `title: string` prop 显示页面标题
- **THEN** MUST 接受 `isLoggedIn: boolean` 和 `userName: string` props
- **THEN** MUST 发射 `avatar-click` 和 `search-click` 事件

#### Scenario: NavItem type
- **WHEN** 导入 `types.ts`
- **THEN** MUST 导出 `NavItem` 接口，包含 `id: string`, `label: string`, `icon: string` 字段
