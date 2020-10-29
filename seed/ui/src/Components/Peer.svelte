<script>
  import Avatar from "./Avatar.svelte";
  import Emoji from "./Emoji.svelte";

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

  .user {
    display: flex;
  }
</style>

<div class="container">
  <div>
    <div class="status">
      <div class="user">
        {#if peer.user}
          <Avatar avatar={peer.user.avatar} />
          <p class="typo-text-bold">{peer.user.name}</p>
        {:else}
          <Emoji emoji="🖥" />
          <p class="typo-text-bold typo-overflow-ellipsis">{peer.peerId}</p>
        {/if}
      </div>
      {#if peer.state.type === 'connected'}
        <p class="status-indicator">online</p>
      {:else}
        <p class="time typo-text-small">{peer.state.since.toLocaleString()}</p>
      {/if}
    </div>
    {#if peer.user}
      <p class="typo-text-small-mono peer-id typo-overflow-ellipsis">
        {peer.peerId}
      </p>
    {/if}
  </div>
</div>
