/**
 * Live2D implementation of the Avatar interface.
 * Wraps Live2DRenderer to provide state-driven animation control.
 */

import type { Avatar } from "./types";
import type { PetState } from "../state/types";
import type { IRenderer } from "../renderer/live2d/Live2DRenderer";

/**
 * Live2D implementation of the Avatar interface.
 * Wraps a renderer implementing IRenderer to provide state-driven animation control.
 */
export class Live2DAvatar implements Avatar {
  private currentState: PetState = "Idle";
  private destroyed = false;

  constructor(private renderer: IRenderer) {}

  async speak(_text?: string): Promise<void> {
    this.ensureAlive();
    this.currentState = "Talking";
    await this.playMotionWithFallback("Talking", "Talk");
  }

  async think(): Promise<void> {
    this.ensureAlive();
    this.currentState = "Thinking";
    await this.playMotionWithFallback("Thinking", "Think");
  }

  async work(): Promise<void> {
    this.ensureAlive();
    this.currentState = "Working";
    await this.playMotionWithFallback("Working", "Work");
  }

  async playMotion(group: string, index?: number): Promise<void> {
    this.ensureAlive();
    await this.renderer.playMotion(group, index);
  }

  async playExpression(nameOrIndex: string | number): Promise<void> {
    this.ensureAlive();
    await this.renderer.playExpression(nameOrIndex);
  }

  getState(): PetState {
    return this.currentState;
  }

  destroy(): void {
    this.destroyed = true;
  }

  /**
   * Try playing a motion group, falling back to alternatives.
   * Chain: primary -> fallback -> Idle -> first available -> silent fail
   */
  private async playMotionWithFallback(primary: string, fallback: string): Promise<void> {
    const groups = this.renderer.getMotionGroups();

    // Try primary group (case-insensitive)
    const primaryMatch = groups.find(
      (g) => g.name.toLowerCase() === primary.toLowerCase()
    );
    if (primaryMatch) {
      await this.renderer.playMotion(primaryMatch.name, 0);
      return;
    }

    // Try fallback group
    const fallbackMatch = groups.find(
      (g) => g.name.toLowerCase() === fallback.toLowerCase()
    );
    if (fallbackMatch) {
      console.warn(`Motion group "${primary}" not found, using fallback "${fallback}"`);
      await this.renderer.playMotion(fallbackMatch.name, 0);
      return;
    }

    // Try Idle
    const idleMatch = groups.find((g) => g.name.toLowerCase() === "idle");
    if (idleMatch) {
      console.warn(`Motion group "${primary}" not found, using Idle`);
      await this.renderer.playMotion(idleMatch.name, 0);
      return;
    }

    // Try first available group
    if (groups.length > 0) {
      console.warn(`No Idle group found, using first available: ${groups[0].name}`);
      await this.renderer.playMotion(groups[0].name, 0);
      return;
    }

    // Silent fail
    console.warn(`No motion groups available for "${primary}"`);
  }

  private ensureAlive(): void {
    if (this.destroyed) {
      throw new Error("Avatar has been destroyed");
    }
  }
}
