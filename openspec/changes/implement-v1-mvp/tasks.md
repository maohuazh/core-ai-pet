## 1. Core Infrastructure Services

- [x] 1.1 Implement EventBus (IEventBus) — in-memory publish/subscribe with thread safety
- [x] 1.2 Implement LogService (ILogService) — Serilog wrapper with ring buffer for debug panel
- [x] 1.3 Implement JsonConfigStore — atomic JSON file read/write with file locking
- [x] 1.4 Implement ConfigService (IConfigService) — typed config sections, change notification, save/reload
- [x] 1.5 Implement PositionService (IPositionService) — load/save window position to config.json
- [x] 1.6 Complete CompositionRoot — register all services in DI container

## 2. Win32 Integration Layer

- [x] 2.1 Create NativeMethods partial classes — P/Invoke declarations for window style, layered window, hooks, tray
- [x] 2.2 Implement WindowStyleManager — borderless + transparent + always-on-top setup via SetWindowLong/DwmExtendFrameIntoClientArea
- [x] 2.3 Implement ClickThroughManager — toggle WS_EX_TRANSPARENT for click-through mode
- [x] 2.4 Implement WindowService (IWindowService) — Show/Hide/ClickThrough/AlwaysOnTop/Scale/Opacity
- [x] 2.5 Implement global hotkey registration — RegisterHotKey for Alt+Space and Ctrl+Alt+P

## 3. Live2D Bridge (C++ DLL)

- [ ] 3.1 Set up Live2D Cubism Native SDK as git submodule in vendor/
- [ ] 3.2 Implement cubism_model.cpp — load .moc3 files, create/manage model instances
- [ ] 3.3 Implement cubism_renderer.cpp — D3D11 initialization, render to HWND, swap chain resize
- [ ] 3.4 Implement cubism_animation.cpp — animation state machine, motion group playback
- [ ] 3.5 Implement eye_tracking.cpp — convert normalized coordinates to ParamEyeBallX/Y
- [ ] 3.6 Complete bridge_api.cpp — wire all exports to implementations, add error handling
- [ ] 3.7 Build and verify DLL exports — compile with CMake, verify all 11 functions exported

## 4. Live2D Rendering (C# Side)

- [x] 4.1 Create Live2DBridgeNative.cs — P/Invoke declarations matching C++ exports
- [x] 4.2 Create NativeStructures.cs — shared structs with LayoutKind.Sequential
- [x] 4.3 Implement Live2DBridgeWrapper — safe wrapper with exception translation, thread safety, lifetime management
- [x] 4.4 Implement Live2DHostControl — HwndHost subclass, create child HWND, pass to bridge
- [x] 4.5 Implement FrameTimer — 60fps render loop via CompositionTarget.Rendering
- [x] 4.6 Implement Live2DRenderHost — orchestrate bridge init, model load, render loop, cleanup
- [x] 4.7 Implement StateAnimationMapper — map CharacterState enum to animation group/name pairs
- [x] 4.8 Implement EyeFollowController — get mouse position, normalize, call Bridge_SetEyeTarget with angle limits
- [x] 4.9 Implement CharacterController (ICharacterController) — state machine with valid transitions, min duration guard

## 5. Desktop Interaction

- [x] 5.1 Implement WindowDragBehavior — left mouse drag at 60fps with capture
- [x] 5.2 Implement position save on drag release — call IPositionService.SaveAsync
- [x] 5.3 Implement position restore on startup — load from IPositionService, set window Left/Top
- [x] 5.4 Implement ClickThroughBehavior — Ctrl+Alt+P toggle via IWindowService
- [x] 5.5 Implement AutoHideBehavior — mouse leave triggers 1s delayed hide for menu

## 6. Hover Radial Menu

- [x] 6.1 Implement RadialMenuControl — Canvas-based circular layout, dynamic item count
- [x] 6.2 Implement RadialMenuViewModel — get menu items from PluginManager, compute positions
- [x] 6.3 Implement menu show/hide animations — Fade In + Scale Up (200-300ms), Fade Out
- [x] 6.4 Implement hover feedback — 110% scale + tooltip on item hover
- [x] 6.5 Wire menu to MainWindow — show on MouseEnter, hide on MouseLeave with 1s delay

## 7. AI Chat System

- [x] 7.1 Implement OpenAIService (IAIService) — Chat Completions API with streaming via official SDK
- [x] 7.2 Implement AzureOpenAIService (IAIService) — Azure OpenAI with deployment name
- [x] 7.3 Implement OllamaService (IAIService) — REST API with /api/chat endpoint and streaming
- [x] 7.4 Implement AIServiceProvider (IAIServiceProvider) — route to active backend, switch provider
- [x] 7.5 Implement ChatSessionManager — maintain conversation history, token trimming
- [x] 7.6 Implement SystemPromptBuilder — build system prompt with character persona
- [x] 7.7 Implement ChatViewModel — input handling, send command, streaming display, state transitions
- [x] 7.8 Implement ChatBubbleWindow — transparent popup window with input area and response display
- [x] 7.9 Implement long text handling — collapse >300 chars with "View more" expand button
- [x] 7.10 Wire Alt+Space global hotkey to open chat

