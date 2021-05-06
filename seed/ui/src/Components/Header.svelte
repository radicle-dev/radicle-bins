<script>
  import * as helpers from "../helpers";
  import Copyable from "./Copyable.svelte";
  import Icon from "./Icon";

  export let seed = null;

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
  }

  .logo {
    width: 7.5rem;
    min-width: 7.5rem;
    margin-right: 2rem;
    border-radius: 3.75rem;
    align-self: flex-start;
  }

  @media screen and (max-width: 63rem) {
    container {
      flex-direction: column;
      text-align: center;
    }

    .logo {
      align-self: center;
    }
  }

  .meta {
    color: var(--color-foreground-level-6);
    padding: 0 2rem 0 0;
    align-self: flex-start;
    width: 100%;
  }

  .meta h1 {
    color: var(--color-foreground);
    padding-bottom: 0.5rem;
  }

  .desc {
    margin-top: 1.5rem;
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
    {#if seed.logoUrl}
      <img src={seed.logoUrl} alt="the logo" class="logo" />
    {/if}
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
    {#if seed.homepage}
      <a class="button right" href={seed.homepage}>
        <Icon.ArrowBoxUpRight style="margin-right: 0.8rem;" />
        {seed.homepage}
      </a>
    {/if}
  </container>
</header>
