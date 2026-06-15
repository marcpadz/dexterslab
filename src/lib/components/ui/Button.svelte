<script lang="ts">
  import type { Snippet } from 'svelte';

  // Properties using Svelte 5 Runes
  let { 
    children, 
    variant = 'primary', 
    size = 'md', 
    onclick, 
    disabled = false, 
    type = 'button',
    class: className = '',
    ...rest 
  }: {
    children?: Snippet;
    variant?: 'primary' | 'secondary' | 'ghost' | 'destructive';
    size?: 'sm' | 'md' | 'lg';
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
    class?: string;
    [key: string]: any;
  } = $props();
</script>

<button
  {type}
  class="btn {variant} {size} focusable {className}"
  {disabled}
  {onclick}
  {...rest}
>
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-sans);
    font-weight: 500;
    border-radius: var(--radius-sm);
    border: 1px solid transparent;
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
    transition: background-color var(--duration-fast) var(--ease-out), 
                border-color var(--duration-fast) var(--ease-out),
                box-shadow var(--duration-fast) var(--ease-out);
  }

  .btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  /* Variants */
  .btn.primary {
    background-color: var(--accent);
    color: var(--text-inverse);
  }

  .btn.primary:hover:not(:disabled) {
    background-color: var(--accent-hover);
  }

  .btn.primary:active:not(:disabled) {
    background-color: var(--accent);
  }

  .btn.secondary {
    background-color: var(--bg-surface);
    border-color: var(--border-default);
    color: var(--text-primary);
  }

  .btn.secondary:hover:not(:disabled) {
    background-color: var(--bg-hover);
  }

  .btn.secondary:active:not(:disabled) {
    background-color: var(--bg-active);
  }

  .btn.ghost {
    background-color: transparent;
    color: var(--text-secondary);
  }

  .btn.ghost:hover:not(:disabled) {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn.ghost:active:not(:disabled) {
    background-color: var(--bg-active);
  }

  .btn.destructive {
    background-color: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.3);
    color: var(--error);
  }

  .btn.destructive:hover:not(:disabled) {
    background-color: var(--error);
    color: var(--text-inverse);
    border-color: transparent;
  }

  /* Sizes */
  .btn.sm {
    padding: var(--space-1) var(--space-2.5);
    font-size: var(--text-xs);
  }

  .btn.md {
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
  }

  .btn.lg {
    padding: var(--space-3) var(--space-6);
    font-size: var(--text-base);
  }
</style>
