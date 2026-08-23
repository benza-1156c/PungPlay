<script lang="ts">
  import { Search } from "@lucide/svelte";
</script>

<div
  class="flex flex-col items-center justify-center py-24 text-neutral-500 space-y-4 border border-dashed border-neutral-800/60 rounded-2xl max-w-3xl mx-auto"
>
  <div class="p-6 rounded-full bg-neutral-900/40">
    <Search class="w-12 h-12 opacity-30" />
  </div>
  <p class="text-lg font-medium text-neutral-400">ลองค้นหาอะไรสักอย่างดูสิ</p>
  <p class="text-sm text-neutral-600">พิมพ์ชื่อเพลง, ศิลปิน, หรือเพลย์ลิสต์ที่ต้องการ</p>
</div>
