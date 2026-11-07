<script lang="ts">
  import { onMount } from "svelte";

  let isMaximized = $state(false);
  let isTauri = $state(false);

  async function checkMaximized() {
    if (!isTauri) return;
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      isMaximized = await getCurrentWindow().isMaximized();
    } catch (e) {
      console.error(e);
    }
  }

  onMount(() => {
    isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

    if (isTauri) {
      checkMaximized();
      let unlisten: (() => void) | undefined;
      import("@tauri-apps/api/window").then(({ getCurrentWindow }) => {
        getCurrentWindow()
          .onResized(() => {
            checkMaximized();
          })
          .then((fn) => {
            unlisten = fn;
          });
      });

      return () => {
        unlisten?.();
      };
    }
  });

  async function handleMinimize() {
    if (!isTauri) return;
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().minimize();
  }

  async function handleToggleMaximize() {
    if (!isTauri) return;
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().toggleMaximize();
    await checkMaximized();
  }

  async function handleClose() {
    if (!isTauri) return;
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().close();
  }
</script>

<header
  data-tauri-drag-region
  class="h-8 w-full bg-black/25 backdrop-blur-2xl border-b border-white/10 select-none flex items-center justify-between text-xs text-white/70 z-50 shrink-0 font-sans"
>
  <!-- Left: Logo & Menus -->
  <div
    class="flex flex-1 items-center h-full px-2 gap-1"
    data-tauri-drag-region="false"
  >
    <div class="flex items-center gap-2 pr-1.5 pl-0.5">
      <img src="/app-icon.png" alt="PungPlay" class="w-4 h-4 object-contain" />
    </div>
  </div>

  <!-- Center: Title with drag region -->
  <div
    data-tauri-drag-region
    class="flex-1 h-full flex items-center justify-center text-[11px] tracking-wide text-muted-foreground/80 cursor-default"
  >
    <span>PungPlay</span>
  </div>

  <!-- Right: Window Controls (Windows 11 style) -->
  <div
    class="flex flex-1 items-center justify-end h-full"
    data-tauri-drag-region="false"
  >
    <!-- Minimize Button -->
    <button
      type="button"
      onclick={handleMinimize}
      title="Minimize"
      aria-label="Minimize"
      class="w-11 h-full flex items-center justify-center hover:bg-muted text-muted-foreground hover:text-foreground transition-colors active:bg-muted/80 outline-none"
    >
      <svg class="w-2.5 h-2.5" viewBox="0 0 10 10">
        <path d="M0 5.5H10" stroke="currentColor" stroke-width="1" />
      </svg>
    </button>

    <!-- Maximize / Restore Button -->
    <button
      type="button"
      onclick={handleToggleMaximize}
      title={isMaximized ? "Restore" : "Maximize"}
      aria-label={isMaximized ? "Restore" : "Maximize"}
      class="w-11 h-full flex items-center justify-center hover:bg-muted text-muted-foreground hover:text-foreground transition-colors active:bg-muted/80 outline-none"
    >
      {#if isMaximized}
        <!-- Restore Icon (Two overlapping boxes) -->
        <svg class="w-2.5 h-2.5" viewBox="0 0 10 10" fill="none">
          <path
            d="M2.5 2.5V1H9V7.5H7.5"
            stroke="currentColor"
            stroke-width="1"
          />
          <rect
            x="1"
            y="2.5"
            width="6.5"
            height="6.5"
            stroke="currentColor"
            stroke-width="1"
          />
        </svg>
      {:else}
        <!-- Maximize Icon (Single square) -->
        <svg class="w-2.5 h-2.5" viewBox="0 0 10 10" fill="none">
          <rect
            x="1"
            y="1"
            width="8"
            height="8"
            stroke="currentColor"
            stroke-width="1"
          />
        </svg>
      {/if}
    </button>

    <!-- Close Button -->
    <button
      type="button"
      onclick={handleClose}
      title="Close"
      aria-label="Close"
      class="w-11 h-full flex items-center justify-center hover:bg-[#e81123] text-muted-foreground hover:text-white transition-colors active:bg-[#c4101f] outline-none"
    >
      <svg class="w-2.5 h-2.5" viewBox="0 0 10 10">
        <path
          d="M1 1L9 9M9 1L1 9"
          stroke="currentColor"
          stroke-width="1.1"
          stroke-linecap="round"
        />
      </svg>
    </button>
  </div>
</header>
