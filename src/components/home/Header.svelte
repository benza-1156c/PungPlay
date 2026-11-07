<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Input from "$lib/components/ui/input/input.svelte";
  import Account from "./Account.svelte";
  import { Search } from "@lucide/svelte";
  import { onMount } from "svelte";

  let isScrolled = $state(false);

  onMount(() => {
    const handleScroll = () => {
      isScrolled = window.scrollY > 10;
    };

    handleScroll();

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  });
</script>

<div
  class="flex h-16 shrink-0 items-center justify-between gap-4 px-4 border-b sticky top-8 z-10
  {isScrolled ? 'bg-background/90' : null}
  "
>
  <!-- Left: Sidebar toggle -->
  <div class="flex items-center gap-2">
    <Sidebar.Trigger class="cursor-pointer" />
  </div>

  <!-- Center: Search bar -->
  <div class="flex flex-1 max-w-lg items-center justify-center">
    <div class="relative w-full">
      <Search
        class="absolute left-3.5 top-1/2 -translate-y-1/2 size-4 text-muted-foreground pointer-events-none"
      />
      <Input class="pl-10 rounded-full h-10" placeholder="Type to search" />
    </div>
  </div>

  <!-- Right: User Account -->
  <div class="flex items-center gap-3">
    <Account />
  </div>
</div>
