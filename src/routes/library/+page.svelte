<script lang="ts">
  import { Heart, ListMusic } from "@lucide/svelte";
</script>

<div class="p-8 w-full h-full text-white bg-zinc-950 overflow-y-auto">
  <h1 class="text-4xl font-bold mb-8">Your Library</h1>

  <div class="space-y-8">
    <div>
      <h2 class="text-2xl font-bold mb-4">Playlists</h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
        <div
          class="aspect-square bg-linear-to-br from-indigo-500 to-purple-600 rounded-xl p-6 flex flex-col justify-end relative group cursor-pointer"
        >
          <Heart class="w-10 h-10 mb-4 fill-white text-white shadow-sm" />
          <h3 class="text-xl font-bold text-white">Liked Songs</h3>
          <p class="text-sm text-white/80">102 Songs</p>
          <div
            class="absolute bottom-4 right-4 w-12 h-12 bg-green-500 rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity translate-y-2 group-hover:translate-y-0 shadow-xl"
          >
            <svg class="w-6 h-6 text-black fill-current" viewBox="0 0 24 24"
              ><path d="M8 5v14l11-7z" /></svg
            >
          </div>
        </div>

        {#each ["Immaculate Vibes", "Funkyyy", "Songs for driving", "Sad vibes :("] as playlist}
          <div
            class="aspect-square bg-zinc-900 rounded-xl p-4 flex flex-col relative group cursor-pointer hover:bg-zinc-800 transition-colors"
          >
            <div
              class="flex-1 bg-zinc-800 rounded-lg mb-3 flex items-center justify-center shadow-inner"
            >
              <ListMusic class="w-8 h-8 text-zinc-600" />
            </div>
            <h3 class="font-bold text-zinc-100 truncate">{playlist}</h3>
            <p class="text-xs text-zinc-500 mt-1">Playlist • You</p>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
