/**
 * SpriteSheet manifest types and related interfaces.
 * Defines the structure for sprite sheet frame animation configuration.
 */

/** 8 cardinal directions for sprite animation */
export type Direction = "N" | "NE" | "E" | "SE" | "S" | "SW" | "W" | "NW";

/** Source rectangle for cropping a frame from the sprite sheet */
export interface SourceRect {
  sx: number;
  sy: number;
  sw: number;
  sh: number;
}

/** Information about the current frame to render */
export interface FrameInfo {
  sourceRect: SourceRect;
  displayWidth: number;
  displayHeight: number;
  expressionOverlay?: string;
}

/** Manifest metadata */
export interface ManifestMeta {
  name: string;
  author?: string;
  description?: string;
  version?: string;
  thumbnail?: string;
  license?: string;
}

/** Sprite sheet image configuration */
export interface SpritesheetConfig {
  image: string;
  frameWidth: number;
  frameHeight: number;
  columns: number;
  rows: number;
  padding?: number;
  spacing?: number;
}

/** Direction configuration */
export interface DirectionsConfig {
  enabled: boolean;
  count: 1 | 4 | 8;
  mapping: Record<string, number>;
}

/** State animation definition */
export interface StateDefinition {
  frames: {
    start: number;
    count: number;
  };
  fps: number;
  loop: boolean;
}

/** Motion definition mapping to a state */
export interface MotionDefinition {
  state: string;
  group?: string;
}

/** Expression overlay definition */
export interface ExpressionDefinition {
  overlay: string | null;
}

/** Default configuration */
export interface DefaultsConfig {
  state: string;
  direction?: string;
  motion?: string;
  expression?: string;
}

/** Complete sprite sheet manifest */
export interface SpriteSheetManifest {
  version: string;
  meta: ManifestMeta;
  spritesheet: SpritesheetConfig;
  directions?: DirectionsConfig;
  states: Record<string, StateDefinition>;
  motions?: Record<string, MotionDefinition>;
  expressions?: Record<string, ExpressionDefinition>;
  defaults: DefaultsConfig;
  /** When false, only render the first frame (no animation). Default: true */
  animated?: boolean;
}
