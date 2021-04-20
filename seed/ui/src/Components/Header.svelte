<script>
  import Copyable from "./Copyable.svelte";
  export let seed = null;
  export let projects = null;
  export let online = null;

  $: seedId = seed.publicAddr
    ? `${seed.peerId}@${seed.publicAddr}`
    : seed.peerId;
</script>

<style>
  .meta {
    grid-column-start: 1;
    grid-column-end: 5;
    color: var(--color-foreground-level-6);
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
    font-size: 70%;
  }
</style>

<div class="meta">
  <h1>{seed.name}</h1>
  <p class="address">
    <Copyable showIcon styleContent={false} copyContent={seedId}>
      <p class="typo-text-small-mono seed-id">{seedId}</p>
    </Copyable>
  </p>
  {#if seed.description}
    <p class="desc">
      {@html seed.description}
    </p>
  {/if}
</div>
<div class="stat">
  <h1>{online ? online.length : 0}</h1>
  <h5>peers</h5>
</div>
<div class="stat">
  <h1>{projects ? projects.length : 0}</h1>
  <h5>projects</h5>
</div>
