<script lang="ts">
  import { commands } from "$lib/bindings.js";
  import type { SearchResultItem } from "$lib/bindings.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Search, SquarePlay, ListVideo, User } from "@lucide/svelte";

  let searchQuery = $state("");
  let results = $state<SearchResultItem[]>([]);
  let isLoading = $state(false);
  let errorMsg = $state("");

  async function performSearch(event: Event) {
    event.preventDefault();
    if (!searchQuery.trim()) return;

    isLoading = true;
    errorMsg = "";
    results = [];

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

<main class="min-h-screen bg-neutral-950 text-white p-6 md:p-12">
  <div class="max-w-7xl mx-auto space-y-8">
    <!-- Header -->
    <div class="text-center space-y-4">
      <h1
        class="text-4xl md:text-6xl font-black tracking-tight bg-linear-to-r from-cyan-400 to-blue-600 bg-clip-text text-transparent inline-flex items-center gap-4"
      >
        <Search class="w-10 h-10 md:w-14 md:h-14 text-cyan-400" />
        PungSearch
      </h1>
      <p class="text-neutral-400 text-lg">
        Search for YouTube Videos, Playlists, or Channels
      </p>
    </div>

    <!-- Search Form -->
    <form
      class="flex w-full max-w-2xl mx-auto items-center space-x-2"
      onsubmit={performSearch}
    >
      <Input
        id="search-query"
        type="text"
        placeholder="Search anything..."
        bind:value={searchQuery}
        class="flex-1 bg-neutral-900 border-neutral-800 text-white placeholder:text-neutral-500 focus-visible:ring-cyan-500 text-base py-6"
      />
      <Button
        type="submit"
        disabled={isLoading}
        class="bg-cyan-600 hover:bg-cyan-500 text-white py-6 px-6"
      >
        {#if isLoading}
          Searching...
        {:else}
          <Search class="w-5 h-5 mr-2" />
          Search
        {/if}
      </Button>
    </form>

    <!-- Error Message -->
    {#if errorMsg}
      <div
        class="p-4 bg-red-900/50 border border-red-500 rounded-lg text-red-200 text-center max-w-2xl mx-auto shadow-lg shadow-red-900/20"
      >
        {errorMsg}
      </div>
    {/if}

    <!-- Content Area -->
    <div class="pt-8">
      {#if isLoading}
        <div
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
        >
          {#each Array(8) as _}
            <div class="space-y-3">
              <Skeleton class="h-48 w-full rounded-xl bg-neutral-800" />
              <div class="space-y-2 pt-2">
                <Skeleton class="h-4 w-3/4 bg-neutral-800" />
                <Skeleton class="h-4 w-1/2 bg-neutral-800" />
              </div>
            </div>
          {/each}
        </div>
      {:else if results.length > 0}
        <div
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
        >
          {#each results as item}
            <Card.Root
              class="group bg-neutral-900 border-neutral-800 overflow-hidden hover:border-cyan-500/50 transition-colors cursor-pointer relative rounded-xl shadow-lg hover:shadow-cyan-900/20"
            >
              <div
                class="relative aspect-video overflow-hidden bg-neutral-950 flex items-center justify-center"
              >
                {#if item.thumbnail}
                  <img
                    src={item.thumbnail}
                    alt={item.title}
                    class="object-cover w-full h-full group-hover:scale-105 transition-transform duration-500"
                    loading="lazy"
                  />
                {:else}
                  <div class="text-neutral-700 w-12 h-12">
                    {#if item.item_type === "video"}
                      <SquarePlay class="w-full h-full" />
                    {:else if item.item_type === "playlist"}
                      <ListVideo class="w-full h-full" />
                    {:else}
                      <User class="w-full h-full" />
                    {/if}
                  </div>
                {/if}

                <div class="absolute top-2 right-2">
                  <Badge
                    variant={item.item_type === "video"
                      ? "default"
                      : item.item_type === "playlist"
                        ? "secondary"
                        : "outline"}
                  >
                    {item.item_type.toUpperCase()}
                  </Badge>
                </div>
              </div>
              <Card.Header class="p-4">
                <Card.Title
                  class="text-sm font-medium leading-relaxed line-clamp-2 text-neutral-200 group-hover:text-cyan-400 transition-colors"
                  title={item.title}
                >
                  {item.title}
                </Card.Title>
              </Card.Header>
            </Card.Root>
          {/each}
        </div>
      {:else}
        <div
          class="flex flex-col items-center justify-center py-20 text-neutral-500 space-y-4 border-2 border-dashed border-neutral-800 rounded-3xl max-w-4xl mx-auto"
        >
          <Search class="w-16 h-16 opacity-20" />
          <p class="text-lg">Search for something to see results.</p>
        </div>
      {/if}
    </div>
  </div>
</main>
