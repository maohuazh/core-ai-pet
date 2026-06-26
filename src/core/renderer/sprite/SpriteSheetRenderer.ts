/**
 * SpriteSheetRenderer - Canvas 2D based frame animation renderer.
 * Implements IRenderer interface for sprite sheet playback.
 */

import type { MotionGroup, ExpressionInfo, IRenderer } from "../live2d/Live2DRenderer";
import type { SpriteSheetManifest } from "../../sprite/types";
import { SpriteSheetAnimationEngine } from "../../sprite/animationEngine";
import { degreeToDirection } from "../../sprite/directionMapper";

export class SpriteSheetRenderer implements IRenderer {
  private container: HTMLElement;
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private manifest: SpriteSheetManifest | null = null;
  private engine: SpriteSheetAnimationEngine | null = null;
  private spriteImage: HTMLImageElement | null = null;
  private expressionImage: HTMLImageElement | null = null;
  private animationFrameId: number = 0;
  private lastTimestamp: number = 0;
  private scale: number = 1;
  private width: number;
  private height: number;
  private manifestBaseUrl: string = "./";
  private spriteLoadFailed: boolean = false;
  private debugMode: boolean = false;

  /** Max display size in pixels (constrains sprite to fit within button area) */
  private maxDisplaySize: number = 120;

  // Performance monitoring
  private frameCount: number = 0;
  private fpsAccumulator: number = 0;
  private currentFps: number = 0;

  constructor(container: HTMLElement, width: number, height: number) {
    this.container = container;
    this.width = width;
    this.height = height;
  }

  async init(): Promise<void> {
    this.canvas = document.createElement("canvas");
    // Canvas fills container like Live2D
    this.canvas.width = this.width;
    this.canvas.height = this.height;
    this.canvas.style.width = "100%";
    this.canvas.style.height = "100%";
    this.container.appendChild(this.canvas);

    this.ctx = this.canvas.getContext("2d");
    if (!this.ctx) {
      throw new Error("Failed to get Canvas 2D context");
    }

    console.log("SpriteSheetRenderer initialized, canvas size:", this.width, "x", this.height);
  }

  async loadModel(modelPath: string): Promise<void> {
    const manifestUrl = modelPath;
    this.manifestBaseUrl = manifestUrl.substring(0, manifestUrl.lastIndexOf("/") + 1);

    // Load and parse manifest
    let response: Response;
    try {
      response = await fetch(manifestUrl);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
    } catch (e) {
      console.error(`Failed to load manifest: ${manifestUrl}`, e);
      throw new Error(`Failed to load manifest: ${manifestUrl}`);
    }

    const raw = await response.json();

    try {
      const { parseManifest } = await import("../../sprite/manifestParser");
      this.manifest = parseManifest(raw);
    } catch (e) {
      console.error("Failed to parse manifest:", e);
      throw e;
    }

    // Load sprite sheet image with graceful degradation
    const imageUrl = this.manifestBaseUrl + this.manifest.spritesheet.image;
    try {
      this.spriteImage = await this.loadImage(imageUrl);
      this.spriteLoadFailed = false;
      console.log(`Sprite sheet loaded: ${this.manifest.spritesheet.image} (${this.spriteImage.width}x${this.spriteImage.height})`);
    } catch (e) {
      console.error(`Failed to load sprite sheet image: ${imageUrl}`, e);
      this.spriteLoadFailed = true;
      // Don't throw - allow renderer to show placeholder
    }

    // Initialize animation engine
    this.engine = new SpriteSheetAnimationEngine(this.manifest);

    // Auto-return to Idle when a non-looping animation completes
    this.engine.onComplete(() => {
      const defaultState = this.manifest?.defaults?.state || "Idle";
      console.log(`SpriteSheetRenderer: animation complete, returning from ${this.engine!.getStateName()} to ${defaultState}`);
      if (this.engine!.getStateName() !== defaultState) {
        this.engine!.setState(defaultState);
      }
    });

    // Canvas stays at container dimensions (like Live2D).
    // Sprite is scaled to fit within the canvas, preserving aspect ratio.
    if (this.canvas) {
      this.canvas.width = this.width;
      this.canvas.height = this.height;
    }

    // Start render loop
    this.startRenderLoop();

    console.log("SpriteSheet model loaded:", this.manifest.meta.name,
      this.manifest.animated === false ? "(static)" : "(animated)");
  }

