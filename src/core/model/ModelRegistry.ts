/**
 * Model Registry - manages all available pet models (Live2D and Sprite).
 */

export type ModelType = "live2d" | "sprite";

export interface PetModelConfig {
  id: string;
  name: string;
  type: ModelType;
  description?: string;
  modelUrl: string;
  manifestPath?: string;
  cubismVersion?: 2 | 4;
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
  id: "haru",
  name: "Haru",
  type: "live2d",
  description: "Original CDN model, anime girl with idle and happy motions",
  modelUrl: "https://cdn.jsdelivr.net/gh/guansss/pixi-live2d-display/test/assets/haru/haru_greeter_t03.model3.json",
  cubismVersion: 4,
});

modelRegistry.register({
  id: "hiyori",
  name: "Hiyori",
  type: "live2d",
  description: "Live2D official Cubism 4 sample, rich idle motions",
  modelUrl: "./models/Hiyori/Hiyori.model3.json",
  cubismVersion: 4,
});

modelRegistry.register({
  id: "mao",
  name: "Mao",
  type: "live2d",
  description: "Live2D official Cubism 4 sample, 8 expressions + 6 tap motions",
  modelUrl: "./models/Mao/Mao.model3.json",
  cubismVersion: 4,
});

modelRegistry.register({
  id: "natori",
  name: "Natori",
  type: "live2d",
  description: "Live2D official Cubism 4 sample, 11 expressions + 5 tap motions",
  modelUrl: "./models/Natori/Natori.model3.json",
  cubismVersion: 4,
});

modelRegistry.setDefault("panda"); // TEMP: testing sprite model — revert to "haru" after

// Sample Sprite models
modelRegistry.register({
  id: "pixel-cat",
  name: "PixelCat",
  type: "sprite",
  description: "A pixel art cat with 8-direction frame animations",
  modelUrl: "./models/pixel-cat/manifest.json",
  manifestPath: "./models/pixel-cat/manifest.json",
});

modelRegistry.register({
  id: "arisa",
  name: "Arisa",
  type: "sprite",
  description: "A pixel art character from BANDORI desktop pet, 9 animation states",
  modelUrl: "./models/arisa/manifest.json",
  manifestPath: "./models/arisa/manifest.json",
});

modelRegistry.register({
  id: "panda",
  name: "Panda",
  type: "sprite",
  description: "A soft, healing panda companion with round eyes, tiny paws, and gentle calming poses.",
  modelUrl: "./models/panda/manifest.json",
  manifestPath: "./models/panda/manifest.json",
});
