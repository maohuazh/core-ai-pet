## ADDED Requirements

### Requirement: Show radial menu on hover
The system SHALL display a radial quick-access menu when the mouse enters the character area. Menu items SHALL be dynamically generated from installed plugins.

#### Scenario: Menu appears on hover
- **WHEN** the mouse enters the character area
- **THEN** a radial menu appears around the character with all registered plugin menu items

#### Scenario: Menu disappears after mouse leave
- **WHEN** the mouse leaves the character and menu area
- **THEN** after 1 second delay, the menu fades out and hides

### Requirement: Menu animation
The menu SHALL animate with Fade In + Scale Up on show (200-300ms) and Fade Out on hide.

#### Scenario: Show animation
- **WHEN** the menu appears
- **THEN** menu items fade in and scale up over 200-300ms

#### Scenario: Hide animation
- **WHEN** the menu is dismissed
- **THEN** menu items fade out

### Requirement: Hover feedback on menu items
Each menu item SHALL scale to 110% and show a tooltip when the mouse hovers over it.

#### Scenario: Item hover effect
- **WHEN** the mouse hovers over a menu item
- **THEN** the item scales to 110% and displays its tooltip text

#### Scenario: Menu item click
- **WHEN** the user clicks a menu item
- **THEN** the corresponding plugin action is executed
