/**
 * Manifest parser - validates and parses sprite sheet manifest.json using Zod.
 */

import { z } from "zod";
import type { SpriteSheetManifest } from "./types";

const ManifestMetaSchema = z.object({
  name: z.string(),
  author: z.string().optional(),
  description: z.string().optional(),
  version: z.string().optional(),
  thumbnail: z.string().optional(),
  license: z.string().optional(),
});

const SpritesheetConfigSchema = z.object({
  image: z.string(),
  frameWidth: z.number().positive(),
  frameHeight: z.number().positive(),
  columns: z.number().int().positive(),
  rows: z.number().int().positive(),
  padding: z.number().min(0).default(0),
  spacing: z.number().min(0).default(0),
});

const DirectionsConfigSchema = z.object({
  enabled: z.boolean(),
  count: z.union([z.literal(1), z.literal(4), z.literal(8)]),
  mapping: z.record(z.string(), z.number().int().nonnegative()),
});

const StateDefinitionSchema = z.object({
  frames: z.object({
    start: z.number().int().nonnegative(),
    count: z.number().int().positive(),
  }),
  fps: z.number().positive().max(60),
  loop: z.boolean().default(true),
});

const MotionDefinitionSchema = z.object({
  state: z.string(),
  group: z.string().default("default"),
});

const ExpressionDefinitionSchema = z.object({
  overlay: z.string().nullable(),
});

const DefaultsConfigSchema = z.object({
  state: z.string(),
  direction: z.string().default("S"),
  motion: z.string().optional(),
  expression: z.string().default("default"),
});

const ManifestSchema = z.object({
  version: z.string(),
  meta: ManifestMetaSchema,
  spritesheet: SpritesheetConfigSchema,
  directions: DirectionsConfigSchema.optional(),
  states: z.record(z.string(), StateDefinitionSchema),
  motions: z.record(z.string(), MotionDefinitionSchema).optional(),
  expressions: z.record(z.string(), ExpressionDefinitionSchema).optional(),
  defaults: DefaultsConfigSchema,
  animated: z.boolean().default(true),
});

/**
 * Parse and validate a raw manifest object.
 * Throws Error with Zod details if validation fails.
 */
export function parseManifest(raw: unknown): SpriteSheetManifest {
  const result = ManifestSchema.safeParse(raw);
  if (!result.success) {
    const errors = result.error.issues
      .map((e: any) => `${e.path.join(".")}: ${e.message}`)
      .join("; ");
    throw new Error(`Invalid manifest.json: ${errors}`);
  }
  return result.data as SpriteSheetManifest;
}
