<script>
  import * as helpers from "../helpers";

  import Avatar from "./Avatar.svelte";
  import Icon from "./Icon";
  import Copyable from "./Copyable.svelte";

  export let peer = null;
</script>

<style>
  .container {
    margin-bottom: 1.5rem;
    cursor: default;
  }
  .status {
    display: flex;
    margin-bottom: 0.5rem;
    align-items: center;
    width: 100%;
  }

  .status-indicator {
    padding: 0rem 0.25rem;
    background-color: var(--color-positive-level-1);
    color: var(--color-positive);
    border-radius: 0.25rem;
    margin-left: 0.5rem;
  }

  .peer-id {
    color: var(--color-foreground-level-5);
  }

  .user {
    display: flex;
    align-items: center;
    width: inherit;
  }

  .time {
    margin-left: 1rem;
    color: var(--color-foreground-level-4);
    white-space: nowrap;
    margin-top: 2px;
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
          <Icon.Computer style="margin-right: 0.5rem" />
          <p class="typo-text-bold typo-overflow-ellipsis">{peer.peerId}</p>
        {/if}
        {#if peer.state.type === 'connected'}
          <p class="status-indicator">online</p>
        {:else}
          <p class="time typo-text-small">
            {peer.state.since.toLocaleDateString('default', {
              month: 'long',
              day: '2-digit',
              hour: 'numeric',
              minute: 'numeric',
            })}
          </p>
        {/if}
      </div>
    </div>
    <Copyable showIcon={true} styleContent={false} copyContent={peer.peerId}>
      <p class="typo-text-small-mono peer-id typo-overflow-ellipsis">
        {helpers.truncate(peer.peerId)}
      </p>
    </Copyable>
  </div>
</div>
