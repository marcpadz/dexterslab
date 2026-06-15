<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Terminal } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="Playground"
    description="Experiment with different prompts and models."
    actionLabel="New Experiment"
    onAction={() => {}}
  >
    {#snippet icon()}
      <Terminal size={24} />
    {/snippet}
  </EmptyState>
</div>
