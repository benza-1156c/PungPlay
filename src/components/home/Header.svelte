<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Account from "./Account.svelte";
  import { Search, Bell } from "@lucide/svelte";
  import { onMount } from "svelte";

  let isScrolled = $state(false);
  let searchQuery = $state("");

  onMount(() => {
    const handleScroll = (e?: Event) => {
      const scrollY =
        (e?.target as HTMLElement)?.scrollTop ?? window.scrollY ?? 0;
      isScrolled = scrollY > 10;
    };

    handleScroll();
    window.addEventListener("scroll", handleScroll, { capture: true });
    return () =>
      window.removeEventListener("scroll", handleScroll, { capture: true });
  });
</script>

<div
  class="flex h-16 shrink-0 items-center justify-between gap-4 px-6 border-b border-white/10 sticky top-0 z-20 transition-colors duration-200 {isScrolled
    ? 'bg-black/50 backdrop-blur-3xl shadow-lg shadow-black/25'
    : 'bg-black/20 backdrop-blur-2xl'}"
>
  <!-- Left: Sidebar toggle -->
  <div class="flex items-center gap-2">
    <Sidebar.Trigger
      class="cursor-pointer text-white/80 hover:text-white hover:bg-white/10 p-2 rounded-full transition-colors"
    />
  </div>

  <!-- Center: Search bar (visionOS frosted pill) -->
  <div class="flex flex-1 max-w-xl items-center justify-center">
    <div
      class="flex items-center w-full bg-white/10 hover:bg-white/15 focus-within:bg-white/15 focus-within:ring-1 focus-within:ring-white/20 border border-white/15 backdrop-blur-2xl rounded-full px-4 h-10 transition-all duration-200 shadow-inner"
    >
      <Search class="size-4 text-white/60 shrink-0 mr-3" />
      <input
        type="text"
        bind:value={searchQuery}
        class="bg-transparent border-none outline-none text-sm text-white placeholder:text-white/50 w-full"
        placeholder="Search"
      />
    </div>
  </div>

  <!-- Right: Actions & User Account -->
  <div class="flex items-center gap-2">
    <button
      type="button"
      title="Notifications"
      class="p-2 rounded-full text-white/80 hover:text-white hover:bg-white/10 transition-colors"
    >
      <Bell class="size-5" />
    </button>
    <div class="ml-1">
      <Account />
    </div>
  </div>
</div>
