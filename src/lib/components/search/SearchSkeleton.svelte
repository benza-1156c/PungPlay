<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";

  interface Props {
    count?: number;
  }

  let { count = 8 }: Props = $props();
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
  {#each Array(count) as _}
    <div class="space-y-3 rounded-xl bg-neutral-900/40 overflow-hidden">
      <Skeleton class="aspect-video w-full bg-neutral-800/60" />
      <div class="p-3 space-y-2">
        <Skeleton class="h-4 w-full bg-neutral-800/60" />
        <Skeleton class="h-4 w-3/4 bg-neutral-800/60" />
        <div class="flex items-center gap-2 pt-1">
          <Skeleton class="h-5 w-5 rounded-full bg-neutral-800/60" />
          <Skeleton class="h-3 w-24 bg-neutral-800/60" />
        </div>
      </div>
    </div>
  {/each}
</div>
