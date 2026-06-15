<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    children,
    class: className = ''
  }: {
    children: Snippet;
    class?: string;
  } = $props();

  let scrollContainer: HTMLDivElement | null = $state(null);

  export function scrollToBottom() {
    if (scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  }

  export function scrollToTop() {
    if (scrollContainer) {
      scrollContainer.scrollTop = 0;
    }
  }
</script>

<div 
  class="scroll-container scrollable {className}" 
  bind:this={scrollContainer}
>
  {@render children()}
</div>

<style>
  .scroll-container {
    width: 100%;
    height: 100%;
    position: relative;
  }
</style>
