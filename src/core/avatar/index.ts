/**
 * Avatar module - unified character abstraction.
 */

export type { Avatar, AvatarType } from "./types";
export { Live2DAvatar } from "./Live2DAvatar";
export { SpriteSheetAvatar } from "./SpriteSheetAvatar";
export { createAvatar } from "./factory";
