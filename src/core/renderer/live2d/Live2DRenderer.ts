import { Application, Ticker } from "pixi.js";
import { Live2DModel } from "pixi-live2d-display/cubism4";

export interface MotionGroup {
  name: string;
  count: number;
}

export interface ExpressionInfo {
  name: string;
}

export interface IRenderer {
  init(): Promise<void>;
  loadModel(modelPath: string): Promise<void>;
  playMotion(group: string, index?: number): Promise<void>;
  playExpression(nameOrIndex: string | number): Promise<void>;
  getMotionGroups(): MotionGroup[];
  getExpressions(): ExpressionInfo[];
  destroy(): void;
}

export class Live2DRenderer implements IRenderer {
  private container: HTMLElement;
  private app: Application | null = null;
  private model: any = null;
  private width: number;
  private height: number;

  constructor(container: HTMLElement, width: number, height: number) {
    this.container = container;
    this.width = width;
    this.height = height;
  }

  async init(): Promise<void> {
    if (!window.Live2DCubismCore) {
      console.warn("Live2D Cubism Core not loaded");
    } else {
      console.log("Live2D Cubism Core loaded");
    }

    this.app = new Application({
      width: this.width,
      height: this.height,
      backgroundAlpha: 0,
      antialias: true,
    });

    // Append PixiJS canvas to container
    if (this.app.view) {
      this.app.view.style.width = "100%";
      this.app.view.style.height = "100%";
      this.container.appendChild(this.app.view);
    }

    // Register Ticker for Live2D model animation
    (Live2DModel as any).registerTicker(Ticker);
    console.log("Live2DModel Ticker registered");

    console.log("Live2DRenderer initialized, canvas size:", this.width, "x", this.height);
  }

  async loadModel(modelPath: string): Promise<void> {
    if (!this.app) {
      throw new Error("Renderer not initialized");
    }

    const app = this.app;

    // Destroy existing model if present
    if (this.model) {
      console.log("Removing old model before loading new one");
      app.stage.removeChild(this.model);
      this.model.destroy(true);
      this.model = null;
    }

    console.log("Loading Live2D model from:", modelPath);

    try {
      console.log("Live2DModel class:", Live2DModel);

      this.model = await Live2DModel.from(modelPath, { autoInteract: false });
      console.log("Model instance created");

      console.log("Model loaded:", this.model);
      console.log("Model dimensions:", this.model.width, "x", this.model.height);

      // Disable interactivity to avoid PixiJS event system compatibility issues
      this.model.interactive = false;
      this.model.eventMode = "none";
      this.model.interactiveChildren = false;

      const targetWidth = app.screen.width;
      const targetHeight = app.screen.height;

      const modelWidth = this.model.width;
      const modelHeight = this.model.height;

      if (modelWidth === 0 || modelHeight === 0) {
        console.error("Model has zero size!");
        return;
      }

      const scaleX = targetWidth / modelWidth;
      const scaleY = targetHeight / modelHeight;
      const scale = Math.min(scaleX, scaleY) * 0.75;

      this.model.scale.set(scale);
      this.model.anchor.set(0.5, 0.5);
      this.model.x = targetWidth / 2;
      this.model.y = targetHeight / 2;

      // Remove any remaining Live2D models from stage
      const existingModels = app.stage.children.filter(c => c.constructor.name === "Live2DModel");
      existingModels.forEach(m => {
        app.stage.removeChild(m);
        (m as any).destroy(true);
      });

      app.stage.addChild(this.model);
      console.log("Model added to stage");
      console.log("Stage children count:", app.stage.children.length);
      console.log("Model visible:", this.model.visible);
      console.log("Model alpha:", this.model.alpha);
      console.log("Model scale:", this.model.scale.x, this.model.scale.y);
      console.log("Model position:", this.model.x, this.model.y);

      // Auto-play idle motion after model loads
      setTimeout(async () => {
        if (this.model) {
          try {
            const groups = this.getMotionGroups();
            console.log("Available motion groups:", groups.map(g => `${g.name}(${g.count})`).join(", "));

            const idleGroup = groups.find(g => g.name.toLowerCase().includes("idle"));
            if (idleGroup) {
              await this.playMotion(idleGroup.name, 0);
              console.log(`Auto-played idle motion: ${idleGroup.name}`);
            } else if (groups.length > 0) {
              await this.playMotion(groups[0].name, 0);
              console.log(`No idle group found, played first group: ${groups[0].name}`);
            }
          } catch (e) {
            console.error("Failed to start idle motion:", e);
          }
        }
      }, 1000);

      console.log("Live2D model setup complete:", modelPath);
    } catch (error) {
      console.error("Failed to load Live2D model:", error);
      throw error;
    }
  }

  async playMotion(group: string, index?: number): Promise<void> {
    if (!this.model) {
      console.warn("playMotion: no model loaded");
      return;
    }

    const groups = this.getMotionGroups();
    const targetGroup = groups.find(g => g.name === group);

    if (!targetGroup) {
      console.warn(`playMotion: motion group "${group}" not found. Available: ${groups.map(g => g.name).join(", ")}`);
      return;
    }

    const motionIndex = index ?? Math.floor(Math.random() * targetGroup.count);

    try {
      await this.model.motion(group, motionIndex);
      console.log(`Motion played: ${group}[${motionIndex}]`);
    } catch (e) {
      console.warn(`playMotion: failed to play ${group}[${motionIndex}]:`, e);
    }
  }

  async playExpression(nameOrIndex: string | number): Promise<void> {
    if (!this.model) {
      console.warn("playExpression: no model loaded");
      return;
    }

    const expressions = this.getExpressions();
    if (expressions.length === 0) {
      console.warn("playExpression: current model has no expressions");
      return;
    }

    try {
      if (typeof nameOrIndex === "number") {
        await this.model.expression(nameOrIndex);
        console.log(`Expression played by index: ${nameOrIndex}`);
      } else {
        const target = expressions.find(e => e.name === nameOrIndex);
        if (!target) {
          console.warn(`playExpression: expression "${nameOrIndex}" not found. Available: ${expressions.map(e => e.name).join(", ")}`);
          return;
        }
        await this.model.expression(nameOrIndex);
        console.log(`Expression played: ${nameOrIndex}`);
      }
    } catch (e) {
      console.warn(`playExpression: failed to play "${nameOrIndex}":`, e);
    }
  }

  getMotionGroups(): MotionGroup[] {
    if (!this.model?.internalModel?.motionManager?.definitions) {
      return [];
    }

    const definitions = this.model.internalModel.motionManager.definitions;
    return Object.entries(definitions).map(([name, motions]) => ({
      name,
      count: (motions as any[]).length,
    }));
  }

  getExpressions(): ExpressionInfo[] {
    if (!this.model?.internalModel?.settings?.expressions) {
      return [];
    }

    return this.model.internalModel.settings.expressions.map((e: any) => ({
      name: e.Name || e.name || "",
    }));
  }

  destroy(): void {
    if (this.app) {
      // Remove canvas from DOM
      if (this.app.view && this.app.view.parentNode) {
        this.app.view.parentNode.removeChild(this.app.view);
      }
      this.app.destroy(true, { children: true, texture: true, baseTexture: true });
      this.app = null;
    }
    this.model = null;
  }
}
