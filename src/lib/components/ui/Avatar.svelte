<script lang="ts">
  let {
    initials = '',
    src = '',
    alt = '',
    size = 28,
    class: className = ''
  }: {
    initials?: string;
    src?: string;
    alt?: string;
    size?: number;
    class?: string;
  } = $props();
</script>

<div 
  class="avatar flex items-center justify-center font-mono font-semibold {className}"
  style="width: {size}px; height: {size}px; font-size: {size * 0.45}px;"
>
  {#if src}
    <img {src} {alt} class="avatar-image" />
  {:else}
    {initials}
  {/if}
</div>

<style>
  .avatar {
    border-radius: var(--radius-full);
    background-color: var(--accent);
    color: var(--text-inverse);
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
    flex-shrink: 0;
  }

  .avatar-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>
