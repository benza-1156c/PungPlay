<script lang="ts">
  import { commands } from "$lib/bindings.js";
  import type { SearchResultItem } from "$lib/bindings.js";
  import SearchResultCard from "$lib/components/search/SearchResultCard.svelte";
  import SearchSkeleton from "$lib/components/search/SearchSkeleton.svelte";
  import { Search, Play, MoreHorizontal, Shuffle, Radio } from "@lucide/svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

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

  // Dummy Data for the reference UI
  const topSongs = [
    {
      title: "Four loves left",
      artist: "Angsty Bird",
      plays: "880K",
      cover:
        "https://images.unsplash.com/photo-1614613535308-eb51bd3d2c17?w=100&q=80",
    },
    {
      title: "Your turn",
      artist: "Angsty Bird",
      plays: "410K",
      cover:
        "https://images.unsplash.com/photo-1614613535308-eb51bd3d2c17?w=100&q=80",
    },
    {
      title: "Every Thursday",
      artist: "Keeping Quiet",
      plays: "98K",
      cover:
        "https://images.unsplash.com/photo-1493225457124-a1a2a5956093?w=100&q=80",
    },
    {
      title: "Heartwarm",
      artist: "Ambiguity",
      plays: "88K",
      cover:
        "https://images.unsplash.com/photo-1614613535308-eb51bd3d2c17?w=100&q=80",
    },
  ];
</script>

