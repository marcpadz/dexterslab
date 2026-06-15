<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { ShoppingBag } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="Marketplace"
    description="Discover new skills and integrations."
    actionLabel="Explore"
    onAction={() => {}}
  >
    {#snippet icon()}
      <ShoppingBag size={24} />
    {/snippet}
  </EmptyState>
</div>
