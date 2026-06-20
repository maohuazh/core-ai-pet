## 1. PetHoverMenu 重写

- [x] 1.1 重写 `PetHoverMenu.vue` 菜单项为 6 个按钮：任务(task)、消息(message)、Jira(jira)、邮件(email)、Agent(agent)、设置(settings)
- [x] 1.2 实现环形布局：6 个按钮以 60° 间隔均匀分布在半径 100px 圆环上
- [x] 1.3 移除原有的 ❌ 关闭按钮
- [x] 1.4 每个按钮显示 emoji 图标 + hover 时显示中文标签

## 2. App.vue 动作处理

- [x] 2.1 更新 `handleMenuAction` 处理 6 个新动作：task、message、jira、email、agent、settings
- [x] 2.2 移除原有的 switchModel、menu、minimize、close 动作处理
- [x] 2.3 每个新动作暂用 alert 占位提示

## 3. 验证

- [ ] 3.1 鼠标悬停显示 6 个按钮环形排列，不遮挡模型
- [ ] 3.2 点击每个按钮触发对应 alert 提示
