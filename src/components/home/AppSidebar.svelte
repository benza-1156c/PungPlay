<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import {
    Compass,
    Flame,
    Clock,
    ThumbsUp,
    Settings,
    Play,
    CircleExclamationPoint,
    RotateCcwClock,
  } from "@lucide/svelte";

  const mainItems = [
    { title: "Explore", icon: Compass },
    { title: "Trending", icon: Flame },
  ];

  const libraryItems = [
    { title: "History", icon: RotateCcwClock },
    { title: "Watch later", icon: Clock },
    { title: "Likes videos", icon: ThumbsUp },
  ];

  const footerItems = [
    { title: "Settings", icon: Settings },
    { title: "Send feedback", icon: CircleExclamationPoint },
  ];
</script>

<Sidebar.Root>
  <Sidebar.Header class="p-4 flex flex-row items-center gap-2.5">
    <div
      class="flex items-center justify-center size-7 rounded-lg bg-blue-600 text-white shrink-0"
    >
      <Play class="size-3.5 fill-white" />
    </div>
    <span class="font-bold text-lg tracking-tight">You</span>
  </Sidebar.Header>

  <Sidebar.Content>
    <!-- Main Group -->
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each mainItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                <item.icon class="size-4" />
                <span>{item.title}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator />

    <!-- Library Group -->
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each libraryItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                <item.icon class="size-4" />
                <span>{item.title}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator />

    <!-- Subscriptions Group -->
    <Sidebar.Group>
      <Sidebar.GroupLabel>SUBSCRIPTIONS</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          <!-- User custom subscriptions -->
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>

    <Sidebar.Separator />

    <!-- More From YouTube -->
    <Sidebar.Group>
      <Sidebar.GroupLabel>MORE FROM YOUTUBE</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each footerItems as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                <item.icon class="size-4" />
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
