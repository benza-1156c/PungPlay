<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Search } from "@lucide/svelte";

  interface Props {
    value: string;
    isLoading: boolean;
    onsubmit: (e: Event) => void;
  }

  let { value = $bindable(), isLoading, onsubmit }: Props = $props();
</script>

<form
  class="flex w-full max-w-2xl mx-auto items-center gap-2"
  {onsubmit}
>
  <Input
    id="search-query"
    type="text"
    placeholder="ค้นหาเพลง, ศิลปิน, เพลย์ลิสต์..."
    bind:value
    class="flex-1 bg-neutral-900/80 border-neutral-700/50 text-white placeholder:text-neutral-500 focus-visible:ring-cyan-500/50 text-base h-12"
  />
  <Button
    type="submit"
    disabled={isLoading}
    class="bg-cyan-600 hover:bg-cyan-500 text-white h-12 px-6"
  >
    {#if isLoading}
      <span class="animate-spin mr-2">⟳</span>
      กำลังค้นหา...
    {:else}
      <Search class="w-4 h-4 mr-2" />
      ค้นหา
    {/if}
  </Button>
</form>
