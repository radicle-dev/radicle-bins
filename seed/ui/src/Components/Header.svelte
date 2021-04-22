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

  .logo {
    min-width: 7.5rem;
    margin-right: 2rem;
  }

  .meta {
    color: var(--color-foreground-level-6);
    padding: 0 2rem 0 0;
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
    <svg
      class="logo"
      width="120"
      height="120"
      viewBox="0 0 120 120"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M0 60C0 26.8629 26.8629 0 60 0V0C93.1371 0 120 26.8629 120 60V60C120 93.1371 93.1371 120 60 120V120C26.8629 120 0 93.1371 0 60V60Z"
        fill="#5555FF" />
      <path
        d="M61.4728 84.8206C60.5665 74.9597 62.0035 61.6698 67.8162 55.231C73.9379 56.5883 86.1812 52.3995 90.6333 43.9043C94.6704 36.201 93.9387 24.8961 96.1984 21.8181C69.5781 21.8179 65.1615 49.3348 57.5386 69.9417C51.9022 54.7051 34.2137 44.438 18.843 50.7006C25.092 55.0203 28.9884 65.6719 37.4554 66.6252C40.1193 66.9252 44.9571 66.6252 48.1435 60.6032C55.6835 66.728 54.5845 74.1554 53.6043 84.8206C53.2087 89.125 51.7195 93.2189 51.9383 96.6338C52.2523 102.665 62.8249 102.665 63.1388 96.6338C63.3577 93.2189 61.8684 89.125 61.4728 84.8206Z"
        fill="url(#paint0_linear)" />
      <defs>
        <linearGradient
          id="paint0_linear"
          x1="57.5207"
          y1="21.8181"
          x2="57.5207"
          y2="101.157"
          gradientUnits="userSpaceOnUse">
          <stop stop-color="#00FF19" />
          <stop offset="1" stop-color="#00B81E" />
        </linearGradient>
      </defs>
    </svg>

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
