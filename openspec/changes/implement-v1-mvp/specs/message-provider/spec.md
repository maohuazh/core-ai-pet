## ADDED Requirements

### Requirement: IMessageProvider interface definition
The Core layer SHALL define the `IMessageProvider` interface with: Id, Name, Icon, ConnectionState, GetUnreadCountAsync, GetRecentMessagesAsync, ConnectAsync, DisconnectAsync, MessageReceived event, ConnectionStateChanged event.

#### Scenario: Provider implements interface
- **WHEN** a plugin implements IMessageProvider
- **THEN** it provides unread count, recent message list, and connection management

### Requirement: Message data model
The system SHALL define `MessageItem` with: Id, ProviderId, Sender, Channel, Content, Timestamp, IsRead, DeepLink.

#### Scenario: Message item contains required fields
- **WHEN** a message provider returns message data
- **THEN** each MessageItem includes sender, channel, content, timestamp, and deep link

### Requirement: Message aggregation in UI
The system SHALL aggregate unread counts and recent messages from all active message providers.

#### Scenario: Aggregate unread count
- **WHEN** Slack has 5 unread, DingTalk has 3, and Feishu has 1
- **THEN** the Message menu badge shows 9 total unread

#### Scenario: Mixed message list
- **WHEN** the message panel is opened
- **THEN** messages from all providers are shown in a unified list sorted by timestamp

### Requirement: Message provider plugin skeletons
Plugin projects SHALL exist for: Slack, DingTalk (钉钉), Feishu (飞书), Teams, QQ, WeChat (企业微信). Each SHALL have plugin.json, entry class implementing IPlugin + IMessageProvider, and project structure ready for respective API integration.

#### Scenario: All message plugins compile
- **WHEN** the solution is built
- **THEN** all 6 message provider plugins compile and output to plugins/ directory

#### Scenario: Each plugin has valid manifest
- **WHEN** each plugin's plugin.json is read
- **THEN** it contains valid id, name, version, entryPoint, className, icon, and description fields
