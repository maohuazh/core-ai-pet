/**
 * SpriteSheet implementation of the Avatar interface.
 * Wraps SpriteSheetRenderer to provide state-driven frame animation control.
 */

import type { Avatar } from "./types";
import type { PetState } from "../state/types";
import type { SpriteSheetRenderer } from "../renderer/sprite/SpriteSheetRenderer";

export class SpriteSheetAvatar implements Avatar {
  private currentState: PetState = "Idle";
  private destroyed = false;

  constructor(private renderer: SpriteSheetRenderer) {}

  async speak(_text?: string): Promise<void> {
    this.ensureAlive();
    this.currentState = "Talking";
    await this.renderer.playMotion("talk");
  }

  async think(): Promise<void> {
    this.ensureAlive();
    this.currentState = "Thinking";
    await this.renderer.playMotion("think");
  }

  async work(): Promise<void> {
    this.ensureAlive();
    this.currentState = "Working";
    await this.renderer.playMotion("work");
  }

  async playMotion(group: string, _index?: number): Promise<void> {
    this.ensureAlive();
    await this.renderer.playMotion(group, _index);
  }

  async playExpression(nameOrIndex: string | number): Promise<void> {
    this.ensureAlive();
    await this.renderer.playExpression(nameOrIndex);
  }

  getState(): PetState {
    return this.currentState;
  }

  setDirection(degree: number): void {
    this.ensureAlive();
    this.renderer.setDirection(degree);
  }

  setScale(scale: number): void {
    this.ensureAlive();
    this.renderer.setScale(scale);
  }

  destroy(): void {
    this.destroyed = true;
    this.renderer.destroy();
  }

  private ensureAlive(): void {
    if (this.destroyed) {
      throw new Error("Avatar has been destroyed");
    }
  }
}
