<script>
  import User from "./User.svelte";
  export let peer = null;
</script>

<style>
  .container {
    margin-bottom: 1.5rem;
  }
  .status {
    display: flex;
    margin-bottom: 0.5rem;
  }

  .status-indicator {
    padding: 0rem 0.25rem;
    background-color: var(--color-positive-level-1);
    color: var(--color-positive);
    border-radius: 0.25rem;
    margin-left: 0.5rem;
  }

  .urn {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container">
  <div class="status">
    <User user={peer} />
    {#if peer.online}
      <p class="status-indicator">online</p>
    {/if}
  </div>
  <p class="typo-text-small-mono urn typo-overflow-ellipsis">{peer.peerId}</p>
</div>
