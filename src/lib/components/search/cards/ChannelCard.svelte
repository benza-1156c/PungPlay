<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { User, MonitorPlay } from "@lucide/svelte";
  import type { SearchResultItem } from "$lib/bindings.js";

  interface Props {
    item: SearchResultItem;
  }

  let { item }: Props = $props();
</script>

<Card.Root
  class="group bg-neutral-900/60 border-neutral-800/50 hover:border-cyan-500/40 transition-all duration-300 cursor-pointer rounded-xl hover:shadow-lg hover:shadow-cyan-900/10 flex flex-col items-center justify-center p-6 text-center"
>
  <div class="relative w-24 h-24 sm:w-28 sm:h-28 mb-4">
    <div
      class="absolute inset-0 bg-cyan-500/20 rounded-full animate-pulse group-hover:scale-110 transition-transform duration-500"
    ></div>
    {#if item.thumbnail}
      <img
        src={item.thumbnail}
        alt={item.title}
        class="relative object-cover w-full h-full rounded-full ring-4 ring-neutral-800 group-hover:ring-cyan-500/50 transition-all duration-300"
        loading="lazy"
        referrerpolicy="no-referrer"
      />
    {:else}
      <div
        class="relative flex items-center justify-center w-full h-full rounded-full ring-4 ring-neutral-800 bg-neutral-800 text-neutral-500 group-hover:ring-cyan-500/50 transition-all duration-300"
      >
        <User class="w-12 h-12" />
      </div>
    {/if}
  </div>

  <Card.Title
    class="text-base sm:text-lg font-bold leading-snug line-clamp-2 text-neutral-100 group-hover:text-cyan-400 transition-colors"
    title={item.title}
  >
    {item.title}
  </Card.Title>

  <div
    class="mt-3 flex items-center gap-1.5 text-xs text-neutral-400 bg-neutral-800/80 px-3 py-1.5 rounded-full border border-neutral-700/50 group-hover:border-cyan-800 group-hover:text-cyan-300 transition-colors"
  >
    <MonitorPlay class="w-3.5 h-3.5" />
    <span>ช่อง YouTube</span>
  </div>
</Card.Root>
