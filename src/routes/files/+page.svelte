<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { FileText } from 'lucide-svelte';
</script>

<div class="h-full flex items-center justify-center p-8">
  <EmptyState 
    title="Files" 
    description="Your uploaded files and attachments will appear here."
  >
    {#snippet icon()}
      <FileText size={24} />
    {/snippet}
  </EmptyState>
</div>
