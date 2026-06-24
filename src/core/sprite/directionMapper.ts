/**
 * Direction mapper - converts degree angles to 8 cardinal directions.
 * 0° = East, counter-clockwise increment (matching existing setDirection convention).
 */

import type { Direction } from "./types";

/**
 * Map a degree angle (0-360) to one of 8 cardinal directions.
 * Each direction occupies a 45° sector centered on the cardinal angle.
 *
 * Mapping:
 *   0°   → E
 *   45°  → NE
 *   90°  → N
 *   135° → NW
 *   180° → W
 *   225° → SW
 *   270° → S
 *   315° → SE
 */
export function degreeToDirection(degree: number): Direction {
  const normalized = ((degree % 360) + 360) % 360;
  const sector = Math.round(normalized / 45) % 8;
  const directions: Direction[] = ["E", "NE", "N", "NW", "W", "SW", "S", "SE"];
  return directions[sector];
}
