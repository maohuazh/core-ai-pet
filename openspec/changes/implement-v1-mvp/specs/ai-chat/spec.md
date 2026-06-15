## ADDED Requirements

### Requirement: Open chat via multiple triggers
The system SHALL open the chat input via: clicking the character, clicking the Task menu item, or pressing Alt+Space.

#### Scenario: Click character opens chat
- **WHEN** the user clicks on the character
- **THEN** the chat input bubble appears above the character

#### Scenario: Alt+Space opens chat
- **WHEN** the user presses Alt+Space globally
- **THEN** the chat input bubble appears

### Requirement: Chat input with multiline support
The system SHALL provide a chat input area supporting up to 4000 characters, Chinese and English text, and multiline input. Enter SHALL send the message. Shift+Enter SHALL insert a newline.

#### Scenario: Send message with Enter
- **WHEN** the user types text and presses Enter
- **THEN** the message is sent to the AI and the input is cleared

#### Scenario: Newline with Shift+Enter
- **WHEN** the user presses Shift+Enter
- **THEN** a newline is inserted at the cursor position without sending

#### Scenario: Character limit enforcement
- **WHEN** the user attempts to type beyond 4000 characters
- **THEN** the system SHALL prevent further input

### Requirement: AI response display with streaming
The system SHALL display AI responses in a bubble near the character, supporting streaming (token-by-token) display.

#### Scenario: Streaming response display
- **WHEN** the AI returns a streaming response
- **THEN** tokens are displayed incrementally in the response bubble

#### Scenario: Long text collapse
- **WHEN** the AI response exceeds 300 characters
- **THEN** the response is collapsed with a "View more" expand button

### Requirement: AI multi-backend support
The system SHALL support OpenAI, Azure OpenAI, and Ollama as AI backends. The user SHALL be able to switch between configured backends.

#### Scenario: Send to OpenAI
- **WHEN** OpenAI is the active provider and the user sends a message
- **THEN** the request is sent to the configured OpenAI endpoint

#### Scenario: Switch to Ollama
- **WHEN** the user changes the active provider to Ollama in settings
- **THEN** subsequent messages are sent to the Ollama endpoint
