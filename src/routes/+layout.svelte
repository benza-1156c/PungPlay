<script lang="ts">
  import "./layout.css";
  import {
    Home,
    Compass,
    Library,
    User,
    Heart,
    Settings,
    Play,
    SkipBack,
    SkipForward,
    Volume2,
    Search,
    ListMusic,
  } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import { Slider } from "$lib/components/ui/slider/index.js";
  import * as Avatar from "$lib/components/ui/avatar/index.js";

  const { children } = $props();

  const playlists = [
    { name: "Liked Songs", count: 102, icon: Heart, color: "text-red-500" },
    {
      name: "Immaculate Vibes",
      count: 32,
      icon: ListMusic,
      color: "text-yellow-500",
    },
    { name: "Funkyyy", count: 16, icon: ListMusic, color: "text-pink-500" },
    {
      name: "Songs for driving",
      count: 85,
      icon: ListMusic,
      color: "text-sky-500",
    },
    {
      name: "Sad vibes :(",
      count: 32,
      icon: ListMusic,
      color: "text-blue-500",
    },
    {
      name: "not loud enough",
      count: 25,
      icon: ListMusic,
      color: "text-purple-500",
    },
  ];

  let volume = $state(75);
  let progress = $state(35);
</script>

<div
  class="flex h-screen overflow-hidden bg-zinc-950 text-zinc-100 font-sans selection:bg-yellow-500/30"
>
  <!-- Sidebar -->
  <aside
    class="w-65 bg-zinc-950 flex flex-col border-r border-zinc-900/50 z-10"
  >
    <!-- Logo -->
    <div class="h-20 flex items-center px-6">
      <div class="flex items-center gap-2 font-bold text-xl tracking-tight">
        <div
          class="w-6 h-6 bg-yellow-500 rounded-sm rounded-tl-xl flex items-center justify-center"
        >
          <div class="w-2 h-2 bg-zinc-950 rounded-full"></div>
        </div>
        musa
      </div>
    </div>

    <!-- Main Nav -->
    <nav class="px-3 space-y-1">
      <Button
        href="/"
        variant="ghost"
        class="w-full justify-start gap-4 text-zinc-100 bg-zinc-900/50 hover:bg-zinc-900 hover:text-white h-12 rounded-xl"
      >
        <Home class="w-5 h-5" />
        <span class="font-medium text-base">Home</span>
      </Button>
      <Button
        href="/explore"
        variant="ghost"
        class="w-full justify-start gap-4 text-zinc-400 hover:text-white hover:bg-zinc-900 h-12 rounded-xl"
      >
        <Compass class="w-5 h-5" />
        <span class="font-medium text-base">Explore</span>
      </Button>
      <Button
        href="/library"
        variant="ghost"
        class="w-full justify-start gap-4 text-zinc-400 hover:text-white hover:bg-zinc-900 h-12 rounded-xl"
      >
        <Library class="w-5 h-5" />
        <span class="font-medium text-base">Library</span>
      </Button>
      <Button
        href="/profile"
        variant="ghost"
        class="w-full justify-start gap-4 text-zinc-400 hover:text-white hover:bg-zinc-900 h-12 rounded-xl"
      >
        <User class="w-5 h-5" />
        <span class="font-medium text-base">Profile</span>
      </Button>
    </nav>

    <!-- Playlists -->
    <div
      class="mt-8 px-6 flex items-center justify-between text-zinc-100 font-semibold tracking-wide"
    >
      Playlists
      <button class="text-zinc-500 hover:text-white transition-colors">+</button
      >
    </div>

    <ScrollArea class="flex-1 mt-4 px-3">
      <div class="space-y-1 pb-24">
        {#each playlists as playlist}
          <button
            class="w-full flex items-center gap-3 p-2 rounded-lg hover:bg-zinc-900 transition-colors text-left group"
          >
            <div
              class="w-10 h-10 rounded-md bg-zinc-900 flex items-center justify-center group-hover:bg-zinc-800 transition-colors"
            >
              <playlist.icon class="w-5 h-5 {playlist.color}" />
            </div>
            <div class="flex-1 overflow-hidden">
              <div
                class="text-sm font-medium text-zinc-200 truncate group-hover:text-white"
              >
                {playlist.name}
              </div>
              <div class="text-xs text-zinc-500 truncate">
                {playlist.count} Songs
              </div>
            </div>
          </button>
        {/each}
      </div>
    </ScrollArea>
  </aside>

  <!-- Main Content Area -->
  <main
    class="flex-1 flex flex-col min-w-0 overflow-hidden bg-zinc-950 relative pb-24"
  >
    {@render children()}
  </main>

  <!-- Bottom Player -->
  <div
    class="fixed bottom-0 left-0 right-0 h-24 bg-zinc-950/80 backdrop-blur-xl border-t border-zinc-900/80 flex items-center px-6 z-50"
  >
    <!-- Now Playing Info -->
    <div class="flex items-center gap-4 w-[30%] min-w-50">
      <img
        src="https://images.unsplash.com/photo-1614613535308-eb51bd3d2c17?w=100&q=80"
        alt="Cover"
        class="w-14 h-14 rounded-md object-cover shadow-lg"
      />
      <div class="overflow-hidden">
        <div class="text-sm font-semibold text-white truncate">
          Beautiful Up Here!
        </div>
        <div class="text-xs text-zinc-400 truncate mt-0.5">
          Betty Felicio • Windswept
        </div>
      </div>
    </div>

    <!-- Player Controls -->
    <div class="flex-1 flex flex-col items-center max-w-2xl mx-auto px-4">
      <div class="flex items-center gap-6">
        <button class="text-zinc-400 hover:text-white transition-colors">
          <SkipBack class="w-5 h-5 fill-current" />
        </button>
        <button
          class="w-10 h-10 bg-white rounded-full flex items-center justify-center text-black hover:scale-105 transition-transform"
        >
          <Play class="w-5 h-5 fill-current ml-1" />
        </button>
        <button class="text-zinc-400 hover:text-white transition-colors">
          <SkipForward class="w-5 h-5 fill-current" />
        </button>
      </div>

      <div class="w-full flex items-center gap-3 mt-2">
        <span class="text-[10px] text-zinc-500 font-medium w-8 text-right"
          >1:30</span
        >
        <Slider
          type="single"
          bind:value={progress}
          max={100}
          step={1}
          class="w-full"
        />
        <span class="text-[10px] text-zinc-500 font-medium w-8">3:12</span>
      </div>
    </div>

    <!-- Extra Controls -->
    <div class="flex items-center justify-end gap-4 w-[30%] min-w-50">
      <Volume2 class="w-5 h-5 text-zinc-400" />
      <Slider
        type="single"
        bind:value={volume}
        max={100}
        step={1}
        class="w-24"
      />
      <div class="flex items-center gap-3 ml-4">
        <Settings
          class="w-4 h-4 text-zinc-400 hover:text-white cursor-pointer"
        />
      </div>
    </div>
  </div>
</div>
