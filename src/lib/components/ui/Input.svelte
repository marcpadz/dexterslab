<script lang="ts">
  let {
    value = $bindable(''),
    label = '',
    placeholder = '',
    type = 'text',
    error = '',
    helperText = '',
    disabled = false,
    id = Math.random().toString(36).substring(2, 9),
    class: className = '',
    ...rest
  }: {
    value?: string;
    label?: string;
    placeholder?: string;
    type?: string;
    error?: string;
    helperText?: string;
    disabled?: boolean;
    id?: string;
    class?: string;
    [key: string]: any;
  } = $props();
</script>

<div class="input-container flex flex-col gap-1.5 {className}">
  {#if label}
    <label for={id} class="input-label font-medium text-xs text-secondary">{label}</label>
  {/if}

  <div class="input-wrapper" class:disabled class:has-error={!!error}>
    <input
      {id}
      {type}
      {placeholder}
      {disabled}
      bind:value
      class="focusable"
      {...rest}
    />
  </div>

  {#if error}
    <span class="error-text text-xs font-medium">{error}</span>
  {:else if helperText}
    <span class="helper-text text-xs text-muted">{helperText}</span>
  {/if}
</div>

<style>
  .input-container {
    width: 100%;
  }

  .input-wrapper {
    position: relative;
    width: 100%;
    background-color: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-sm);
    transition: border-color var(--duration-fast) var(--ease-out),
                box-shadow var(--duration-fast) var(--ease-out);
  }

  .input-wrapper:hover:not(.disabled) {
    border-color: var(--text-muted);
  }

  .input-wrapper:focus-within:not(.disabled) {
    border-color: var(--accent);
  }

  .input-wrapper.has-error {
    border-color: var(--error);
  }

  .input-wrapper.disabled {
    opacity: 0.5;
  }

  input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-sm);
    color: var(--text-primary);
  }

  input::placeholder {
    color: var(--text-muted);
  }

  .error-text {
    color: var(--error);
  }
</style>
