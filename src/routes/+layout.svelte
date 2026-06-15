<script lang="ts">
  import '$lib/styles/tokens.css';
  import '$lib/styles/reset.css';
  import '$lib/styles/utilities.css';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import AppShell from '$lib/components/shell/AppShell.svelte';
  import Toast from '$lib/components/ui/Toast.svelte';
  import { shortcuts } from '$lib/utils/shortcuts';
  import { toast } from '$lib/utils/toast.svelte';

  let { children } = $props();

  onMount(() => {
    // Register global shortcuts
    shortcuts.register('b', () => {
      // Trigger sidebar toggle (we can trigger this by simulating click on sidebar toggle or dispatching event)
      const toggleBtn = document.querySelector('.toggle-btn') as HTMLButtonElement | null;
      if (toggleBtn) toggleBtn.click();
    }, { description: 'Toggle Sidebar' });

    shortcuts.register('k', () => {
      toast.info('Command Palette coming in Phase 2!');
    }, { description: 'Open Command Palette' });

    // Navigation shortcuts
    const navs = ['/', '/knowledge', '/graph', '/skills', '/mcp', '/models', '/settings', '/subscription'];
    navs.forEach((path, index) => {
      shortcuts.register((index + 1).toString(), () => {
        goto(path);
      }, { description: `Navigate to tab ${index + 1}` });
    });

    // Start listening
    shortcuts.start();

    return () => {
      shortcuts.stop();
    };
  });
</script>

<AppShell>
  {@render children()}
</AppShell>

<Toast />
