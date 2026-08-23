<script lang="ts">
  import { commands } from "$lib/bindings.js";
  import type { SearchResultItem } from "$lib/bindings.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import SearchBar from "$lib/components/search/SearchBar.svelte";
  import SearchResultCard from "$lib/components/search/SearchResultCard.svelte";
  import SearchSkeleton from "$lib/components/search/SearchSkeleton.svelte";
  import EmptyState from "$lib/components/search/EmptyState.svelte";
  import { Music } from "@lucide/svelte";

  let searchQuery = $state("");
  let results = $state<SearchResultItem[]>([]);
  let isLoading = $state(false);
  let errorMsg = $state("");
  let hasSearched = $state(false);

  async function performSearch(event: Event) {
    event.preventDefault();
    if (!searchQuery.trim()) return;

    isLoading = true;
    errorMsg = "";
    results = [];
    hasSearched = true;

    try {
      const res = await commands.searchYoutube(searchQuery);
      if (res.status === "ok") {
        results = res.data;
      } else {
        errorMsg = res.error;
      }
    } catch (err: any) {
      errorMsg = err.toString();
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="min-h-screen bg-neutral-950 text-white">
  <!-- Header -->
  <header
    class="sticky top-0 z-50 bg-neutral-950/80 backdrop-blur-xl border-b border-neutral-800/40"
  >
    <div
      class="max-w-7xl mx-auto px-6 py-4 flex flex-col sm:flex-row items-center gap-4"
    >
      <!-- Logo -->
      <div class="flex items-center gap-2 shrink-0">
        <div class="p-1.5 rounded-lg bg-cyan-600/20">
          <Music class="w-5 h-5 text-cyan-400" />
        </div>
        <span class="text-lg font-bold tracking-tight text-white">PungPlay</span
        >
      </div>

      <Separator
        orientation="vertical"
        class="h-6 hidden sm:block bg-neutral-800"
      />

      <!-- Search -->
      <div class="w-full sm:flex-1">
        <SearchBar
          bind:value={searchQuery}
          {isLoading}
          onsubmit={performSearch}
        />
      </div>
    </div>
  </header>

  <!-- Content -->
  <ScrollArea class="h-[calc(100vh-73px)]">
    <div class="max-w-7xl mx-auto px-6 py-8">
      <!-- Error -->
      {#if errorMsg}
        <div
          class="p-4 mb-6 bg-red-950/50 border border-red-800/50 rounded-xl text-red-300 text-center text-sm"
        >
          {errorMsg}
        </div>
      {/if}

      {#if isLoading}
        <SearchSkeleton />
      {:else if results.length > 0}
        <!-- Results count -->
        <p class="text-sm text-neutral-500 mb-5">
          พบ {results.length} ผลลัพธ์
        </p>

        <div
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5"
        >
          {#each results as item (item.id)}
            <SearchResultCard {item} />
          {/each}
        </div>
      {:else}
        <EmptyState />
      {/if}
    </div>
  </ScrollArea>
</main>
