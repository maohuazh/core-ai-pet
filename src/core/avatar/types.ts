/**
 * Avatar interface - unified abstraction for character rendering.
 * Supports Live2D, Pixel, 3D implementations.
 */

import type { PetState } from "../state/types";

export interface Avatar {
  /** Speak with optional text (triggers Talking animation) */
  speak(text?: string): Promise<void>;

  /** Enter thinking state */
  think(): Promise<void>;

  /** Enter working state */
  work(): Promise<void>;

  /** Play a specific motion group */
  playMotion(group: string, index?: number): Promise<void>;

  /** Play a specific expression */
  playExpression(nameOrIndex: string | number): Promise<void>;

  /** Get current avatar state */
  getState(): PetState;

  /** Destroy the avatar and release resources */
  destroy(): void;
}

export type AvatarType = "live2d" | "pixel" | "3d";
