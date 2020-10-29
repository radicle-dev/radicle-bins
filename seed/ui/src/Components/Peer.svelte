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
    align-items: center;
  }

  .status-indicator {
    padding: 0rem 0.25rem;
    background-color: var(--color-positive-level-1);
    color: var(--color-positive);
    border-radius: 0.25rem;
    margin-left: 0.5rem;
  }

  .time {
    margin-left: 1rem;
    color: var(--color-foreground-level-3);
    flex: 1;
  }

  .peer-id {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container">
  <div class="status">
    <User user={peer.user} />
    {#if peer.state.type === 'connected'}
      <p class="status-indicator">online</p>
    {:else}
      <p class="time typo-text-small">{peer.state.since.toLocaleString()}</p>
    {/if}
  </div>
  <p class="typo-text-small-mono peer-id typo-overflow-ellipsis">
    {peer.peerId}
  </p>
</div>
