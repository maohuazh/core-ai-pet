/**
 * PetStore - reactive state management for the current pet model.
 */

import { ref, type Ref } from "vue";
import { modelRegistry, type PetModelConfig } from "./ModelRegistry";

class PetStore {
  currentModel: Ref<PetModelConfig>;
  models: Ref<PetModelConfig[]>;

  constructor() {
    const defaultModel = modelRegistry.getDefault();
    if (!defaultModel) {
      throw new Error("No default model registered in ModelRegistry");
    }
    this.currentModel = ref(defaultModel);
    this.models = ref(modelRegistry.getAll());
  }

  setCurrentModel(model: PetModelConfig): void {
    this.currentModel.value = model;
  }

  switchToNextModel(): void {
    const all = this.models.value;
    const currentId = this.currentModel.value.id;
    const currentIndex = all.findIndex((m) => m.id === currentId);
    const nextIndex = (currentIndex + 1) % all.length;
    this.setCurrentModel(all[nextIndex]);
  }
}

export const petStore = new PetStore();
