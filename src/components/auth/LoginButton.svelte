<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { login } from "../../stores/auth.svelte";
  import { LogIn } from "@lucide/svelte";
</script>

<Button onclick={login} class="bg-red-600 hover:bg-red-500 text-white gap-2">
  <LogIn class="w-4 h-4" />
  <span>เข้าสู่ระบบ YouTube Music</span>
</Button>
