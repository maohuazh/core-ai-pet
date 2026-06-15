## ADDED Requirements

### Requirement: IEmailProvider interface definition
The Core layer SHALL define the `IEmailProvider` interface with: Id, Name, Icon, ConnectionState, GetUnreadCountAsync, GetRecentEmailsAsync, ConnectAsync, DisconnectAsync, EmailReceived event, ConnectionStateChanged event.

#### Scenario: Provider implements interface
- **WHEN** a plugin implements IEmailProvider
- **THEN** it provides unread count, recent email list, and connection management

### Requirement: Email data model
The system SHALL define `EmailItem` with: Id, ProviderId, From, Subject, Preview, Timestamp, IsRead, IsImportant, DeepLink.

#### Scenario: Email item contains required fields
- **WHEN** an email provider returns email data
- **THEN** each EmailItem includes sender, subject, preview text, timestamp, and a deep link to open in the source app

### Requirement: Email aggregation in UI
The system SHALL aggregate unread counts and recent emails from all active email providers.

#### Scenario: Aggregate unread count
- **WHEN** Outlook has 3 unread and Gmail has 5 unread
- **THEN** the Email menu badge shows 8 total unread

#### Scenario: Deep link opens source app
- **WHEN** the user clicks an email item
- **THEN** the system opens the corresponding email in the source application via DeepLink

### Requirement: Outlook plugin skeleton
A plugin project `CoreAIpet.Plugin.Email.Outlook` SHALL exist with plugin.json, entry class implementing IPlugin + IEmailProvider, and project structure ready for Microsoft Graph API integration.

#### Scenario: Plugin compiles
- **WHEN** the solution is built
- **THEN** the Outlook plugin compiles and outputs to plugins/ directory

### Requirement: Gmail plugin skeleton
A plugin project `CoreAIpet.Plugin.Email.Gmail` SHALL exist with the same structure, ready for Gmail API integration.

#### Scenario: Plugin compiles
- **WHEN** the solution is built
- **THEN** the Gmail plugin compiles successfully

### Requirement: IMAP plugin skeleton
A plugin project `CoreAIpet.Plugin.Email.IMAP` SHALL exist with the same structure, ready for generic IMAP server integration.

#### Scenario: Plugin compiles
- **WHEN** the solution is built
- **THEN** the IMAP plugin compiles successfully
