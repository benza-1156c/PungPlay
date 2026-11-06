<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { auth } from "../../stores/auth.svelte.js";
  import { LogIn, LogOut, CheckCircle2, Loader2 } from "@lucide/svelte";

  let { onLoginSuccess }: { onLoginSuccess?: () => void } = $props();

  $effect(() => {
    auth.init(() => {
      onLoginSuccess?.();
    });

    return () => {};
  });
</script>

<div class="inline-flex items-center gap-2">
  {#if auth.authStatus === "authenticated"}
    <div
      class="flex items-center gap-2 px-3 py-1.5 bg-emerald-950/40 border border-emerald-800/50 rounded-lg text-emerald-400 text-xs font-medium"
    >
      <CheckCircle2 class="w-4 h-4" />
      <span>Connected to YouTube Music</span>
      <button
        onclick={() => auth.logout()}
        class="ml-2 text-zinc-400 hover:text-red-400 transition-colors p-1"
        title="Sign out"
      >
        <LogOut class="w-3.5 h-3.5" />
      </button>
    </div>
  {:else}
    <Button
      variant="default"
      class="bg-red-600 hover:bg-red-700 text-white font-medium flex items-center gap-2 px-4 py-2 rounded-xl text-sm transition-all shadow-md shadow-red-900/20 disabled:opacity-60"
      disabled={auth.authStatus === "logging_in"}
      onclick={() => auth.login()}
    >
      {#if auth.authStatus === "logging_in"}
        <Loader2 class="w-4 h-4 animate-spin" />
        <span>Logging in...</span>
      {:else}
        <LogIn class="w-4 h-4" />
        <span>Login YouTube Music</span>
      {/if}
    </Button>
  {/if}

  {#if auth.err}
    <p class="text-xs text-red-400 mt-1">{auth.err}</p>
  {/if}
</div>
