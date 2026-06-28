/**
 * 宠物功能菜单项 —— hover 圆环与右键菜单共享的 single source of truth。
 */
export interface PetMenuItem {
  action: string;
  icon: string;
  label: string;
}

export const petMenuItems: PetMenuItem[] = [
  { action: "home", icon: "🏠", label: "主页" },
  { action: "task", icon: "📋", label: "任务" },
  { action: "message", icon: "💬", label: "消息" },
  { action: "jira", icon: "🔗", label: "Jira" },
  { action: "email", icon: "📧", label: "邮件" },
  { action: "agent", icon: "🤖", label: "Agent" },
  { action: "settings", icon: "⚙️", label: "设置" },
];
