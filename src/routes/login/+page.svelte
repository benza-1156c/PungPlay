<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { auth, initAuth, switchProfile, login } from "../../stores/auth.svelte";
  import LoginButton from "../../components/auth/LoginButton.svelte";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
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
    Music,
    UserPlus,
    ChevronRight,
  } from "@lucide/svelte";

  onMount(() => {
    const cleanupPromise = initAuth();
    return () => {
      cleanupPromise.then((cleanup) => cleanup?.());
    };
  });

  function getInitials(name?: string | null): string {
    if (!name) return "U";
    const parts = name.trim().split(/\s+/);
    if (parts.length >= 2) {
      return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return name.trim().slice(0, 2).toUpperCase();
  }

  async function handleSelectProfile(profileId: string) {
    await switchProfile(profileId);
    goto("/");
  }
</script>

<div
  class="min-h-screen bg-background text-foreground flex items-center justify-center p-6 selection:bg-red-500/30"
>
  <Card class="w-full max-w-md shadow-2xl border-border/60">
    <CardHeader class="flex flex-col items-center text-center gap-3">
      <div
        class="w-14 h-14 rounded-2xl bg-red-600 flex items-center justify-center text-white shadow-lg shadow-red-950/40"
      >
        <Music class="w-7 h-7" />
      </div>
      <CardTitle class="text-2xl">
        {#if auth.profiles.length > 0}
          เลือกบัญชีผู้ใช้
        {:else}
          เข้าสู่ระบบ <span class="text-yellow-500">PungPlay</span>
        {/if}
      </CardTitle>
      <CardDescription>
        {#if auth.profiles.length > 0}
          เลือกโปรไฟล์ YouTube Music ที่ต้องการใช้งาน หรือเพิ่มบัญชีใหม่
        {:else}
          เข้าสู่ระบบด้วยบัญชี YouTube Music เพื่อฟังเพลงและจัดการคลังเพลงของคุณ
        {/if}
      </CardDescription>
    </CardHeader>

    <CardContent class="flex flex-col gap-4">
      <!-- Error Message -->
      {#if auth.status === "error"}
        <div
          class="flex items-center gap-2 p-3 rounded-xl border border-destructive/50 bg-destructive/10 text-destructive text-sm"
        >
          <CircleAlert class="w-4 h-4 shrink-0" />
          <span>{auth.err || "เกิดข้อผิดพลาดในการล็อกอิน"}</span>
        </div>
      {/if}

      <!-- Logging in loading state -->
      {#if auth.status === "logging_in"}
        <div
          class="flex items-center justify-center gap-3 p-4 rounded-xl border bg-muted text-muted-foreground text-sm"
        >
          <LoaderCircle class="w-4 h-4 text-red-500 animate-spin" />
          <span>กำลังเปิดหน้าต่างล็อกอิน...</span>
        </div>
      {/if}

      <!-- Profiles List (User selects manually) -->
      {#if auth.profiles.length > 0}
        <div class="flex flex-col gap-2">
          {#each auth.profiles as profile (profile.id)}
            <button
              type="button"
              onclick={() => handleSelectProfile(profile.id)}
              class="w-full flex items-center justify-between p-3 rounded-2xl border border-border/60 hover:border-primary/50 hover:bg-muted/40 transition-all text-left group cursor-pointer"
            >
              <div class="flex items-center gap-3 min-w-0">
                <Avatar.Root class="size-11 shrink-0">
                  {#if profile.avatar_url}
                    <Avatar.Image
                      src={profile.avatar_url}
                      alt={profile.name}
                      referrerpolicy="no-referrer"
                      class="object-cover"
                    />
                  {/if}
                  <Avatar.Fallback
                    class="bg-primary/20 text-primary font-semibold text-sm"
                  >
                    {getInitials(profile.name)}
                  </Avatar.Fallback>
                </Avatar.Root>
                <div class="flex flex-col min-w-0">
                  <div class="flex items-center gap-1.5">
                    <span
                      class="text-sm font-semibold truncate text-foreground group-hover:text-primary transition-colors"
                    >
                      {profile.name}
                    </span>
                    {#if profile.is_active}
                      <span
                        class="text-[10px] bg-emerald-500/10 text-emerald-500 border border-emerald-500/30 px-1.5 py-0.5 rounded-full font-medium"
                      >
                        ใช้งานอยู่
                      </span>
                    {/if}
                  </div>
                  {#if profile.email}
                    <span class="text-xs text-muted-foreground truncate">
                      {profile.email}
                    </span>
                  {/if}
                </div>
              </div>
              <ChevronRight
                class="size-4 text-muted-foreground group-hover:text-foreground transition-colors shrink-0 ml-2"
              />
            </button>
          {/each}
        </div>

        <div class="pt-2 flex flex-col gap-2">
          <Button
            variant="outline"
            onclick={login}
            class="w-full rounded-xl h-11 gap-2 cursor-pointer hover:bg-muted"
          >
            <UserPlus class="w-4 h-4" />
            <span>เข้าสู่ระบบบัญชีอื่น</span>
          </Button>
        </div>
      {:else if auth.status !== "logging_in"}
        <!-- No profiles: Initial Login Button -->
        <div class="flex justify-center pt-2">
          <LoginButton />
        </div>
      {/if}
    </CardContent>
  </Card>
</div>
