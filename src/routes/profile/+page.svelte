<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
</script>

<div class="p-8 w-full h-full text-white bg-zinc-950 overflow-y-auto flex flex-col items-center justify-center">
  <Avatar.Root class="w-32 h-32 mb-6 border-4 border-zinc-800">
    <Avatar.Image src="https://avatars.githubusercontent.com/u/1?v=4" alt="Profile" />
    <Avatar.Fallback class="bg-zinc-800 text-4xl">U</Avatar.Fallback>
  </Avatar.Root>
  
  <h1 class="text-4xl font-bold mb-2">User Profile</h1>
  <p class="text-zinc-500 mb-8">Premium Member</p>
  
  <div class="flex gap-4">
    <Button variant="outline" class="border-zinc-700 hover:bg-zinc-800 text-zinc-300">Edit Profile</Button>
    <Button class="bg-yellow-500 text-black hover:bg-yellow-600">Settings</Button>
  </div>
</div>
