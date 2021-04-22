<script>
  import * as helpers from "../helpers";
  import Copyable from "./Copyable.svelte";
  import Icon from "./Icon";
  export let seed = null;
  export let projects = null;
  export let online = null;

  $: seedId = seed.publicAddr
    ? `${seed.peerId}@${seed.publicAddr}`
    : seed.peerId;

  $: truncatedSeedId = seed.publicAddr
    ? `${helpers.truncate(seed.peerId)}@${seed.publicAddr}`
    : helpers.truncate(seed.peerId);
</script>

<style>
  header {
    border-bottom: 1px solid var(--color-foreground-level-2);
    cursor: default;
    background-color: var(--color-foreground-level-1);
  }

  container {
    display: flex;
    max-width: 90rem;
    margin: 0 auto;
    padding: 4rem;
    align-items: center;
    justify-content: space-between;
  }

  .meta {
    color: var(--color-foreground-level-6);
    max-width: 75%;
    align-self: flex-start;
  }

  .meta h1 {
    color: var(--color-foreground);
    padding-bottom: 0.5rem;
  }

  .desc {
    margin-top: 1.5rem;
  }

  .stat h2 {
    padding-bottom: 0.5rem;
  }

  .stat h5 {
    color: var(--color-foreground-level-6);
    line-height: 120%;
  }

  .seed-id {
    color: var(--color-foreground-level-5);
  }

  a.button {
    padding: 0.5rem 0.75rem;
    border-radius: 0.25rem;
    font-weight: 600;
    display: flex;
    border: 1px solid var(--color-foreground-level-2);
  }

  a.button:hover {
    background: var(--color-foreground-level-2);
  }
</style>

<header>
  <container>
    <!-- INSERT LOGO HERE 
  
    <img src="logo.png">
  
  -->
    <div class="meta">
      <h1>{seed.name}</h1>
      <p class="address">
        <Copyable showIcon styleContent={false} copyContent={seedId}>
          <p class="typo-text-small-mono seed-id">{truncatedSeedId}</p>
        </Copyable>
      </p>
      {#if seed.description}
        <p class="desc">
          {@html seed.description}
        </p>
      {/if}
    </div>
    <a class="button right" href="radicle.xyz">
      <Icon.ArrowBoxUpRight style="margin-right: 0.8rem;" />radicle.xyz
    </a>
  </container>
</header>
