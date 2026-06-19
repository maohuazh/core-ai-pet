/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

// Live2D Cubism Core global declaration
declare namespace Live2DCubismCore {
  function start(): void;
  function dispose(): void;
}

declare global {
  interface Window {
    Live2DCubismCore: typeof Live2DCubismCore;
  }
}

// pixi-live2d-display module declaration
declare module "pixi-live2d-display" {
  import { Container } from "pixi.js";

  export class Live2DModel extends Container {
    static from(source: string, options?: any): Promise<Live2DModel>;
    motion(group: string, index?: number, priority?: number): void;
    expression(id: string | number): void;
    internalModel: {
      motionManager: {
        definitions: {
          idle?: any;
          [key: string]: any;
        };
      };
    };
    width: number;
    height: number;
    scale: {
      set(x: number, y?: number): void;
    };
    anchor: {
      set(x: number, y?: number): void;
    };
    x: number;
    y: number;
  }
}
