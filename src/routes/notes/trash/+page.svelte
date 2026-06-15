<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Trash2 } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Trash" 
    description="Deleted notes will be kept here for 30 days."
  >
    {#snippet icon()}
      <Trash2 size={24} />
    {/snippet}
  </EmptyState>
</div>
