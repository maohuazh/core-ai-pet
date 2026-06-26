/**
 * Predefined effects list
 */

import type { AvailableEffect } from "./types";

export const AVAILABLE_EFFECTS: AvailableEffect[] = [
  {
    id: "sparkle",
    name: "闪光",
    icon: "✨",
    description: "闪烁的星光效果",
    defaultDuration: 1500,
  },
  {
    id: "heart",
    name: "爱心",
    icon: "❤️",
    description: "飘浮的爱心",
    defaultDuration: 2000,
  },
  {
    id: "sweat_drop",
    name: "汗滴",
    icon: "💧",
    description: "额头汗滴效果",
    defaultDuration: 2500,
  },
  {
    id: "exclamation",
    name: "感叹号",
    icon: "❗",
    description: "头顶感叹号",
    defaultDuration: 1500,
  },
  {
    id: "question",
    name: "问号",
    icon: "❓",
    description: "头顶问号",
    defaultDuration: 2000,
  },
  {
    id: "music_note",
    name: "音符",
    icon: "🎵",
    description: "跳动的音符",
    defaultDuration: 2000,
  },
  {
    id: "zzz",
    name: "睡眠",
    icon: "💤",
    description: "睡眠 ZZZ 效果",
    defaultDuration: 3000,
  },
  {
    id: "anger",
    name: "怒气",
    icon: "💢",
    description: "愤怒符号",
    defaultDuration: 1500,
  },
  {
    id: "blush",
    name: "脸红",
    icon: "😊",
    description: "脸颊红晕",
    defaultDuration: 2000,
  },
  {
    id: "star",
    name: "星星",
    icon: "⭐",
    description: "闪烁的星星",
    defaultDuration: 1500,
  },
  {
    id: "check_mark",
    name: "对勾",
    icon: "✅",
    description: "完成对勾",
    defaultDuration: 1500,
  },
  {
    id: "warning",
    name: "警告",
    icon: "⚠️",
    description: "警告标志",
    defaultDuration: 2500,
  },
];

// Trigger info for display
export const TRIGGER_INFO: Array<{
  key: string;
  label: string;
  icon: string;
  required: boolean;
  description: string;
}> = [
  {
    key: "daily_1",
    label: "日常1",
    icon: "☀️",
    required: true,
    description: "宠物的基础待机状态，始终需要",
  },
  {
    key: "daily_2",
    label: "日常2",
    icon: "☀️",
    required: false,
    description: "第二日常状态，随机切换",
  },
  {
    key: "daily_3",
    label: "日常3",
    icon: "☀️",
    required: false,
    description: "第三日常状态，随机切换",
  },
  {
    key: "new_message",
    label: "新消息",
    icon: "💬",
    required: false,
    description: "收到聊天消息时触发",
  },
  {
    key: "new_task",
    label: "新任务",
    icon: "📋",
    required: false,
    description: "Jira 新任务分配时触发",
  },
  {
    key: "new_email",
    label: "新邮件",
    icon: "📧",
    required: false,
    description: "收到新邮件时触发",
  },
  {
    key: "task_in_progress",
    label: "任务进行中",
    icon: "⏳",
    required: false,
    description: "Jira 任务状态变为进行中",
  },
  {
    key: "task_completed",
    label: "任务已完成",
    icon: "✅",
    required: false,
    description: "Jira 任务完成时触发",
  },
  {
    key: "task_approaching_deadline",
    label: "任务将到期",
    icon: "⚠️",
    required: false,
    description: "任务截止时间临近（< 24h）",
  },
  {
    key: "task_overdue",
    label: "任务超时",
    icon: "🚨",
    required: false,
    description: "任务超过截止时间",
  },
];
