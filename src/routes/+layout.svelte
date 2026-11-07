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

<div class="flex flex-col min-h-screen bg-background text-foreground">
  <div class="sticky top-0 z-50">
    <TitleBar />
  </div>

  <div class="flex-1 flex flex-col">
    {#if page.data?.hideSidebar}
      {@render children?.()}
    {:else}
      <Sidebar.Provider>
        <AppSidebar />
        <Sidebar.Inset class="min-h-[calc(100vh-2rem)]">
          {@render children?.()}
        </Sidebar.Inset>
      </Sidebar.Provider>
    {/if}
  </div>
</div>
