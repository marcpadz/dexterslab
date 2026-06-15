<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { CheckSquare } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Tasks" 
    description="Manage your to-dos and action items."
  >
    {#snippet icon()}
      <CheckSquare size={24} />
    {/snippet}
  </EmptyState>
</div>
