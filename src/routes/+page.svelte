<script lang="ts">
  import LoginButton from "../components/auth/LoginButton.svelte";
</script>

<div
  class="min-h-screen bg-zinc-950 text-white flex flex-col items-center justify-center p-8 gap-6 selection:bg-yellow-500/30"
>
  <div class="text-center space-y-2">
    <h1 class="text-3xl font-bold tracking-tight text-yellow-500">PungPlay</h1>
    <p class="text-zinc-400 text-sm">YouTube Music Desktop Player</p>
  </div>

  <div
    class="p-6 bg-zinc-900/60 border border-zinc-800/80 rounded-2xl flex flex-col items-center gap-4 shadow-xl"
  >
    <LoginButton />
  </div>
</div>
