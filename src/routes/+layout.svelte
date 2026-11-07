<script lang="ts">
  import { page } from "$app/state";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import "./layout.css";
  import { ModeWatcher } from "mode-watcher";
  import AppSidebar from "../components/home/AppSidebar.svelte";

  let { children } = $props();
</script>

<ModeWatcher defaultMode="dark" />

{#if page.data?.hideSidebar}
  {@render children?.()}
{:else}
  <Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset class="min-h-screen">
      {@render children?.()}
    </Sidebar.Inset>
  </Sidebar.Provider>
{/if}

{#if page.data?.hideSidebar || page.url.pathname === "/login"}
  {@render children?.()}
{:else}
  <Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset class="min-h-screen">
      {@render children?.()}
    </Sidebar.Inset>
  </Sidebar.Provider>
{/if}
