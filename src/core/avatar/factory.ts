/**
 * Avatar factory - creates the appropriate Avatar implementation.
 */

import type { Avatar, AvatarType } from "./types";
import { Live2DAvatar } from "./Live2DAvatar";
import { SpriteSheetAvatar } from "./SpriteSheetAvatar";
import type { IRenderer } from "../renderer/live2d/Live2DRenderer";
import type { SpriteSheetRenderer } from "../renderer/sprite/SpriteSheetRenderer";

/**
 * Create an Avatar instance based on the type.
 */
export function createAvatar(type: AvatarType, renderer: IRenderer | SpriteSheetRenderer): Avatar {
  switch (type) {
    case "live2d":
      return new Live2DAvatar(renderer as IRenderer);
    case "sprite":
      return new SpriteSheetAvatar(renderer as SpriteSheetRenderer);
    case "pixel":
      throw new Error("Pixel avatar not yet implemented");
    case "3d":
      throw new Error("3D avatar not yet implemented");
    default:
      throw new Error(`Unknown avatar type: ${type}`);
  }
}
