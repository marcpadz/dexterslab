<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { MessageSquare } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="All Chats" 
    description="A comprehensive history of all your conversations."
  >
    {#snippet icon()}
      <MessageSquare size={24} />
    {/snippet}
  </EmptyState>
</div>
