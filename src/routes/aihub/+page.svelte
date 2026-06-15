<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Cpu } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="AI Hub" 
    description="Explore and manage AI models and capabilities."
  >
    {#snippet icon()}
      <Cpu size={24} />
    {/snippet}
  </EmptyState>
</div>
