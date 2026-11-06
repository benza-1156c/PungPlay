<script lang="ts">
  import { auth, login, logout, switchProfile } from "../../stores/auth.svelte";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import LogOutIcon from "@lucide/svelte/icons/log-out";
  import UserPlusIcon from "@lucide/svelte/icons/user-plus";
  import CheckIcon from "@lucide/svelte/icons/check";
  import LogInIcon from "@lucide/svelte/icons/log-in";

  function getInitials(name?: string | null): string {
    if (!name) return "U";
    const parts = name.trim().split(/\s+/);
    if (parts.length >= 2) {
      return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return name.trim().slice(0, 2).toUpperCase();
  }
</script>

{#if auth.activeProfile}
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="ghost"
          size="icon"
          class="rounded-full overflow-hidden p-0 size-9 hover:ring-2 hover:ring-primary/40 transition-all cursor-pointer"
        >
          <Avatar.Root class="size-full">
            {#if auth.activeProfile?.avatar_url}
              <Avatar.Image
                src={auth.activeProfile.avatar_url}
                alt={auth.activeProfile.name}
                class="object-cover"
              />
            {/if}
            <Avatar.Fallback
              class="bg-primary/20 text-primary font-semibold text-xs"
            >
              {getInitials(auth.activeProfile?.name)}
            </Avatar.Fallback>
          </Avatar.Root>
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>

    <DropdownMenu.Content align="end" class="w-64 p-2">
      <!-- Active Profile Header -->
      <div class="flex items-center gap-3 p-2">
        <Avatar.Root class="size-10 shrink-0">
          {#if auth.activeProfile.avatar_url}
            <Avatar.Image
              src={auth.activeProfile.avatar_url}
              alt={auth.activeProfile.name}
              class="object-cover"
            />
          {/if}
          <Avatar.Fallback
            class="bg-primary/20 text-primary font-semibold text-sm"
          >
            {getInitials(auth.activeProfile.name)}
          </Avatar.Fallback>
        </Avatar.Root>
        <div class="flex flex-col min-w-0">
          <span
            class="text-sm font-semibold truncate leading-tight text-foreground"
          >
            {auth.activeProfile.name}
          </span>
          {#if auth.activeProfile.email}
            <span
              class="text-xs text-muted-foreground truncate leading-tight mt-0.5"
            >
              {auth.activeProfile.email}
            </span>
          {/if}
        </div>
      </div>

      <DropdownMenu.Separator />

      <!-- Multi-Account List (if more than 1 account exists) -->
      {#if auth.profiles.length > 1}
        <DropdownMenu.Label>Switch Account</DropdownMenu.Label>
        <DropdownMenu.Group>
          {#each auth.profiles as profile (profile.id)}
            <DropdownMenu.Item
              onclick={() => switchProfile(profile.id)}
              class="flex items-center justify-between gap-2 cursor-pointer"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <Avatar.Root class="size-6 shrink-0">
                  {#if profile.avatar_url}
                    <Avatar.Image src={profile.avatar_url} alt={profile.name} />
                  {/if}
                  <Avatar.Fallback class="text-[10px]">
                    {getInitials(profile.name)}
                  </Avatar.Fallback>
                </Avatar.Root>
                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-medium truncate"
                    >{profile.name}</span
                  >
                  {#if profile.email}
                    <span class="text-[10px] text-muted-foreground truncate"
                      >{profile.email}</span
                    >
                  {/if}
                </div>
              </div>
              {#if profile.is_active}
                <CheckIcon class="size-4 text-emerald-500 shrink-0 ml-1" />
              {/if}
            </DropdownMenu.Item>
          {/each}
        </DropdownMenu.Group>
        <DropdownMenu.Separator />
      {/if}

      <!-- Add Account -->
      <DropdownMenu.Group>
        <DropdownMenu.Item onclick={login} class="cursor-pointer gap-2">
          <UserPlusIcon class="size-4" />
          <span>Add Account</span>
        </DropdownMenu.Item>
      </DropdownMenu.Group>

      <DropdownMenu.Separator />

      <!-- Sign Out -->
      <DropdownMenu.Item
        onclick={logout}
        variant="destructive"
        class="cursor-pointer gap-2 text-red-500 focus:text-red-500 focus:bg-red-500/10"
      >
        <LogOutIcon class="size-4" />
        <span>Sign Out</span>
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{:else}
  <Button
    onclick={login}
    class="bg-red-600 hover:bg-red-500 text-white rounded-full px-4 h-9 gap-2 text-sm font-medium shadow-md transition-all cursor-pointer"
  >
    <LogInIcon class="size-4" />
    <span>Sign In</span>
  </Button>
{/if}
