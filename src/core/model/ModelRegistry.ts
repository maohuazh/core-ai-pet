/**
 * Model Registry - manages all available Live2D pet models.
 */

export interface PetModelConfig {
  id: string;
  name: string;
  description?: string;
  modelUrl: string;
  cubismVersion: 2 | 4;
}

export class ModelRegistry {
  private models: PetModelConfig[] = [];
  private defaultId: string = "";

  register(config: PetModelConfig): void {
    this.models.push(config);
  }

  getAll(): PetModelConfig[] {
    return [...this.models];
  }

  getById(id: string): PetModelConfig | null {
    return this.models.find((m) => m.id === id) ?? null;
  }

  getDefault(): PetModelConfig | null {
    return this.getById(this.defaultId);
  }

  setDefault(id: string): void {
    this.defaultId = id;
  }
}

// Singleton instance with built-in models
export const modelRegistry = new ModelRegistry();

modelRegistry.register({
  id: "hiyori",
  name: "Hiyori",
  description: "Live2D official Cubism 4 sample, rich idle motions",
  modelUrl: "./models/Hiyori/Hiyori.model3.json",
  cubismVersion: 4,
});

modelRegistry.register({
  id: "mao",
  name: "Mao",
  description: "Live2D official Cubism 4 sample, 8 expressions + 6 tap motions",
  modelUrl: "./models/Mao/Mao.model3.json",
  cubismVersion: 4,
});

modelRegistry.register({
  id: "natori",
  name: "Natori",
  description: "Live2D official Cubism 4 sample, 11 expressions + 5 tap motions",
  modelUrl: "./models/Natori/Natori.model3.json",
  cubismVersion: 4,
});

modelRegistry.setDefault("hiyori");
