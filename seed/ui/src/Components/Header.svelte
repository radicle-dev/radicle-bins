<script>
  export let seed = null;
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
</style>

<div class="meta">
  <h2>{seed.name}</h2>
  <p class="address typo-text-small-mono"><span>{seed.peerId}</span></p>
  {#if seed.description}
    <p class="desc">
      {@html seed.description}
    </p>
  {/if}
</div>
<div class="stat">
  <h2>{seed.peers}</h2>
  <h5>connected<br />peers</h5>
</div>
<div class="stat">
  <h2>{seed.projects}</h2>
  <h5>seeded<br />projects</h5>
</div>