## 8. Plugin Framework

- [x] 8.1 Implement PluginManager — scan plugins/ directory, read plugin.json, discover plugins
- [x] 8.2 Implement PluginLoadContext — AssemblyLoadContext subclass with isCollectible, shared Core reference
- [x] 8.3 Implement plugin instantiation — load DLL via PluginLoadContext, create instance via className reflection
- [x] 8.4 Implement PluginContext (IPluginContext/IPluginHost) — provide config, logger, event bus, AI, data directory
- [x] 8.5 Implement PluginHostService (IHostedService) — orchestrate Load → Activate → Execute lifecycle
- [x] 8.6 Implement graceful shutdown — Deactivate → Unload all plugins on app exit
- [x] 8.7 Implement error isolation — catch plugin exceptions, log, mark as Error, continue

## 9. System Tray

- [x] 9.1 Implement TrayIconService — create tray icon, handle balloon notifications
- [x] 9.2 Implement tray right-click menu — Show/Hide/Settings/Restart/Exit options
- [x] 9.3 Implement double-click to restore — show character window at last position
- [x] 9.4 Implement TrayViewModel — commands for each menu action

## 10. Settings UI

- [x] 10.1 Create SettingsWindow.xaml — Tab layout: Appearance / System / AI / Plugins
- [x] 10.2 Implement Appearance tab — scale slider (50-200%), opacity slider (20-100%), theme toggle
- [x] 10.3 Implement System tab — auto-start checkbox, always-on-top checkbox, click-through checkbox
- [x] 10.4 Implement AI tab — provider dropdown, endpoint/apiKey/model fields per provider, test connection button
- [x] 10.5 Implement SettingsViewModel — bind to IConfigService, save on change
- [x] 10.6 Implement theme switching — LightTheme.xaml / DarkTheme.xaml resource dictionaries, ThemeManager

## 11. Debug Panel

- [x] 11.1 Create DebugWindow.xaml — layout for FPS/CPU/Memory metrics, plugin list, log viewer
- [x] 11.2 Implement PerformanceMonitor — collect FPS (from FrameTimer), CPU, memory (Process.GetCurrentProcess)
- [x] 11.3 Implement DebugViewModel — bind metrics, plugin states, log entries
- [x] 11.4 Wire log viewer to LogService.GetRecentLogs + LogAdded event

## 12. Provider Plugin Projects

- [x] 12.1 Create CoreAIpet.Plugin.Jira — JiraPlugin (IPlugin), JiraApiClient stub, JiraIssue model, plugin.json
- [x] 12.2 Create CoreAIpet.Plugin.Email.Outlook — OutlookPlugin (IPlugin + IEmailProvider), OutlookClient stub
- [x] 12.3 Create CoreAIpet.Plugin.Email.Gmail — GmailPlugin (IPlugin + IEmailProvider), GmailClient stub
- [x] 12.4 Create CoreAIpet.Plugin.Email.IMAP — ImapPlugin (IPlugin + IEmailProvider), ImapClient stub
- [x] 12.5 Create CoreAIpet.Plugin.Message.Slack — SlackPlugin (IPlugin + IMessageProvider), SlackClient stub
- [x] 12.6 Create CoreAIpet.Plugin.Message.DingTalk — DingTalkPlugin (IPlugin + IMessageProvider), DingTalkClient stub
- [x] 12.7 Create CoreAIpet.Plugin.Message.Feishu — FeishuPlugin (IPlugin + IMessageProvider), FeishuClient stub
- [x] 12.8 Create CoreAIpet.Plugin.Message.Teams — TeamsPlugin (IPlugin + IMessageProvider), TeamsClient stub
- [x] 12.9 Create CoreAIpet.Plugin.Message.QQ — QQPlugin (IPlugin + IMessageProvider), QQClient stub
- [x] 12.10 Create CoreAIpet.Plugin.Message.WeChat — WeChatPlugin (IPlugin + IMessageProvider), WeChatClient stub
- [x] 12.11 Add all plugin projects to CoreAIpet.sln
- [x] 12.12 Verify all plugins compile and DLLs output to plugins/ directory

## 13. App Integration & Startup

- [x] 13.1 Wire MainWindow.xaml — integrate Live2DHostControl, RadialMenuControl, chat trigger
- [x] 13.2 Wire App.xaml.cs — full Generic Host startup: config, services, position restore, main window show
- [x] 13.3 Implement graceful shutdown — save position, stop plugins, dispose host
- [ ] 13.4 End-to-end smoke test — launch app, verify character displays, hover menu works, chat opens, plugins load
