<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Book } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Library" 
    description="Your saved documents and references will appear here."
  >
    {#snippet icon()}
      <Book size={24} />
    {/snippet}
  </EmptyState>
</div>
