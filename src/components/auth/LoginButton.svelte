<script lang="ts">
  import { onMount } from "svelte";
  import { auth, login, logout, initAuth } from "../../stores/auth.svelte";
  import {
    LogIn,
    LogOut,
    CircleCheck,
    CircleAlert,
    LoaderCircle,
    RotateCw,
  } from "@lucide/svelte";

  onMount(() => {
    const cleanupPromise = initAuth();
    return () => cleanupPromise.then((cleanup) => cleanup());
  });
</script>

<div class="inline-flex items-center">
  {#if auth.status === "idle"}
    <button
      onclick={login}
      class="inline-flex items-center gap-2.5 px-5 py-2.5 bg-red-600 hover:bg-red-500 active:scale-95 text-white font-medium text-sm rounded-xl shadow-lg shadow-red-950/40 transition-all cursor-pointer"
    >
      <LogIn class="w-4 h-4" />
      <span>เข้าสู่ระบบ YouTube Music</span>
    </button>
  {:else if auth.status === "logging_in"}
    <div
      class="inline-flex items-center gap-3 px-5 py-2.5 bg-zinc-900 border border-zinc-800 text-zinc-300 font-medium text-sm rounded-xl shadow-inner"
    >
      <LoaderCircle class="w-4 h-4 text-red-500 animate-spin" />
      <span>กำลังเปิดหน้าต่างล็อกอิน...</span>
    </div>
  {:else if auth.status === "error"}
    <div
      class="flex flex-col sm:flex-row items-center gap-3 p-3 bg-red-950/30 border border-red-900/50 rounded-xl"
    >
      <div class="flex items-center gap-2 text-red-400 text-sm">
        <CircleAlert class="w-4 h-4 shrink-0" />
        <span>{auth.err || "เกิดข้อผิดพลาดในการล็อกอิน"}</span>
      </div>
      <button
        onclick={login}
        class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-red-600 hover:bg-red-500 text-white text-xs font-medium rounded-lg transition-colors cursor-pointer"
      >
        <RotateCw class="w-3.5 h-3.5" />
        <span>ลองใหม่</span>
      </button>
    </div>
  {:else if auth.status === "authenticated"}
    <div
      class="inline-flex items-center gap-3 px-4 py-2 bg-emerald-950/40 border border-emerald-800/50 rounded-xl text-emerald-400 text-sm font-medium shadow-sm"
    >
      <div class="flex items-center gap-2">
        <CircleCheck class="w-4 h-4" />
        <span>พร้อมดึงเพลงจาก YouTube Music!</span>
      </div>
      <button
        onclick={logout}
        class="inline-flex items-center gap-1 text-zinc-400 hover:text-red-400 text-xs px-2.5 py-1 rounded-lg hover:bg-zinc-800/60 transition-colors cursor-pointer ml-1"
        title="ออกจากระบบ"
      >
        <LogOut class="w-3.5 h-3.5" />
        <span>ออกจากระบบ</span>
      </button>
    </div>
  {/if}
</div>
