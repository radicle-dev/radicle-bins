<script>
  import * as helpers from "../helpers";

  import Icon from "./Icon";
  import Copyable from "./Copyable.svelte";

  export let project = null;
</script>

<style>
  .container {
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    position: relative;
    cursor: default;
  }

  .urn {
    color: var(--color-foreground-level-5);
  }

  .desc {
    margin: 1rem 0 0rem;
  }

  .bottom {
    display: flex;
    margin-top: 1.5rem;
    justify-content: space-between;
  }

  .stats,
  .stat {
    display: flex;
    align-items: center;
    margin-right: 2rem;
  }

  .radicle-id {
    display: flex;
  }

  h2 {
    margin-bottom: 0.75rem;
  }

  a.button {
    visibility: hidden;
    padding: 0.5rem 0.75rem;
    border-radius: 0.25rem;
    position: absolute;
    top: 1rem;
    right: 1rem;
    font-weight: 600;
    display: flex;
  }

  .container:hover a.button {
    visibility: visible;
  }

  a.button:hover {
    background: var(--color-foreground-level-1);
  }
</style>

<div class="container">
  <h2>{project.name}</h2>
  <div class="radicle-id">
    <Icon.At style="margin-right: 0.25rem;" />
    <Copyable showIcon={true} styleContent={false} copyContent={project.urn}>
      <p class="typo-text-small-mono urn">
        {helpers.truncate(project.urn.replace('rad:git:', ''))}
      </p>
    </Copyable>
  </div>
  <p class="typo-text desc">{project.description}</p>
  {#if project.stats}
    <div class="bottom">
      <div class="stats">
        <p class="typo-text typo-mono-bold stat">
          <Icon.Commit style="margin-right: 0.8rem;" />{project.stats.commits}
        </p>
        <p class="typo-text typo-mono-bold stat">
          <Icon.Branch style="margin-right: 0.8rem;" />{project.stats.branches}
        </p>
        <p class="typo-text typo-mono-bold stat">
          <Icon.User
            style="margin-right: 0.8rem;" />{project.stats.contributors}
        </p>
      </div>
    </div>
  {/if}
  <a class="button" href={`radicle://link/v0/${project.urn}`}>
    <Icon.ArrowBoxUpRight style="margin-right: 0.8rem;" />View in Upstream
  </a>
</div>
