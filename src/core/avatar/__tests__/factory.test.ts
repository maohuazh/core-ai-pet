import { describe, it, expect } from "vitest";
import { createAvatar } from "../factory";
import { Live2DAvatar } from "../Live2DAvatar";
import { SpriteSheetAvatar } from "../SpriteSheetAvatar";

describe("createAvatar", () => {
  it("creates Live2DAvatar for type 'live2d'", () => {
    const mockRenderer = {} as any;
    const avatar = createAvatar("live2d", mockRenderer);
    expect(avatar).toBeInstanceOf(Live2DAvatar);
  });

  it("creates SpriteSheetAvatar for type 'sprite'", () => {
    const mockRenderer = {} as any;
    const avatar = createAvatar("sprite", mockRenderer);
    expect(avatar).toBeInstanceOf(SpriteSheetAvatar);
  });

  it("throws for unknown type", () => {
    expect(() => createAvatar("unknown" as any, {} as any)).toThrow(/Unknown avatar type/);
  });

  it("throws for 'pixel' type (not yet implemented)", () => {
    expect(() => createAvatar("pixel", {} as any)).toThrow(/not yet implemented/);
  });

  it("throws for '3d' type (not yet implemented)", () => {
    expect(() => createAvatar("3d", {} as any)).toThrow(/not yet implemented/);
  });
});
