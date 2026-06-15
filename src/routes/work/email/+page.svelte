<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { Mail } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Email" 
    description="Your connected email inbox."
  >
    {#snippet icon()}
      <Mail size={24} />
    {/snippet}
  </EmptyState>
</div>
