<script lang="ts">
  import { Share2 } from '@lucide/svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">

  <EmptyState
    title="Graph View Empty"
    description="Your knowledge graph will automatically extract entities and conceptual links as you interact with Dexter."
  >
    {#snippet icon()}
      <Share2 size={20} />
    {/snippet}
  </EmptyState>
</div>

<style>
  .page-container {
    flex: 1;
    overflow-y: auto;
  }
</style>
