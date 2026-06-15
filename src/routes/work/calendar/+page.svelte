<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Calendar } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Calendar" 
    description="Your upcoming events and schedule."
  >
    {#snippet icon()}
      <Calendar size={24} />
    {/snippet}
  </EmptyState>
</div>
