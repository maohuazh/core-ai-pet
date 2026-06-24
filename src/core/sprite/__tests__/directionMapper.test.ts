import { describe, it, expect } from "vitest";
import { degreeToDirection } from "../directionMapper";

describe("degreeToDirection", () => {
  // Cardinal angles
  it("maps 0° to E", () => expect(degreeToDirection(0)).toBe("E"));
  it("maps 45° to NE", () => expect(degreeToDirection(45)).toBe("NE"));
  it("maps 90° to N", () => expect(degreeToDirection(90)).toBe("N"));
  it("maps 135° to NW", () => expect(degreeToDirection(135)).toBe("NW"));
  it("maps 180° to W", () => expect(degreeToDirection(180)).toBe("W"));
  it("maps 225° to SW", () => expect(degreeToDirection(225)).toBe("SW"));
  it("maps 270° to S", () => expect(degreeToDirection(270)).toBe("S"));
  it("maps 315° to SE", () => expect(degreeToDirection(315)).toBe("SE"));

  // Boundary values (sector edges at 22.5° intervals)
  it("maps 22° to E (boundary)", () => expect(degreeToDirection(22)).toBe("E"));
  it("maps 23° to NE (boundary)", () => expect(degreeToDirection(23)).toBe("NE"));
  it("maps 67° to NE (boundary)", () => expect(degreeToDirection(67)).toBe("NE"));
  it("maps 68° to N (boundary)", () => expect(degreeToDirection(68)).toBe("N"));

  // 360° wraps to 0°
  it("maps 360° to E", () => expect(degreeToDirection(360)).toBe("E"));
  it("maps 359° to E (near wrap)", () => expect(degreeToDirection(359)).toBe("E"));
  it("maps 338° to E (near wrap, closer to 360 than 315)", () => expect(degreeToDirection(338)).toBe("E"));

  // Negative angles
  it("maps -90° to S", () => expect(degreeToDirection(-90)).toBe("S"));
  it("maps -45° to SE", () => expect(degreeToDirection(-45)).toBe("SE"));
  it("maps -360° to E", () => expect(degreeToDirection(-360)).toBe("E"));

  // Large angles
  it("maps 720° to E", () => expect(degreeToDirection(720)).toBe("E"));
  it("maps 450° to N (450 = 360+90 = 90° = N)", () => expect(degreeToDirection(450)).toBe("N"));
});
