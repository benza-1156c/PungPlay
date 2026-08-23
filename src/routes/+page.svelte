<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>
  <form class="gap-2" onsubmit={greet}>
    <Input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
      class="w-64"
    />
    <Button type="submit">Greet</Button>
  </form>
  <p>{greetMsg}</p>
</main>

<style>
</style>
