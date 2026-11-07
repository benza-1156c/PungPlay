<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import {
    House,
    Flame,
    Tv,
    Video,
    RotateCcwClock,
    Clock,
    ThumbsUp,
    Scissors,
    Settings,
    CircleHelp,
  } from "@lucide/svelte";

  let activeItem = $state("Home");

  const mainItems = [
    { title: "Home", icon: House },
    { title: "Shorts", icon: Flame },
    { title: "Subscriptions", icon: Tv },
  ];

  const libraryItems = [
    { title: "History", icon: RotateCcwClock },
    { title: "Watch later", icon: Clock },
    { title: "Liked videos", icon: ThumbsUp },
  ];

  const footerItems = [
    { title: "Settings", icon: Settings },
    { title: "Help & Feedback", icon: CircleHelp },
  ];
</script>

<Sidebar.Root
  class="top-8 h-[calc(100svh-2rem)] border-r border-white/10 bg-black/20 backdrop-blur-2xl"
>
  <Sidebar.Header class="p-4 flex flex-row items-center gap-2.5">
    <div
      class="flex items-center justify-center size-7 rounded-xl bg-red-600 text-white shadow-md shadow-red-600/30 shrink-0"
    >
      <svg class="size-3.5 fill-white" viewBox="0 0 24 24">
        <path d="M8 5v14l11-7z" />
      </svg>
    </div>
    <span class="font-bold text-lg tracking-tight text-white">YouTube</span>
  </Sidebar.Header>

  <Sidebar.Content class="px-2">
    <!-- Main Group -->
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each mainItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                onclick={() => (activeItem = item.title)}
                class="rounded-xl px-3 py-2 text-sm transition-all {activeItem ===
                item.title
                  ? 'bg-white/15 text-white font-medium shadow-sm backdrop-blur-md'
                  : 'text-white/70 hover:text-white hover:bg-white/10'}"
              >
                <item.icon class="size-4.5" />
                <span>{item.title}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator class="bg-white/10 my-2" />

    <!-- Library Group -->
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each libraryItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                onclick={() => (activeItem = item.title)}
                class="rounded-xl px-3 py-2 text-sm transition-all {activeItem ===
                item.title
                  ? 'bg-white/15 text-white font-medium shadow-sm backdrop-blur-md'
                  : 'text-white/70 hover:text-white hover:bg-white/10'}"
              >
                <item.icon class="size-4.5" />
                <span>{item.title}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator class="bg-white/10 my-2" />

    <!-- Footer Items -->
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each footerItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                onclick={() => (activeItem = item.title)}
                class="rounded-xl px-3 py-2 text-sm transition-all {activeItem ===
                item.title
                  ? 'bg-white/15 text-white font-medium shadow-sm backdrop-blur-md'
                  : 'text-white/70 hover:text-white hover:bg-white/10'}"
              >
                <item.icon class="size-4.5" />
                <span>{item.title}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>

  <Sidebar.Footer />
</Sidebar.Root>
