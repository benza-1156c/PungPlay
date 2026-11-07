<script lang="ts">
  import { page } from "$app/state";
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
</script>

<ModeWatcher defaultMode="dark" />

<div class="flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground select-none">
  <div class="shrink-0 z-50">
    <TitleBar />
  </div>

  <div class="flex-1 flex overflow-hidden min-h-0">
    {#if page.data?.hideSidebar}
      <div class="flex-1 h-full overflow-y-auto">
        {@render children?.()}
      </div>
    {:else}
      <Sidebar.Provider class="h-full w-full min-h-0">
        <AppSidebar />
        <Sidebar.Inset class="h-full min-h-0 overflow-y-auto overflow-x-hidden flex-1">
          {@render children?.()}
        </Sidebar.Inset>
      </Sidebar.Provider>
    {/if}
  </div>
</div>
