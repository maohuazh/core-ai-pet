import { describe, it, expect, vi } from "vitest";
import { SpriteSheetAnimationEngine } from "../animationEngine";
import type { SpriteSheetManifest } from "../types";

function makeManifest(overrides?: Partial<SpriteSheetManifest>): SpriteSheetManifest {
  return {
    version: "1.0",
    meta: { name: "Test" },
    spritesheet: { image: "sheet.png", frameWidth: 32, frameHeight: 32, columns: 8, rows: 16 },
    states: {
      Idle: { frames: { start: 0, count: 4 }, fps: 10, loop: true },
      Walking: { frames: { start: 4, count: 8 }, fps: 20, loop: true },
      Alert: { frames: { start: 12, count: 4 }, fps: 10, loop: false },
    },
    defaults: { state: "Idle", direction: "S" },
    ...overrides,
  };
}

function makeDirectionalManifest(): SpriteSheetManifest {
  return makeManifest({
    directions: {
      enabled: true,
      count: 8,
      mapping: { N: 0, NE: 2, E: 4, SE: 6, S: 8, SW: 10, W: 12, NW: 14 },
    },
  });
}

describe("SpriteSheetAnimationEngine", () => {
  describe("frame advance", () => {
    it("starts at frame 0", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      const frame = engine.update(0);
      expect(frame.sourceRect.sx).toBe(0);
      expect(frame.sourceRect.sy).toBe(0);
    });

    it("advances frame when elapsed exceeds interval", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      // fps=10 → interval=100ms
      engine.update(100);
      const frame = engine.update(0);
      expect(frame.sourceRect.sx).toBe(32); // frame 1 * frameWidth
    });

    it("accumulates elapsed time across updates", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      // 3 updates of 40ms each = 120ms → should advance 1 frame (100ms interval)
      engine.update(40);
      engine.update(40);
      const frame = engine.update(40);
      expect(frame.sourceRect.sx).toBe(32); // frame 1
    });

    it("does not advance when elapsed < interval", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.update(50); // 50ms < 100ms interval
      const frame = engine.update(0);
      expect(frame.sourceRect.sx).toBe(0); // still frame 0
    });
  });

  describe("loop behavior", () => {
    it("wraps around when loop=true", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      // Idle: 4 frames at 10fps → 400ms for full cycle
      engine.update(500); // push past end
      const frame = engine.update(0);
      expect(frame.sourceRect.sx).toBe(32); // wrapped to frame 1
    });

    it("stops at last frame when loop=false", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.setState("Alert"); // frames.start=12, count=4, fps=10, loop=false
      engine.update(500); // push past end
      const frame = engine.update(0);
      // Last frame = start + count - 1 = 12 + 3 = 15
      // col = 15 % 8 = 7, sx = 7 * 32 = 224
      expect(frame.sourceRect.sx).toBe(7 * 32);
    });

    it("fires onComplete callback for non-looping", () => {
      const callback = vi.fn();
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.setState("Alert");
      engine.onComplete(callback);
      engine.update(500); // push past end
      expect(callback).toHaveBeenCalledOnce();
    });
  });

  describe("state switching", () => {
    it("resets frame on state change", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.update(250); // advance some frames in Idle
      engine.setState("Walking");
      const frame = engine.update(0);
      // Walking starts at frame.start=4, col=4 → sx=4*32=128
      expect(frame.sourceRect.sx).toBe(128);
    });

    it("updates frameInterval on state change", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.setState("Walking"); // fps=20 → interval=50ms
      engine.update(50);
      const frame = engine.update(0);
      // Walking starts at 4, frame 1 = absFrame 5 → col=5 → sx=160
      expect(frame.sourceRect.sx).toBe(160);
    });

    it("falls back to Idle for unknown state", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      engine.setState("NonExistent");
      expect(engine.getStateName()).toBe("Idle");
    });
  });

  describe("direction switching", () => {
    it("keeps frame index when direction changes", () => {
      const engine = new SpriteSheetAnimationEngine(makeDirectionalManifest());
      engine.update(150); // advance 1 frame in Idle
      engine.setDirection("E");
      const frame = engine.update(0);
      // Frame index should be 1, but row should change based on direction
      expect(frame.sourceRect.sx).toBe(32); // same column
    });

    it("updates source rect row for new direction", () => {
      const engine = new SpriteSheetAnimationEngine(makeDirectionalManifest());
      // Default direction is "S" (row 8)
      const sFrame = engine.update(0);
      engine.setDirection("N"); // row 0
      const nFrame = engine.update(0);
      expect(nFrame.sourceRect.sy).not.toBe(sFrame.sourceRect.sy);
    });

    it("ignores direction when directions disabled", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      // Should not throw
      engine.setDirection("E");
      expect(engine.getDirection()).toBe("S"); // unchanged
    });
  });

  describe("source rect computation", () => {
    it("computes correct rect for no-direction sprite", () => {
      const engine = new SpriteSheetAnimationEngine(makeManifest());
      const frame = engine.update(0);
      expect(frame.sourceRect).toEqual({ sx: 0, sy: 0, sw: 32, sh: 32 });
    });

    it("computes rect with padding", () => {
      const manifest = makeManifest();
      manifest.spritesheet.padding = 2;
      const engine = new SpriteSheetAnimationEngine(manifest);
      engine.update(100); // frame 1
      const frame = engine.update(0);
      // col=1, sx = 1 * (32 + 2) = 34
      expect(frame.sourceRect.sx).toBe(34);
    });

    it("computes rect across rows", () => {
      const manifest = makeManifest();
      manifest.states = {
        Long: { frames: { start: 0, count: 10 }, fps: 10, loop: true },
      };
      manifest.defaults = { state: "Long" };
      const engine = new SpriteSheetAnimationEngine(manifest);
      engine.update(900); // 9 frames at 100ms = frame 9
      const frame = engine.update(0);
      // col = 9 % 8 = 1, row = floor(9/8) = 1
      expect(frame.sourceRect.sx).toBe(32);
      expect(frame.sourceRect.sy).toBe(32);
    });
  });
});
