<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Briefcase } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="Work"
    description="Manage tasks and workflows."
    actionLabel="Create Task"
    onAction={() => {}}
  >
    {#snippet icon()}
      <Briefcase size={24} />
    {/snippet}
  </EmptyState>
</div>