  async playMotion(group: string, _index?: number): Promise<void> {
    if (!this.engine || !this.manifest) {
      console.warn("playMotion: renderer not initialized");
      return;
    }

    const motion = this.manifest.motions?.[group];
    if (!motion) {
      console.warn(`playMotion: motion "${group}" not found`);
      return;
    }

    console.log(`SpriteSheetRenderer: setState(${motion.state}) for motion "${group}", current state: ${this.engine.getStateName()}`);
    this.engine.setState(motion.state);
  }

  async playExpression(nameOrIndex: string | number): Promise<void> {
    if (!this.manifest) {
      console.warn("playExpression: renderer not initialized");
      return;
    }

    const expressions = this.getExpressions();
    if (expressions.length === 0) {
      console.warn("playExpression: no expressions available");
      return;
    }

    const name = typeof nameOrIndex === "number" ? expressions[nameOrIndex]?.name : nameOrIndex;
    if (!name) {
      console.warn(`playExpression: expression "${nameOrIndex}" not found`);
      return;
    }

    const exprDef = this.manifest.expressions?.[name];
    if (!exprDef || !exprDef.overlay) {
      console.warn(`playExpression: no overlay for "${name}"`);
      return;
    }

    // Load expression image with graceful degradation
    try {
      this.expressionImage = await this.loadImage(this.manifestBaseUrl + exprDef.overlay);
    } catch (e) {
      console.warn(`Failed to load expression overlay "${name}": ${exprDef.overlay}`, e);
      this.expressionImage = null;
      // Continue rendering without expression overlay
    }
  }

  getMotionGroups(): MotionGroup[] {
    if (!this.manifest?.motions) return [];
    return Object.entries(this.manifest.motions).map(([name, _def]) => ({
      name,
      count: 1,
    }));
  }

  getExpressions(): ExpressionInfo[] {
    if (!this.manifest?.expressions) return [];
    return Object.keys(this.manifest.expressions).map((name) => ({ name }));
  }

  /** Set direction by degree (0-360) */
  setDirection(degree: number): void {
    if (!this.engine || !this.manifest?.directions?.enabled) return;
    const dir = degreeToDirection(degree);
    this.engine.setDirection(dir);
  }

  /** Set display scale */
  setScale(scale: number): void {
    this.scale = scale;
  }

