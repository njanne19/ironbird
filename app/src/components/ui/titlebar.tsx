import { getCurrentWindow } from "@tauri-apps/api/window"
import { useIronbirdAppStore } from "@/store/app";

const appWindow = getCurrentWindow()

export function Titlebar() {
    const titlebarLabel = useIronbirdAppStore((state) => state.titlebarLabel);
    return (
        <div
        data-tauri-drag-region
        className="h-8 flex items-center justify-between px-3 select-none fixed top-0 left-0 right-0 z-50"
        style={{ background: "var(--background)", borderBottom: "1px solid var(--border)" }}
        >
        <span className="text-xs text-muted-foreground">{titlebarLabel}</span>

        {/* Window controls - right side */}
        <div className="flex items-center">
        <button
        onClick={() => appWindow.minimize()}
        className="h-8 w-12 flex items-center justify-center hover:bg-accent text-muted-foreground hover:text-foreground"
        >
        ─
        </button>
        <button
        onClick={() => appWindow.toggleMaximize()}
        className="h-8 w-12 flex items-center justify-center hover:bg-accent text-muted-foreground hover:text-foreground"
        >
        □
        </button>
        <button
        onClick={() => appWindow.close()}
        className="h-8 w-12 flex items-center justify-center hover:bg-destructive hover:text-destructive-foreground text-muted-foreground"
        >
        ✕
        </button>
        </div>
        </div>
    )
}
