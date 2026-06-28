/**
 * 极简定位 composable —— 没有依赖。
 * 支持锚点为 HTMLElement 或视口坐标 {x, y}；自动 flip；窗口 resize / 祖先 scroll 重算。
 * 仅覆盖项目内部需求：bottom-start / bottom-end / top-start / top-end / right-start / left-start。
 */

import { ref, watch, onScopeDispose, type Ref } from "vue";

export type Placement =
  | "bottom-start"
  | "bottom-end"
  | "top-start"
  | "top-end"
  | "right-start"
  | "left-start";

export type Anchor = HTMLElement | { x: number; y: number } | null;

interface UseFloatingOpts {
  anchor: Ref<Anchor>;
  open: Ref<boolean>;
  floating: Ref<HTMLElement | null>;
  placement?: Placement;
  offset?: number;
  /** 视口边距，避免贴边 */
  viewportPadding?: number;
}

export function useFloating(opts: UseFloatingOpts) {
  const x = ref(0);
  const y = ref(0);
  const ready = ref(false);
  const placement = opts.placement ?? "bottom-start";
  const offset = opts.offset ?? 6;
  const pad = opts.viewportPadding ?? 8;

  function compute() {
    const a = opts.anchor.value;
    const f = opts.floating.value;
    if (!a || !f) {
      ready.value = false;
      return;
    }
    const fw = f.offsetWidth;
    const fh = f.offsetHeight;
    const vw = window.innerWidth;
    const vh = window.innerHeight;

    // 锚点矩形（Element 或 point→1x1）
    let ax: number, ay: number, aw: number, ah: number;
    if (a instanceof HTMLElement) {
      const r = a.getBoundingClientRect();
      ax = r.left;
      ay = r.top;
      aw = r.width;
      ah = r.height;
    } else {
      ax = a.x;
      ay = a.y;
      aw = 0;
      ah = 0;
    }

    let p = placement;

    // 翻转：底部空间不足 → top；顶部不足 → 还是 bottom（按视口大者）
    if (p.startsWith("bottom") && ay + ah + offset + fh > vh - pad) {
      if (ay - offset - fh >= pad) p = p.replace("bottom", "top") as Placement;
    } else if (p.startsWith("top") && ay - offset - fh < pad) {
      if (ay + ah + offset + fh <= vh - pad) p = p.replace("top", "bottom") as Placement;
    }

    let nx = 0;
    let ny = 0;
    switch (p) {
      case "bottom-start":
        nx = ax;
        ny = ay + ah + offset;
        break;
      case "bottom-end":
        nx = ax + aw - fw;
        ny = ay + ah + offset;
        break;
      case "top-start":
        nx = ax;
        ny = ay - offset - fh;
        break;
      case "top-end":
        nx = ax + aw - fw;
        ny = ay - offset - fh;
        break;
      case "right-start":
        nx = ax + aw + offset;
        ny = ay;
        break;
      case "left-start":
        nx = ax - offset - fw;
        ny = ay;
        break;
    }

    // 视口边界夹紧
    nx = Math.min(Math.max(nx, pad), vw - fw - pad);
    ny = Math.min(Math.max(ny, pad), vh - fh - pad);

    x.value = nx;
    y.value = ny;
    ready.value = true;
  }

  // 监听锚点祖先的滚动 + 窗口 resize
  let scrollParents: (Element | Window)[] = [];

  function collectScrollParents(el: Element | null): (Element | Window)[] {
    const out: (Element | Window)[] = [window];
    let cur: Element | null = el;
    while (cur && cur !== document.body) {
      const s = getComputedStyle(cur);
      if (/(auto|scroll|overlay)/.test(s.overflow + s.overflowY + s.overflowX)) {
        out.push(cur);
      }
      cur = cur.parentElement;
    }
    return out;
  }

  function attach() {
    const a = opts.anchor.value;
    scrollParents = collectScrollParents(a instanceof HTMLElement ? a : null);
    scrollParents.forEach((p) => p.addEventListener("scroll", compute, { passive: true }));
    window.addEventListener("resize", compute);
  }

  function detach() {
    scrollParents.forEach((p) => p.removeEventListener("scroll", compute));
    window.removeEventListener("resize", compute);
    scrollParents = [];
  }

  watch(
    [opts.open, opts.anchor, opts.floating],
    async ([open]) => {
      detach();
      if (open && opts.anchor.value && opts.floating.value) {
        attach();
        // 等下一帧让 floating 完成首次布局再算
        requestAnimationFrame(compute);
      } else {
        ready.value = false;
      }
    },
    { flush: "post" }
  );

  onScopeDispose(detach);

  return { x, y, ready, recompute: compute };
}
