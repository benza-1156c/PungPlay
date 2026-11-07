<script lang="ts">
  import { onMount } from "svelte";
  import Header from "../components/home/Header.svelte";
  import { commands, type FeedVideo } from "$lib/generated/bindings";
  import { Check, CircleMinus, Loader } from "@lucide/svelte";

  let videos = $state<FeedVideo[]>([]);
  let continuation = $state<string | null>(null);
  let isLoading = $state(false);
  let hasMore = $state(true);
  let sentinel: HTMLDivElement | null = $state(null);

  const categories = [
    "All",
    "Gaming",
    "Music",
    "Sports leagues",
    "Mixes",
    "Live",
    "Podcasts",
    "Technology",
    "Animation",
  ];
  let selectedCategory = $state("All");

  async function loadFeed(isInitial = false) {
    if (isLoading || (!isInitial && !hasMore)) return;
    isLoading = true;

    try {
      let res;
      if (selectedCategory === "All") {
        res = await commands.fetchYoutubeFeed(isInitial ? null : continuation);
      } else {
        res = await commands.searchYoutube(
          selectedCategory,
          isInitial ? null : continuation,
        );
      }

      if (res.status === "ok") {
        if (isInitial) {
          videos = res.data.videos;
        } else {
          videos = [...videos, ...res.data.videos];
        }
        continuation = res.data.continuation;
        hasMore = continuation !== null;
      }
    } catch (e) {
      console.error("Failed to load feed:", e);
    } finally {
      isLoading = false;
    }
  }

  function handleSelectCategory(cat: string) {
    if (selectedCategory === cat) return;
    selectedCategory = cat;
    continuation = null;
    hasMore = true;
    videos = [];
    loadFeed(true);
  }

  onMount(() => {
    loadFeed(true);

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && !isLoading && hasMore) {
          loadFeed();
        }
      },
      { rootMargin: "300px" },
    );

    if (sentinel) {
      observer.observe(sentinel);
    }

    return () => {
      observer.disconnect();
    };
  });
</script>

<div class="flex flex-col min-h-screen">
  <Header />

  <main class="flex-1 px-8 py-6">
    <!-- Category Pills (visionOS Frosted Glass Style) -->
    <div
      class="flex items-center gap-2.5 overflow-x-auto pb-6 pt-1 select-none no-scrollbar"
    >
      {#each categories as category}
        <button
          type="button"
          onclick={() => handleSelectCategory(category)}
          class="rounded-full px-4 py-1.5 text-xs font-medium transition-all duration-200 shrink-0 cursor-pointer {selectedCategory ===
          category
            ? 'bg-white text-black font-semibold shadow-md shadow-white/10'
            : 'bg-white/10 hover:bg-white/15 text-white/90 border border-white/5 backdrop-blur-xl'}"
        >
          {category}
        </button>
      {/each}
    </div>

    <!-- Video Grid -->
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-5"
    >
      {#each videos as video (video.id)}
        <div class="flex flex-col gap-2.5 group cursor-pointer">
          <!-- Thumbnail with rounded-2xl and glass duration tag -->
          <div
            class="relative aspect-video rounded-2xl overflow-hidden bg-white/5 border border-white/10 shadow-lg shadow-black/20"
          >
            {#if video.thumbnail}
              <img
                src={video.thumbnail}
                alt={video.title}
                class="w-full h-full object-cover group-hover:scale-[1.03] transition-transform duration-300 ease-out"
                loading="lazy"
              />
            {/if}
            {#if video.duration_text}
              <span
                class="absolute bottom-2.5 right-2.5 bg-black/75 backdrop-blur-md text-white text-[11px] px-2 py-0.5 rounded-md font-medium tracking-tight shadow-sm"
              >
                {video.duration_text}
              </span>
            {/if}
          </div>

          <!-- Video Info -->
          <div class="flex gap-3 px-0.5">
            {#if video.channel_avatar}
              <img
                src={video.channel_avatar}
                alt={video.channel ?? ""}
                class="w-9 h-9 rounded-full object-cover shrink-0 ring-1 ring-white/15 shadow-sm mt-0.5"
              />
            {:else}
              <div
                class="w-9 h-9 rounded-full bg-white/10 shrink-0 ring-1 ring-white/10 mt-0.5"
              ></div>
            {/if}

            <div class="flex flex-col min-w-0">
              <h3
                class="font-medium text-sm text-white line-clamp-2 leading-snug group-hover:text-primary/70 transition-colors"
                title={video.title}
              >
                {video.title}
              </h3>
              {#if video.channel}
                <div
                  class="flex items-center gap-1 text-xs text-white/60 mt-1 truncate"
                >
                  <span class=" transition-colors">{video.channel}</span>
                  <span
                    class="inline-flex items-center justify-center size-3 rounded-full bg-white/20 text-white text-[8px]"
                  >
                    <Check class="size-2" />
                  </span>
                </div>
              {/if}
              <div
                class="flex items-center gap-1.5 text-xs text-white/50 mt-0.5"
              >
                {#if video.view_count_text}
                  <span>{video.view_count_text}</span>
                {/if}
                {#if video.view_count_text && video.published_time}
                  <span>•</span>
                {/if}
                {#if video.published_time}
                  <span>{video.published_time}</span>
                {/if}
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Sentinel element for Infinite Scroll -->
    <div
      bind:this={sentinel}
      class="py-12 flex justify-center items-center w-full"
    >
      {#if isLoading}
        <div class="text-sm text-white/60 flex items-center gap-2">
          <Loader class="animate-spin" />
        </div>
      {:else if !hasMore && videos.length > 0}
        <div class="text-xs text-white/40">
          <CircleMinus />
        </div>
      {/if}
    </div>
  </main>
</div>
