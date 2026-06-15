<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { FileText } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="All Notes" 
    description="All your notes in one place."
  >
    {#snippet icon()}
      <FileText size={24} />
    {/snippet}
  </EmptyState>
</div>
