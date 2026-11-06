<script lang="ts">
  import Input from "$lib/components/ui/input/input.svelte";
  import Account from "./Account.svelte";
</script>

<div class="mt-2">
  <div class="flex items-center justify-between gap-4">
    <div class="flex-1">Logo</div>
    <div class="flex-2">
      <Input class="rounded-xl" placeholder="Search" />
    </div>
    <div class="flex-1 flex justify-end">
      <Account />
    </div>
  </div>
</div>
