<script lang="ts">
  import { page } from "$app/state";
  import { onNavigate } from "$app/navigation";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { onMount } from "svelte";
  import { loadProfiles } from "../stores/auth.svelte";
  import "./layout.css";
  import { ModeWatcher } from "mode-watcher";
  import AppSidebar from "../components/home/AppSidebar.svelte";
  import TitleBar from "../components/layout/TitleBar.svelte";

  let { children } = $props();

  onMount(() => {
    loadProfiles();
  });

  onNavigate((navigation) => {
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<ModeWatcher defaultMode="dark" />

<div
  class="flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground select-none"
>
  <div class="shrink-0 z-50">
    <TitleBar />
  </div>

  <div class="flex-1 flex overflow-hidden min-h-0">
    {#if page.data?.hideSidebar}
      <div class="flex-1 h-full overflow-y-auto page-content">
        {@render children?.()}
      </div>
    {:else}
      <Sidebar.Provider class="h-full w-full min-h-0">
        <AppSidebar />
        <Sidebar.Inset
          class="h-full min-h-0 overflow-y-auto overflow-x-hidden flex-1 page-content"
        >
          {@render children?.()}
        </Sidebar.Inset>
      </Sidebar.Provider>
    {/if}
  </div>
</div>

<style>
  :global(header[data-tauri-drag-region]) {
    view-transition-name: app-titlebar;
  }

  :global([data-slot="sidebar-container"]) {
    view-transition-name: app-sidebar;
  }

  :global(.page-content) {
    view-transition-name: app-page;
  }

  :global(::view-transition-old(app-page)) {
    animation: 140ms cubic-bezier(0.4, 0, 1, 1) both vt-fade-out;
  }

  :global(::view-transition-new(app-page)) {
    animation: 240ms cubic-bezier(0, 0, 0.2, 1) both vt-fade-in;
  }

  @keyframes vt-fade-in {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes vt-fade-out {
    from {
      opacity: 1;
      transform: translateY(0);
    }
    to {
      opacity: 0;
      transform: translateY(-4px);
    }
  }
</style>
