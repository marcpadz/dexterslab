<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { AppWindow } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="Apps"
    description="Connect and manage external apps here."
    actionLabel="Browse Apps"
    onAction={() => {}}
  >
    {#snippet icon()}
      <AppWindow size={24} />
    {/snippet}
  </EmptyState>
</div>
