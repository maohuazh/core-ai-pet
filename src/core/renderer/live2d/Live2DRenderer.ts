import { Application } from "pixi.js";

export interface IRenderer {
  init(): Promise<void>;
  loadModel(modelPath: string): Promise<void>;
  destroy(): void;
}

export class Live2DRenderer implements IRenderer {
  private canvas: HTMLCanvasElement;
  private app: Application | null = null;
  private model: any = null;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
  }

  async init(): Promise<void> {
    // Check if Cubism Core is loaded
    if (!window.Live2DCubismCore) {
      console.warn("Live2D Cubism Core not loaded");
    } else {
      console.log("Live2D Cubism Core loaded");
    }

    // Initialize PixiJS Application
    this.app = new Application({
      view: this.canvas,
      width: 400,
      height: 400,
      backgroundAlpha: 0,
      antialias: true,
    });

    console.log("Live2DRenderer initialized, canvas size:", this.canvas.width, "x", this.canvas.height);
  }

  async loadModel(modelPath: string): Promise<void> {
    if (!this.app) {
      throw new Error("Renderer not initialized");
    }

    console.log("Loading Live2D model from:", modelPath);
    console.log("Live2DCubismCore available:", !!window.Live2DCubismCore);

    try {
      // Dynamic import
      const module = await import("pixi-live2d-display");
      console.log("pixi-live2d-display loaded");

      const { Live2DModel } = module;

      // Load the model
      this.model = await Live2DModel.from(modelPath, { autoInteract: false });

      console.log("Model loaded:", this.model);
      console.log("Model dimensions:", this.model.width, "x", this.model.height);
      console.log("Model bounds:", this.model.getBounds());

      // Disable interactivity to avoid PixiJS event system compatibility issues
      this.model.interactive = false;
      this.model.eventMode = "none";
      this.model.interactiveChildren = false;

      // Get canvas dimensions
      const targetWidth = this.app.screen.width;
      const targetHeight = this.app.screen.height;

      console.log("Canvas dimensions:", targetWidth, "x", targetHeight);

      // Calculate scale to fit model in canvas
      const modelWidth = this.model.width;
      const modelHeight = this.model.height;

      if (modelWidth === 0 || modelHeight === 0) {
        console.error("Model has zero size!");
        return;
      }

      const scaleX = targetWidth / modelWidth;
      const scaleY = targetHeight / modelHeight;
      const scale = Math.min(scaleX, scaleY) * 0.9;

      console.log("Scale factors:", scaleX, scaleY, "Final:", scale);

      // Apply scale
      this.model.scale.set(scale);
      this.model.anchor.set(0.5, 0.5);
      this.model.x = targetWidth / 2;
      this.model.y = targetHeight / 2;

      console.log("Model position after adjustment:", this.model.x, this.model.y);
      console.log("Model scale:", this.model.scale.x, this.model.scale.y);

      // Add to stage
      this.app.stage.addChild(this.model);

      console.log("Model added to stage");
      console.log("Stage children count:", this.app.stage.children.length);

      // Start idle motion after a short delay
      setTimeout(() => {
        if (this.model) {
          console.log("Trying to start idle animation...");
          // Try different ways to start animation
          try {
            // Method 1: Use motion method
            if (this.model.motion) {
              this.model.motion("idle", 0, 2);
              console.log("Started idle motion (method 1)");
            }
          } catch (e) {
            console.log("Method 1 failed:", e);
          }

          try {
            // Method 2: Check internal model
            if (this.model.internalModel?.motionManager) {
              const mm = this.model.internalModel.motionManager;
              console.log("Motion manager definitions:", Object.keys(mm.definitions || {}));
            }
          } catch (e) {
            console.log("Method 2 error:", e);
          }
        }
      }, 1000);

      console.log("Live2D model setup complete:", modelPath);
    } catch (error) {
      console.error("Failed to load Live2D model:", error);
      if (error instanceof Error) {
        console.error("Error name:", error.name);
        console.error("Error message:", error.message);
        console.error("Error stack:", error.stack);
      }
      throw error;
    }
  }

  destroy(): void {
    if (this.app) {
      this.app.destroy(true, { children: true, texture: true, baseTexture: true });
      this.app = null;
    }
    this.model = null;
  }
}
