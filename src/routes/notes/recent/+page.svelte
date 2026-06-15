<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Clock } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Recent Notes" 
    description="Your most recently edited notes."
  >
    {#snippet icon()}
      <Clock size={24} />
    {/snippet}
  </EmptyState>
</div>
