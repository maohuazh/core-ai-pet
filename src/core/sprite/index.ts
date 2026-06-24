/**
 * Sprite module - sprite sheet frame animation support.
 */

export type {
  Direction,
  SourceRect,
  FrameInfo,
  ManifestMeta,
  SpritesheetConfig,
  DirectionsConfig,
  StateDefinition,
  MotionDefinition,
  ExpressionDefinition,
  DefaultsConfig,
  SpriteSheetManifest,
} from "./types";

export { parseManifest } from "./manifestParser";
export { degreeToDirection } from "./directionMapper";
export { SpriteSheetAnimationEngine } from "./animationEngine";
