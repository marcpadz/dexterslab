<script lang="ts">
  let {
    width = '100%',
    height = '14px',
    radius = 'var(--radius-sm)',
    class: className = ''
  }: {
    width?: string;
    height?: string;
    radius?: string;
    class?: string;
  } = $props();
</script>

<div 
  class="skeleton {className}" 
  style="width: {width}; height: {height}; border-radius: {radius};"
></div>

<style>
  .skeleton {
    background-color: var(--bg-hover);
    animation: pulse 1.5s infinite ease-in-out;
  }

  @keyframes pulse {
    0% {
      opacity: 0.6;
    }
    50% {
      opacity: 0.3;
    }
    100% {
      opacity: 0.6;
    }
  }
</style>
