## 1. Database Layer

- [x] 1.1 Create migration SQL for `model_action_mappings` table in `src-tauri/src/infrastructure/storage/mod.rs` with all required fields and constraints
- [ ] 1.2 Verify table creation on application startup by checking logs

## 2. Rust Backend - Action Mapping Commands

- [x] 2.1 Implement `get_action_mappings(model_id: String)` command in `src-tauri/src/commands/settings.rs` that queries all mappings for a model ordered by trigger_key
- [x] 2.2 Implement `save_action_mapping` command in `src-tauri/src/commands/settings.rs` that performs upsert (insert or update based on model_id + trigger_key uniqueness)
- [x] 2.3 Implement `delete_action_mapping(id: String)` command in `src-tauri/src/commands/settings.rs` that deletes a single mapping by id
- [x] 2.4 Add validation logic in `save_action_mapping` to reject daily_1 mappings with `use_default = 0` and no motion/expression configured
- [x] 2.5 Register all three commands in `src-tauri/src/main.rs` invoke handler
- [ ] 2.6 Test commands manually using Tauri dev tools to verify CRUD operations

## 3. Rust Backend - Resource Extraction

- [x] 3.1 Implement `get_live2d_motions(model_path: &str)` function in `src-tauri/src/commands/settings.rs` that parses `.model3.json` FileReferences.Motions section
- [x] 3.2 Implement `get_live2d_expressions(model_path: &str)` function in `src-tauri/src/commands/settings.rs` that parses `.model3.json` FileReferences.Expressions section
- [x] 3.3 Implement `get_sprite_motions(model_path: &str)` function in `src-tauri/src/commands/settings.rs` that parses `manifest.json` motions section
- [x] 3.4 Implement `get_sprite_expressions(model_path: &str)` function in `src-tauri/src/commands/settings.rs` that parses `manifest.json` expressions section
- [x] 3.5 Implement `get_available_motions(model_id: String)` Tauri command that determines model type and calls appropriate extraction function
- [x] 3.6 Implement `get_available_expressions(model_id: String)` Tauri command that determines model type and calls appropriate extraction function
- [x] 3.7 Register both resource extraction commands in `src-tauri/src/main.rs` invoke handler
- [ ] 3.8 Test resource extraction for Haru (Live2D) and PixelCat (Sprite) models to verify correct parsing

## 4. Frontend Service Layer

- [x] 4.1 Create `src/core/action/types.ts` with TypeScript interfaces for ActionMappingRecord, MappingFormData, MotionInfo, ExpressionInfo, AvailableEffect
- [x] 4.2 Create `src/core/action/effects.ts` with the 12 predefined effects array (AVAILABLE_EFFECTS)
- [x] 4.3 Create `src/core/action/actionMappingService.ts` with `loadMappings(modelId)` method that invokes `get_action_mappings`
- [x] 4.4 Add `saveMappings(modelId, formData[])` method to actionMappingService that validates daily_1 constraint and invokes `save_action_mapping` for each record
- [x] 4.5 Add `getAvailableMotions(modelId)` method to actionMappingService that invokes `get_available_motions`
- [x] 4.6 Add `getAvailableExpressions(modelId)` method to actionMappingService that invokes `get_available_expressions`
- [x] 4.7 Add helper methods `recordToFormData()` and `formDataToParams()` to actionMappingService for data conversion
- [x] 4.8 Add `createDefaultMappings()` method to actionMappingService that returns 10 default mappings with daily_1 use_default=1

## 5. Frontend UI Components - Selectors

- [x] 5.1 Create `src/components/action-mapping/MotionSelector.vue` with cascading dropdown for motion group and motion name, loading options from available motions
- [x] 5.2 Create `src/components/action-mapping/ExpressionSelector.vue` with dropdown for expression name, loading options from available expressions
- [x] 5.3 Create `src/components/action-mapping/EffectSelector.vue` with dropdown for effect selection from AVAILABLE_EFFECTS, including duration and position fields
- [x] 5.4 Test each selector component independently with mock data to verify correct rendering and event emission

## 6. Frontend UI Components - Mapping Row and Panel

- [x] 6.1 Create `src/components/action-mapping/MappingRow.vue` with expandable card for single trigger scenario, including use_default checkbox, motion/expression/effect selectors with enable checkboxes, and preview button
- [x] 6.2 Add logic to MappingRow to disable selectors when use_default is checked
- [x] 6.3 Add logic to MappingRow to emit preview event when preview button is clicked
- [x] 6.4 Create `src/components/action-mapping/ActionMappingPanel.vue` with header (back button, model name, save/reset buttons), 10 MappingRow instances, and preview area placeholder
- [x] 6.5 Add data loading logic to ActionMappingPanel to fetch mappings, available motions, and available expressions on mount
- [x] 6.6 Add save logic to ActionMappingPanel that calls actionMappingService.saveMappings and shows success toast
- [x] 6.7 Add reset logic to ActionMappingPanel that reloads data from service and discards unsaved changes
- [x] 6.8 Add unsaved changes detection to ActionMappingPanel and confirmation dialog when navigating away

## 7. Settings Panel Integration

- [x] 7.1 Add "⚙ 动作映射" button to each model card in `src/components/settings/modules/ModelConfigModule.vue`
- [x] 7.2 Add navigation logic to ModelConfigModule to switch between model list view and ActionMappingPanel when button is clicked
- [x] 7.3 Test navigation flow: model list → action mapping panel → back to model list

## 8. Preview Functionality

- [x] 8.1 Add preview event listener in `src/App.vue` that listens for `preview-action-mapping` event from settings window
- [x] 8.2 Implement preview handler in App.vue that calls renderer.playMotion() and renderer.playExpression() based on event payload
- [x] 8.3 Update ActionMappingPanel preview button to emit `preview-action-mapping` Tauri event with modelId, motionGroup, motionName, expressionName, effectName
- [x] 8.4 Test preview functionality by clicking preview button in mapping row and verifying pet window plays the configured motion/expression

## 9. Runtime Trigger Integration

- [x] 9.1 Create `src/core/events/triggerHandler.ts` with TriggerHandler class that listens for `external-event` on eventBus
- [x] 9.2 Implement `eventTypeToTriggerKey()` mapping in triggerHandler for all 7 external event types
- [x] 9.3 Implement `fireTrigger(triggerKey)` method in triggerHandler that queries database for mapping and executes motion/expression/effect
- [x] 9.4 Add daily-tick timer to triggerHandler that fires every 5 minutes and randomly triggers daily_1/daily_2/daily_3 with 70%/20%/10% weights
- [x] 9.5 Initialize triggerHandler in `src/main.ts` to start listening for events
- [x] 9.6 Test trigger handler by emitting mock external events and verifying correct motion/expression playback

## 10. Testing and Polish

- [ ] 10.1 Test complete flow: configure mapping for a model → save → trigger event → verify motion/expression plays
- [ ] 10.2 Test daily_1 mandatory validation by attempting to save daily_1 with no configuration
- [ ] 10.3 Test model switching by configuring mappings for two models and switching between them
- [ ] 10.4 Add empty state guidance when a model has no available motions or expressions
- [ ] 10.5 Verify database persistence by configuring mappings, restarting app, and confirming mappings are retained
