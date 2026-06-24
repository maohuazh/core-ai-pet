import { describe, it, expect } from "vitest";
import { ModelRegistry, type PetModelConfig } from "../ModelRegistry";

describe("ModelRegistry", () => {
  it("registers and retrieves a Live2D model", () => {
    const registry = new ModelRegistry();
    const config: PetModelConfig = {
      id: "test-live2d",
      name: "TestModel",
      type: "live2d",
      modelUrl: "./models/test.model3.json",
      cubismVersion: 4,
    };
    registry.register(config);
    expect(registry.getById("test-live2d")).toEqual(config);
  });

  it("registers and retrieves a Sprite model", () => {
    const registry = new ModelRegistry();
    const config: PetModelConfig = {
      id: "test-sprite",
      name: "PixelCat",
      type: "sprite",
      modelUrl: "./models/pixel-cat/manifest.json",
      manifestPath: "./models/pixel-cat/manifest.json",
    };
    registry.register(config);
    const result = registry.getById("test-sprite");
    expect(result).not.toBeNull();
    expect(result!.type).toBe("sprite");
    expect(result!.manifestPath).toBe("./models/pixel-cat/manifest.json");
  });

  it("returns all registered models including mixed types", () => {
    const registry = new ModelRegistry();
    registry.register({ id: "a", name: "A", type: "live2d", modelUrl: "a.json", cubismVersion: 4 });
    registry.register({ id: "b", name: "B", type: "sprite", modelUrl: "b.json", manifestPath: "b.json" });
    const all = registry.getAll();
    expect(all).toHaveLength(2);
    expect(all.map(m => m.type)).toContain("live2d");
    expect(all.map(m => m.type)).toContain("sprite");
  });

  it("returns null for non-existent model", () => {
    const registry = new ModelRegistry();
    expect(registry.getById("nonexistent")).toBeNull();
  });

  it("sets and gets default model", () => {
    const registry = new ModelRegistry();
    registry.register({ id: "a", name: "A", type: "live2d", modelUrl: "a.json", cubismVersion: 4 });
    registry.setDefault("a");
    expect(registry.getDefault()?.id).toBe("a");
  });

  it("returns null default when not set", () => {
    const registry = new ModelRegistry();
    expect(registry.getDefault()).toBeNull();
  });
});
