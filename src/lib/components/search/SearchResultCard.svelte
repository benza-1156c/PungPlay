<script lang="ts">
  import type { SearchResultItem } from "$lib/bindings.js";
  import VideoCard from "./cards/VideoCard.svelte";
  import ChannelCard from "./cards/ChannelCard.svelte";

  interface Props {
    item: SearchResultItem;
  }

  let { item }: Props = $props();
</script>

{#if item.item_type === "channel"}
  <ChannelCard {item} />
{:else}
  <VideoCard {item} />
{/if}
