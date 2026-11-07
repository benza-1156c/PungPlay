<script lang="ts">
  import { onMount } from "svelte";
  import Header from "../components/home/Header.svelte";
  import { commands, type FeedVideo } from "$lib/generated/bindings";

  let videos = $state<FeedVideo[]>([]);
  let continuation = $state<string | null>(null);
  let isLoading = $state(false);
  let hasMore = $state(true);
  let sentinel: HTMLDivElement | null = $state(null);

  async function loadFeed(isInitial = false) {
    if (isLoading || (!isInitial && !hasMore)) return;
    isLoading = true;

    try {
      const res = await commands.fetchYoutubeFeed(
        isInitial ? null : continuation,
      );
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

  <main class="flex-1 p-6">
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4"
    >
      {#each videos as video (video.id)}
        <div class="flex flex-col gap-2 group cursor-pointer">
          <div class="relative aspect-video rounded-xl overflow-hidden">
            {#if video.thumbnail}
              <img
                src={video.thumbnail}
                alt={video.title}
                class="w-full h-full object-cover group-hover:scale-101 transition-transform duration-200"
                loading="lazy"
              />
            {/if}
            {#if video.duration_text}
              <span
                class="absolute bottom-2 right-2 text-white text-xs px-1.5 py-0.5 rounded font-medium"
              >
                {video.duration_text}
              </span>
            {/if}
          </div>

          <div class="flex gap-3">
            {#if video.channel_avatar}
              <img
                src={video.channel_avatar}
                alt={video.channel ?? ""}
                class="w-9 h-9 rounded-full object-cover shrink-0"
              />
            {:else}
              <div class="w-9 h-9 rounded-full bg-muted shrink-0"></div>
            {/if}

            <div class="flex flex-col min-w-0">
              <h3
                class="font-medium text-sm line-clamp-2 leading-tight group-hover:text-primary"
                title={video.title}
              >
                {video.title}
              </h3>
              {#if video.channel}
                <p class="text-xs text-muted-foreground mt-1 truncate">
                  {video.channel}
                </p>
              {/if}
              <div
                class="flex items-center gap-1 text-xs text-muted-foreground"
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
      class="py-8 flex justify-center items-center w-full"
    >
      {#if isLoading}
        <div class="text-sm text-muted-foreground animate-pulse">
          กำลังโหลดวิดีโอเพิ่มเติม...
        </div>
      {:else if !hasMore && videos.length > 0}
        <div class="text-xs text-muted-foreground">
          ไม่มีวิดีโอเพิ่มเติมแล้ว
        </div>
      {/if}
    </div>
  </main>
</div>
