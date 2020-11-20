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

  .meta h2 {
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
  <h2>{seed.name}</h2>
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
  <h2>{online ? online.length : 0}</h2>
  <h5>connected<br />peers</h5>
</div>
<div class="stat">
  <h2>{projects ? projects.length : 0}</h2>
  <h5>seeded<br />projects</h5>
</div>
