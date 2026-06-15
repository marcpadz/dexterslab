<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Folder } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Projects" 
    description="Manage your project workspaces and context here."
  >
    {#snippet icon()}
      <Folder size={24} />
    {/snippet}
  </EmptyState>
</div>
