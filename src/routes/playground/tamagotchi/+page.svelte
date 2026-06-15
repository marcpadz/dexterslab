<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Smile } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Tamagotchi" 
    description="Your digital AI companion."
  >
    {#snippet icon()}
      <Smile size={24} />
    {/snippet}
  </EmptyState>
</div>
