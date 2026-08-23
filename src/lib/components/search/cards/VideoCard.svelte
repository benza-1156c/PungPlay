<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import { SquarePlay, ListVideo, User, Clock, Eye } from "@lucide/svelte";
  import type { SearchResultItem } from "$lib/bindings.js";

  interface Props {
    item: SearchResultItem;
  }

  let { item }: Props = $props();

  function formatViews(views: number): string {
    if (views >= 1_000_000) return `${(views / 1_000_000).toFixed(1)}M`;
    if (views >= 1_000) return `${(views / 1_000).toFixed(1)}K`;
    return views.toString();
  }

  const typeIcon = {
    video: SquarePlay,
    playlist: ListVideo,
    channel: User,
    unknown: SquarePlay,
  };

  const typeLabel = {
    video: "วิดีโอ",
    playlist: "เพลย์ลิสต์",
    channel: "ช่อง",
    unknown: "อื่นๆ",
  };
</script>

<Card.Root
  class="group bg-neutral-900/60 border-neutral-800/50 overflow-hidden hover:border-cyan-500/40 transition-all duration-300 cursor-pointer rounded-xl hover:shadow-lg hover:shadow-cyan-900/10"
>
  <!-- Thumbnail -->
  <div class="relative aspect-video overflow-hidden bg-neutral-950">
    {#if item.thumbnail}
      <img
        src={item.thumbnail}
        alt={item.title}
        class="object-cover w-full h-full group-hover:scale-105 transition-transform duration-500"
        loading="lazy"
        referrerpolicy="no-referrer"
      />
    {:else}
      {@const Icon = typeIcon[item.item_type]}
      <div
        class="flex items-center justify-center w-full h-full text-neutral-700"
      >
        <Icon class="w-12 h-12" />
      </div>
    {/if}

    <!-- Duration badge -->
    {#if item.duration}
      <div class="absolute bottom-2 right-2">
        <Badge
          variant="secondary"
          class="bg-black/80 text-white text-xs font-mono border-0"
        >
          {item.duration}
        </Badge>
      </div>
    {/if}

    <!-- Type badge -->
    <div class="absolute top-2 left-2">
      <Badge
        variant={item.item_type === "video"
          ? "default"
          : item.item_type === "playlist"
            ? "secondary"
            : "outline"}
        class="text-xs"
      >
        {typeLabel[item.item_type]}
      </Badge>
    </div>
  </div>

  <!-- Info -->
  <div class="p-3 space-y-2">
    <Card.Title
      class="text-sm font-medium leading-snug line-clamp-2 text-neutral-100 group-hover:text-cyan-400 transition-colors"
      title={item.title}
    >
      {item.title}
    </Card.Title>

    {#if item.artist}
      <div class="flex items-center gap-2">
        <Avatar.Root class="w-5 h-5">
          <Avatar.Fallback class="bg-neutral-700 text-[10px] text-neutral-300">
            {item.artist.charAt(0).toUpperCase()}
          </Avatar.Fallback>
        </Avatar.Root>
        <span class="text-xs text-neutral-400 truncate">{item.artist}</span>
      </div>
    {/if}

    {#if item.views > 0 || item.duration}
      <div class="flex items-center gap-3 text-xs text-neutral-500">
        {#if item.views > 0}
          <span class="flex items-center gap-1">
            <Eye class="w-3 h-3" />
            {formatViews(item.views)} views
          </span>
        {/if}
        {#if item.duration}
          <span class="flex items-center gap-1">
            <Clock class="w-3 h-3" />
            {item.duration}
          </span>
        {/if}
      </div>
    {/if}
  </div>
</Card.Root>
