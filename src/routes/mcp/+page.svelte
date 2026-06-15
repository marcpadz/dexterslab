<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Plug } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="MCP"
    description="Model Context Protocol connections."
    actionLabel="Add Connection"
    onAction={() => {}}
  >
    {#snippet icon()}
      <Plug size={24} />
    {/snippet}
  </EmptyState>
</div>