  /** Enable/disable debug overlay */
  setDebugMode(enabled: boolean): void {
    this.debugMode = enabled;
  }

  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    if (this.canvas) {
      this.canvas.width = width;
      this.canvas.height = height;
    }
  }

  destroy(): void {
    this.stopRenderLoop();
    if (this.canvas && this.canvas.parentNode) {
      this.canvas.parentNode.removeChild(this.canvas);
    }
    this.canvas = null;
    this.ctx = null;
    this.spriteImage = null;
    this.expressionImage = null;
    this.engine = null;
    this.manifest = null;
  }

  /** Start the render loop */
  private startRenderLoop(): void {
    this.lastTimestamp = performance.now();
    const loop = (timestamp: number) => {
      const deltaMs = timestamp - this.lastTimestamp;
      this.lastTimestamp = timestamp;

      this.render(deltaMs);
      this.animationFrameId = requestAnimationFrame(loop);
    };
    this.animationFrameId = requestAnimationFrame(loop);
  }

  /** Stop the render loop */
  private stopRenderLoop(): void {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = 0;
    }
  }

  /**
   * Calculate the draw rect to center the sprite in the canvas,
   * fitting within maxDisplaySize while preserving aspect ratio.
   */
  private computeDrawRect(frameW: number, frameH: number): { dx: number; dy: number; dw: number; dh: number } {
    const canvasW = this.canvas?.width ?? this.width;
    const canvasH = this.canvas?.height ?? this.height;

    // Scale to fit within maxDisplaySize, preserving aspect ratio
    const fitScale = Math.min(this.maxDisplaySize / frameW, this.maxDisplaySize / frameH);
    const dw = frameW * fitScale * this.scale;
    const dh = frameH * fitScale * this.scale;
    // Center in canvas
    const dx = (canvasW - dw) / 2;
    const dy = (canvasH - dh) / 2;

    return { dx, dy, dw, dh };
  }

  /** Render a single frame */
  private render(deltaMs: number): void {
    if (!this.ctx || !this.canvas) return;

    // Update FPS counter
    this.frameCount++;
    this.fpsAccumulator += deltaMs;
    if (this.fpsAccumulator >= 1000) {
      this.currentFps = this.frameCount;
      this.frameCount = 0;
      this.fpsAccumulator = 0;
      if (this.debugMode) {
        console.log(`SpriteSheet FPS: ${this.currentFps}`);
      }
    }

    // Clear canvas
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    // If sprite failed to load, show placeholder
    if (this.spriteLoadFailed || !this.spriteImage || !this.engine || !this.manifest) {
      this.renderPlaceholder();
      return;
    }

    // Always pass deltaMs so state animations play correctly
    const frame = this.engine.update(deltaMs);

    // Draw main frame from sprite sheet
    const { sx, sy, sw, sh } = frame.sourceRect;
    const { dx, dy, dw, dh } = this.computeDrawRect(sw, sh);

    this.ctx.drawImage(
      this.spriteImage,
      sx, sy, sw, sh,
      dx, dy, dw, dh
    );

    // Draw expression overlay if present (graceful: skip if failed to load)
    if (this.expressionImage) {
      this.ctx.globalCompositeOperation = "source-over";
      this.ctx.drawImage(this.expressionImage, dx, dy, dw, dh);
      this.ctx.globalCompositeOperation = "source-over";
    }

    // Debug overlay
    if (this.debugMode) {
      this.renderDebugOverlay(frame);
    }
  }

  /** Render a placeholder when sprite sheet fails to load */
  private renderPlaceholder(): void {
    if (!this.ctx || !this.canvas) return;
    const ctx = this.ctx;
    const w = this.canvas.width;
    const h = this.canvas.height;

    // Draw a simple animated placeholder
    ctx.fillStyle = "rgba(100, 100, 200, 0.3)";
    ctx.fillRect(0, 0, w, h);
    ctx.strokeStyle = "#6666cc";
    ctx.lineWidth = 2;
    ctx.strokeRect(1, 1, w - 2, h - 2);

    ctx.fillStyle = "#6666cc";
    ctx.font = "12px monospace";
    ctx.textAlign = "center";
    ctx.fillText("Sprite Load Failed", w / 2, h / 2 - 8);
    ctx.font = "10px monospace";
    ctx.fillText("Check console for errors", w / 2, h / 2 + 8);
  }

  /** Render debug overlay showing frame info */
  private renderDebugOverlay(frame: { sourceRect: { sx: number; sy: number }; }): void {
    if (!this.ctx || !this.engine || !this.manifest || !this.canvas) return;
    const ctx = this.ctx;

    ctx.fillStyle = "rgba(0, 0, 0, 0.6)";
    ctx.fillRect(0, 0, 140, 60);

    ctx.fillStyle = "#00ff00";
    ctx.font = "10px monospace";
    ctx.textAlign = "left";
    ctx.fillText(`State: ${this.engine.getStateName()}`, 4, 12);
    ctx.fillText(`Dir: ${this.engine.getDirection()}`, 4, 24);
    ctx.fillText(`FPS: ${this.currentFps}`, 4, 36);
    ctx.fillText(`Src: (${frame.sourceRect.sx}, ${frame.sourceRect.sy})`, 4, 48);
  }

  /** Load an image and return a promise */
  private loadImage(url: string): Promise<HTMLImageElement> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = () => reject(new Error(`Failed to load image: ${url}`));
      img.src = url;
    });
  }
}
