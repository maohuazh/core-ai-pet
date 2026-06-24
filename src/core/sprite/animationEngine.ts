/**
 * Sprite sheet animation engine - handles frame scheduling, state management,
 * direction switching, and source rect computation.
 */

import type { Direction, FrameInfo, SourceRect, SpriteSheetManifest } from "./types";

interface AnimationState {
  currentFrame: number;
  elapsed: number;
  frameInterval: number;
  isPlaying: boolean;
  isComplete: boolean;
}

export class SpriteSheetAnimationEngine {
  private state: AnimationState;
  private manifest: SpriteSheetManifest;
  private currentStateName: string;
  private currentDirection: Direction;
  private onCompleteCallback?: () => void;

  constructor(manifest: SpriteSheetManifest) {
    this.manifest = manifest;
    this.currentStateName = manifest.defaults.state;
    this.currentDirection = (manifest.defaults.direction as Direction) || "S";

    const stateDef = manifest.states[this.currentStateName];
    if (!stateDef) {
      throw new Error(`Default state "${this.currentStateName}" not found in manifest`);
    }

    this.state = {
      currentFrame: 0,
      elapsed: 0,
      frameInterval: 1000 / stateDef.fps,
      isPlaying: true,
      isComplete: false,
    };
  }

  /** Set callback for when a non-looping animation completes */
  onComplete(callback: () => void): void {
    this.onCompleteCallback = callback;
  }

  /** Advance animation by deltaMs and return current frame info */
  update(deltaMs: number): FrameInfo {
    if (!this.state.isPlaying || this.state.isComplete) {
      return this.buildFrameInfo();
    }

    this.state.elapsed += deltaMs;

    while (this.state.elapsed >= this.state.frameInterval) {
      this.state.elapsed -= this.state.frameInterval;
      this.state.currentFrame++;

      const stateDef = this.manifest.states[this.currentStateName];
      if (this.state.currentFrame >= stateDef.frames.count) {
        if (stateDef.loop) {
          this.state.currentFrame = 0;
        } else {
          this.state.currentFrame = stateDef.frames.count - 1;
          this.state.isComplete = true;
          this.state.isPlaying = false;
          this.onCompleteCallback?.();
          break;
        }
      }
    }

    return this.buildFrameInfo();
  }

  /** Switch to a new animation state, resetting frame index */
  setState(stateName: string): void {
    const stateDef = this.manifest.states[stateName];
    if (!stateDef) {
      console.warn(`Animation state "${stateName}" not found, falling back to Idle`);
      if (stateName !== "Idle" && this.manifest.states["Idle"]) {
        this.setState("Idle");
      }
      return;
    }

    this.currentStateName = stateName;
    this.state.currentFrame = 0;
    this.state.elapsed = 0;
    this.state.frameInterval = 1000 / stateDef.fps;
    this.state.isPlaying = true;
    this.state.isComplete = false;
  }

  /** Switch direction without resetting frame index */
  setDirection(dir: Direction): void {
    if (!this.manifest.directions?.enabled) {
      console.warn("Sprite sheet has no directions configured");
      return;
    }
    this.currentDirection = dir;
  }

  /** Get current state name */
  getStateName(): string {
    return this.currentStateName;
  }

  /** Get current direction */
  getDirection(): Direction {
    return this.currentDirection;
  }

  /** Compute the source rectangle for the current frame */
  private computeSourceRect(): SourceRect {
    const stateDef = this.manifest.states[this.currentStateName];
    const absFrame = stateDef.frames.start + this.state.currentFrame;
    const columns = this.manifest.spritesheet.columns;
    const frameWidth = this.manifest.spritesheet.frameWidth;
    const frameHeight = this.manifest.spritesheet.frameHeight;
    const padding = this.manifest.spritesheet.padding ?? 0;

    const col = absFrame % columns;
    let row: number;

    if (this.manifest.directions?.enabled) {
      const dirRow = this.manifest.directions.mapping[this.currentDirection] ?? 0;
      row = dirRow + Math.floor(absFrame / columns);
    } else {
      row = Math.floor(absFrame / columns);
    }

    return {
      sx: col * (frameWidth + padding),
      sy: row * (frameHeight + padding),
      sw: frameWidth,
      sh: frameHeight,
    };
  }

  /** Build FrameInfo from current state */
  private buildFrameInfo(): FrameInfo {
    const sourceRect = this.computeSourceRect();
    return {
      sourceRect,
      displayWidth: sourceRect.sw,
      displayHeight: sourceRect.sh,
    };
  }
}
