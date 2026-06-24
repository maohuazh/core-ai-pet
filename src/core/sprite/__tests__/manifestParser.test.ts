import { describe, it, expect } from "vitest";
import { parseManifest } from "../manifestParser";

const validManifest = {
  version: "1.0",
  meta: { name: "TestCat" },
  spritesheet: {
    image: "sheet.png",
    frameWidth: 64,
    frameHeight: 64,
    columns: 8,
    rows: 16,
  },
  states: {
    Idle: { frames: { start: 0, count: 4 }, fps: 8, loop: true },
    Walking: { frames: { start: 4, count: 8 }, fps: 12, loop: true },
    Alert: { frames: { start: 12, count: 4 }, fps: 12, loop: false },
  },
  motions: {
    idle: { state: "Idle" },
    walk: { state: "Walking" },
    alert: { state: "Alert" },
  },
  defaults: { state: "Idle" },
};

describe("parseManifest", () => {
  it("parses a valid manifest", () => {
    const result = parseManifest(validManifest);
    expect(result.version).toBe("1.0");
    expect(result.meta.name).toBe("TestCat");
    expect(result.spritesheet.frameWidth).toBe(64);
    expect(result.states.Idle.fps).toBe(8);
    expect(result.defaults.state).toBe("Idle");
  });

  it("applies default values for padding and spacing", () => {
    const result = parseManifest(validManifest);
    expect(result.spritesheet.padding).toBe(0);
    expect(result.spritesheet.spacing).toBe(0);
  });

  it("applies default loop value", () => {
    const manifest = {
      ...validManifest,
      states: { Idle: { frames: { start: 0, count: 4 }, fps: 8 } },
    };
    const result = parseManifest(manifest);
    expect(result.states.Idle.loop).toBe(true);
  });

  it("parses manifest with directions", () => {
    const manifest = {
      ...validManifest,
      directions: {
        enabled: true,
        count: 8 as const,
        mapping: { N: 0, E: 2, S: 4, W: 6 },
      },
    };
    const result = parseManifest(manifest);
    expect(result.directions?.enabled).toBe(true);
    expect(result.directions?.count).toBe(8);
  });

  it("parses manifest without directions", () => {
    const result = parseManifest(validManifest);
    expect(result.directions).toBeUndefined();
  });

  it("parses manifest without expressions", () => {
    const result = parseManifest(validManifest);
    expect(result.expressions).toBeUndefined();
  });

  it("throws on missing version field", () => {
    const { version, ...noVersion } = validManifest;
    expect(() => parseManifest(noVersion)).toThrow(/version/i);
  });

  it("throws on missing meta field", () => {
    const { meta, ...noMeta } = validManifest;
    expect(() => parseManifest(noMeta)).toThrow(/meta/i);
  });

  it("throws on missing states field", () => {
    const { states, ...noStates } = validManifest;
    expect(() => parseManifest(noStates)).toThrow(/states/i);
  });

  it("throws on negative frameWidth", () => {
    const bad = {
      ...validManifest,
      spritesheet: { ...validManifest.spritesheet, frameWidth: -1 },
    };
    expect(() => parseManifest(bad)).toThrow(/frameWidth/i);
  });

  it("throws on fps > 60", () => {
    const bad = {
      ...validManifest,
      states: { Idle: { frames: { start: 0, count: 4 }, fps: 120, loop: true } },
    };
    expect(() => parseManifest(bad)).toThrow(/fps/i);
  });

  it("throws on zero frame count", () => {
    const bad = {
      ...validManifest,
      states: { Idle: { frames: { start: 0, count: 0 }, fps: 8, loop: true } },
    };
    expect(() => parseManifest(bad)).toThrow(/count/i);
  });

  it("throws on null input", () => {
    expect(() => parseManifest(null)).toThrow();
  });

  it("throws on non-object input", () => {
    expect(() => parseManifest("string")).toThrow();
  });
});
