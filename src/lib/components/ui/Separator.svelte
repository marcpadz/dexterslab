<script lang="ts">
  let {
    orientation = 'horizontal',
    class: className = ''
  }: {
    orientation?: 'horizontal' | 'vertical';
    class?: string;
  } = $props();
</script>

<div 
  class="separator {orientation} {className}" 
  role="separator" 
  aria-orientation={orientation}
></div>

<style>
  .separator {
    background-color: var(--border-subtle);
  }

  .separator.horizontal {
    width: 100%;
    height: 1px;
  }

  .separator.vertical {
    width: 1px;
    height: 100%;
  }
</style>
