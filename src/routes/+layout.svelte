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

<Sidebar.Provider>
  <AppSidebar />
  <main>
    <ModeWatcher defaultMode="dark" />
    <Sidebar.Trigger />
    {@render children?.()}
  </main>
</Sidebar.Provider>
