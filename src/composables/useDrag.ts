import { ref, reactive, onUnmounted, shallowRef } from 'vue';

const DRAG_THRESHOLD = 5;

type DragType = 'device' | 'connection';

interface DragState {
    type: DragType | null;
    id: string | null;
    active: boolean;
    x: number;
    y: number;
}

export const dragState = reactive<DragState>({
    type: null,
    id: null,
    active: false,
    x: 0,
    y: 0,
});

interface Target {
    el: HTMLElement;
    accepts: DragType;
    onDrop: (id: string) => void;
    isOver: ReturnType<typeof ref<boolean>>;
}

const targets = shallowRef<Target[]>([]);

export function registerDropTarget(target: Target) {
    targets.value.push(target);

    onUnmounted(() => {
        targets.value = targets.value.filter(t => t !== target);
    });
}

function hitTest(x: number, y: number): Target | null {
    let best: Target | null = null;
    let bestArea = Infinity;

    for (const t of targets.value) {
        const r = t.el.getBoundingClientRect();

        if (
            x < r.left ||
            x > r.right ||
            y < r.top ||
            y > r.bottom
        ) continue;

        if (t.accepts !== dragState.type) continue;

        const area = r.width * r.height;

        if (area < bestArea) {
            best = t;
            bestArea = area;
        }
    }

    return best;
}

/**
 * CORE ENGINE (single source of truth)
 */
function useDragCore() {
    let dragging = false;
    let startX = 0;
    let startY = 0;

    let activePointerId: number | null = null;
    let activeTarget: Target | null = null;

    const ghost = ref<{ label: string; visible: boolean }>({
        label: '',
        visible: false,
    });

    let raf = 0;

    function updateHover(x: number, y: number) {
        const hit = hitTest(x, y);

        if (hit !== activeTarget) {
            if (activeTarget) activeTarget.isOver.value = false;

            activeTarget = hit;

            if (activeTarget) activeTarget.isOver.value = true;
        }
    }

    function startDrag(
        type: DragType,
        id: string,
        label: string,
        e: PointerEvent
    ) {
        const source = e.currentTarget as HTMLElement;

        startX = e.clientX;
        startY = e.clientY;

        dragging = false;

        dragState.type = type;
        dragState.id = id;
        dragState.x = e.clientX;
        dragState.y = e.clientY;

        const onMove = (ev: PointerEvent) => {
            dragState.x = ev.clientX;
            dragState.y = ev.clientY;

            if (!dragging) {
                const dx = ev.clientX - startX;
                const dy = ev.clientY - startY;

                if (Math.hypot(dx, dy) < DRAG_THRESHOLD) return;

                dragging = true;
                activePointerId = ev.pointerId;

                try {
                    source.setPointerCapture(ev.pointerId);
                } catch {}

                ghost.value = {
                    label,
                    visible: true,
                };
            }

            cancelAnimationFrame(raf);

            raf = requestAnimationFrame(() => {
                updateHover(ev.clientX, ev.clientY);
            });
        };

        const end = (ev: PointerEvent) => {
            source.removeEventListener('pointermove', onMove);
            source.removeEventListener('pointerup', end);
            source.removeEventListener('pointercancel', end);
            window.removeEventListener('blur', end);

            cancelAnimationFrame(raf);

            if (dragging && activeTarget && dragState.id) {
                activeTarget.onDrop(dragState.id);
            }

            if (activeTarget) {
                activeTarget.isOver.value = false;
                activeTarget = null;
            }

            ghost.value.visible = false;

            try {
                if (
                    activePointerId !== null &&
                    source.hasPointerCapture(activePointerId)
                ) {
                    source.releasePointerCapture(activePointerId);
                }
            } catch {}

            activePointerId = null;
            dragging = false;

            dragState.type = null;
            dragState.id = null;
        };

        source.addEventListener('pointermove', onMove);
        source.addEventListener('pointerup', end);
        source.addEventListener('pointercancel', end);
        window.addEventListener('blur', end);
    }

    onUnmounted(() => {
        cancelAnimationFrame(raf);
    });

    return {
        startDrag,
        ghost,
    };
}

/**
 * =========================
 * BACKWARD COMPATIBILITY API
 * =========================
 */

export function useDragSource() {
    const core = useDragCore();

    return {
        startDrag: core.startDrag,
        ghost: core.ghost,
    };
}

export function useDragTarget(
    el: () => HTMLElement | null,
    accepts: DragType,
    onDrop: (id: string) => void
) {
    const isOver = ref(false);

    let registered: HTMLElement | null = null;

    function register() {
        const node = el();

        if (!node) return;

        if (node !== registered) {
            if (registered) {
                const idx = targets.value.findIndex(t => t.el === registered);
                if (idx !== -1) targets.value.splice(idx, 1);
            }

            registered = node;

            targets.value.push({
                el: node,
                accepts,
                onDrop,
                isOver,
            });
        }
    }

    onUnmounted(() => {
        if (registered) {
            const idx = targets.value.findIndex(t => t.el === registered);
            if (idx !== -1) targets.value.splice(idx, 1);
            registered = null;
        }

        isOver.value = false;
    });

    return { isOver, register };
}