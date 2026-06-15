<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Link } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Connectors" 
    description="Manage your third-party integrations here."
  >
    {#snippet icon()}
      <Link size={24} />
    {/snippet}
  </EmptyState>
</div>
