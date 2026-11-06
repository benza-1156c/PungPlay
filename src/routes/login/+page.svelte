<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { auth, initAuth } from "../../stores/auth.svelte";
  import LoginButton from "../../components/auth/LoginButton.svelte";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
  } from "$lib/components/ui/card";
  import {
    LoaderCircle,
    CircleAlert,
    CircleCheck,
    Music,
  } from "@lucide/svelte";

  onMount(() => {
    const cleanupPromise = initAuth();
    return () => {
      cleanupPromise.then((cleanup) => cleanup?.());
    };
  });

  $effect(() => {
    if (auth.status === "authenticated") {
      goto("/");
    }
  });
</script>

<div
  class="min-h-screen bg-background text-foreground flex items-center justify-center p-6"
>
  <Card class="w-full max-w-md text-center shadow-2xl">
    <CardHeader class="flex flex-col items-center gap-3">
      <div
        class="w-14 h-14 rounded-2xl bg-red-600 flex items-center justify-center text-white shadow-lg shadow-red-950/40"
      >
        <Music class="w-7 h-7" />
      </div>
      <CardTitle class="text-2xl">
        เข้าสู่ระบบ <span class="text-yellow-500">PungPlay</span>
      </CardTitle>
      <CardDescription>
        เข้าสู่ระบบด้วยบัญชี YouTube Music เพื่อฟังเพลงและจัดการคลังเพลงของคุณ
      </CardDescription>
    </CardHeader>

    <CardContent class="flex flex-col items-center gap-4 pt-2">
      {#if auth.status === "logging_in"}
        <div
          class="inline-flex items-center gap-3 px-4 py-2.5 rounded-lg border bg-muted text-muted-foreground text-sm"
        >
          <LoaderCircle class="w-4 h-4 text-red-500 animate-spin" />
          <span>กำลังเปิดหน้าต่างล็อกอิน...</span>
        </div>
      {:else if auth.status === "error"}
        <div
          class="flex flex-col items-center gap-3 p-3 rounded-lg border border-destructive/50 bg-destructive/10 text-destructive text-sm w-full"
        >
          <div class="flex items-center gap-2">
            <CircleAlert class="w-4 h-4 shrink-0" />
            <span>{auth.err || "เกิดข้อผิดพลาดในการล็อกอิน"}</span>
          </div>
          <LoginButton />
        </div>
      {:else if auth.status === "authenticated"}
        <div
          class="inline-flex items-center gap-2 px-4 py-2.5 rounded-lg border border-emerald-500/50 bg-emerald-500/10 text-emerald-500 text-sm"
        >
          <CircleCheck class="w-4 h-4" />
          <span>เข้าสู่ระบบสำเร็จ กำลังไปหน้าหลัก...</span>
        </div>
      {:else}
        <LoginButton />
      {/if}
    </CardContent>
  </Card>
</div>
