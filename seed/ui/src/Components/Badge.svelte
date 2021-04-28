<script>
  export let style = "";
  export let caption = "";
</script>

<style>
  .badge {
    color: var(--color-primary);
    background: var(--color-primary-level-1);
    border-radius: 4px;
    padding: 0.125rem 0.25rem 0.25rem;
    margin-top: 0.125rem;
    white-space: nowrap;
  }
</style>

<span {style} class="badge typo-text-small">{caption}</span>
