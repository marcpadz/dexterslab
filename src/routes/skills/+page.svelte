<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Wrench } from 'lucide-svelte';
</script>

<div class="page-container flex flex-col gap-6 p-6">
  <EmptyState
    title="Skills"
    description="Manage custom skills for your agents."
    actionLabel="Add Skill"
    onAction={() => {}}
  >
    {#snippet icon()}
      <Wrench size={24} />
    {/snippet}
  </EmptyState>
</div>
