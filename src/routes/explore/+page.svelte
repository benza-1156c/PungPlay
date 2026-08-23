<div class="p-8 w-full h-full text-white bg-zinc-950 overflow-y-auto">
  <h1 class="text-4xl font-bold mb-8">Explore</h1>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
    {#each ["Pop", "Hip-Hop", "Rock", "R&B", "Electronic", "Jazz", "Classical", "K-Pop"] as genre, i}
      <div class="h-32 rounded-xl bg-linear-to-br from-zinc-800 to-zinc-900 border border-zinc-800 p-4 relative overflow-hidden group cursor-pointer hover:border-yellow-500/50 transition-colors">
        <h3 class="text-lg font-bold z-10 relative">{genre}</h3>
        <div class="absolute -right-4 -bottom-4 w-20 h-20 bg-yellow-500/20 rounded-full blur-xl group-hover:bg-yellow-500/40 transition-colors"></div>
      </div>
    {/each}
  </div>
</div>
