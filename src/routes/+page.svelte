<script lang="ts">
  import { onMount } from "svelte";
  import Header from "../components/home/Header.svelte";
  import { commands } from "$lib/generated/bindings";

  async function loadFeed() {
    try {
      const res = await commands.fetchYoutubeFeed();
      console.log(res);
    } catch (e) {}
  }

  onMount(() => {
    loadFeed();
  });
</script>

<div>
  <Header />
</div>