<div class="flex h-full w-full">
  <!-- Main Content (Left) -->
  <div class="flex-1 h-full overflow-y-auto custom-scrollbar">
    {#if hasSearched || isLoading}
      <!-- Search Results State -->
      <div class="p-8">
        <h2 class="text-3xl font-bold mb-6">
          Search Results for "{searchQuery}"
        </h2>

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
          <p class="text-sm text-zinc-500 mb-5">
            Found {results.length} results
          </p>
          <div
            class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5 pb-10"
          >
            {#each results as item (item.id)}
              <SearchResultCard {item} />
            {/each}
          </div>
        {:else}
          <div class="text-center text-zinc-500 mt-20">No results found.</div>
        {/if}
      </div>
    {:else}
      <!-- Default Hero State (Matches Reference Image) -->
      <div
        class="relative h-[60vh] min-h-100 w-full bg-yellow-500 overflow-hidden flex flex-col justify-end p-10"
      >
        <!-- Background Image overlay -->
        <div
          class="absolute inset-0 bg-linear-to-t from-black/80 via-black/20 to-transparent z-10"
        ></div>
        <img
          src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=1000&auto=format&fit=crop"
          alt="Betty Felicio"
          class="absolute inset-0 w-full h-full object-cover mix-blend-overlay opacity-90"
        />

        <div class="relative z-20">
          <h1
            class="text-6xl sm:text-8xl font-black text-white tracking-tighter mb-4"
          >
            Betty Felicio
          </h1>
        </div>
      </div>

      <!-- Albums Row -->
      <div class="p-8">
        <h3 class="text-xl font-bold text-white mb-4">Albums</h3>
        <div class="flex gap-4 overflow-x-auto pb-4 custom-scrollbar">
          {#each [1, 2, 3, 4, 5] as i}
            <div class="min-w-40 cursor-pointer group">
              <div class="w-40 h-40 rounded-xl overflow-hidden mb-3 relative">
                <img
                  src={`https://images.unsplash.com/photo-1614613535308-eb51bd3d2c17?w=300&q=80&sig=${i}`}
                  alt="Album"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                />
                <div
                  class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"
                >
                  <Play class="w-10 h-10 text-white fill-current" />
                </div>
              </div>
              <p class="font-semibold text-sm text-zinc-100 truncate">
                Beautiful Up Here!
              </p>
              <p class="text-xs text-zinc-500 mt-1">2023 • Album</p>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <!-- Right Sidebar -->
  <div
    class="w-80 bg-zinc-950 border-l border-zinc-900/50 h-full overflow-y-auto hidden lg:block custom-scrollbar"
  >
    <!-- Search Bar -->
    <div class="p-6 sticky top-0 bg-zinc-950/90 backdrop-blur-md z-10">
      <form onsubmit={performSearch} class="relative">
        <Search
          class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500"
        />
        <Input
          bind:value={searchQuery}
          placeholder="Search albums, artists, songs"
          class="pl-10 bg-zinc-900 border-none text-zinc-100 h-11 rounded-full placeholder:text-zinc-600 focus-visible:ring-yellow-500"
        />
      </form>
    </div>

    <!-- Artist Details (Matching Reference) -->
    <div class="px-6 pb-6">
      <!-- Profile Header -->
      <div class="flex items-center gap-4 mb-6">
        <img
          src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=100&auto=format&fit=crop"
          alt="Avatar"
          class="w-16 h-16 rounded-full object-cover border border-zinc-800"
        />
        <div>
          <h3 class="font-bold text-lg text-white flex items-center gap-1">
            Betty Felicio
            <svg
              class="w-4 h-4 text-blue-400"
              viewBox="0 0 24 24"
              fill="currentColor"
              ><path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
              /></svg
            >
          </h3>
          <p class="text-xs text-zinc-400">New Zealand songwriter</p>
        </div>
      </div>

      <div class="flex items-center justify-between mb-8">
        <div class="text-xs text-zinc-500">
          <span class="text-white font-medium">79K</span> Followers
        </div>
        <Button
          variant="outline"
          size="sm"
          class="rounded-full border-zinc-800 text-zinc-300 hover:bg-zinc-800 hover:text-white h-8 text-xs"
          >Follow</Button
        >
      </div>

      <!-- About -->
      <div class="mb-8">
        <h4 class="font-bold text-white mb-2">About</h4>
        <p class="text-xs text-zinc-400 leading-relaxed">
          Hailing from the coastal town of Napier, New Zealand, Betty Felicio
          blends dreamy alt-pop with earthy acoustic textures and introspective
          lyrics... <span class="text-white cursor-pointer hover:underline"
            >More</span
          >
        </p>
      </div>

      <!-- Top Songs -->
      <div>
        <div class="flex items-center justify-between mb-4">
          <h4 class="font-bold text-white">Top Songs</h4>
          <div class="flex gap-2">
            <button
              class="text-xs text-zinc-400 hover:text-white flex items-center gap-1"
              ><Shuffle class="w-3 h-3" /> Shuffle</button
            >
            <button
              class="text-xs text-zinc-400 hover:text-white flex items-center gap-1"
              ><Radio class="w-3 h-3" /> Radio</button
            >
          </div>
        </div>

        <div class="space-y-3">
          {#each topSongs as song}
            <div
              class="flex items-center gap-3 group cursor-pointer p-1 rounded-md hover:bg-zinc-900 transition-colors"
            >
              <div class="relative w-10 h-10 rounded-md overflow-hidden">
                <img
                  src={song.cover}
                  alt="cover"
                  class="w-full h-full object-cover"
                />
                <div
                  class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center"
                >
                  <Play class="w-4 h-4 text-white fill-current" />
                </div>
              </div>
              <div class="flex-1 overflow-hidden">
                <p
                  class="text-sm font-medium text-zinc-200 truncate group-hover:text-yellow-400"
                >
                  {song.title}
                </p>
                <div class="flex items-center gap-2 mt-0.5">
                  <p class="text-[10px] text-zinc-500 truncate">
                    {song.artist}
                  </p>
                  <span class="w-1 h-1 bg-zinc-700 rounded-full"></span>
                  <p class="text-[10px] text-zinc-500">{song.plays} plays</p>
                </div>
              </div>
              <button
                class="text-zinc-600 opacity-0 group-hover:opacity-100 hover:text-white px-2"
              >
                <MoreHorizontal class="w-4 h-4" />
              </button>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  /* Custom scrollbar for better visual blending */
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
    height: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: #27272a; /* zinc-800 */
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #3f3f46; /* zinc-700 */
  }
</style>
