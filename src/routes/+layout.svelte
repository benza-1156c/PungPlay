<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { onMount } from "svelte";
  import { initAuth } from "../stores/auth.svelte";
  import "./layout.css";
  import { ModeWatcher } from "mode-watcher";
  import AppSidebar from "../components/home/AppSidebar.svelte";

  let { children } = $props();

  onMount(() => {
    const cleanupPromise = initAuth();
    return () => {
      cleanupPromise.then((cleanup) => cleanup?.());
    };
  });
</script>

<ModeWatcher defaultMode="dark" />
<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset class="min-h-screen">
    {@render children?.()}
  </Sidebar.Inset>
</Sidebar.Provider>
