<script>
  export let style = "";
  export let value = "";
  export let placeholder = "";
  export let disabled = false;
  export let spellcheck = false;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  input {
    border: 1px solid var(--color-foreground-level-3);
    padding: 0.5rem;
    border-radius: 0.25rem;
    width: 100%;
    height: 2.5rem;
    line-height: 3rem;
    padding: 0 0.75rem;
    background-color: var(--color-background);
  }

  input[disabled] {
    cursor: not-allowed;
    color: var(--color-foreground-level-4);
    background-color: var(--color-foreground-level-1);
  }

  input[disabled]::placeholder {
    color: var(--color-foreground-level-4);
  }

  input[disabled]:hover {
    background-color: var(--color-foreground-level-1);
  }

  input::placeholder {
    color: var(--color-foreground-level-5);
  }

  input:focus,
  input:hover {
    outline: none;
    border: 1px solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }
</style>

<div {style} class="wrapper">
  <input {placeholder} {disabled} {spellcheck} bind:value />
</div>
