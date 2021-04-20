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
  }

  .name {
    display: flex;
    justify-content: space-between;
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

  h3 {
    margin-bottom: 0.5rem;
  }
</style>

<div class="container">
  <h3>
    <a class="name" href={`radicle://link/v0/${project.urn}`}>{project.name}</a>
  </h3>
  <div class="radicle-id">
    <Icon.At style="margin-right: 0.25rem;" />
    <Copyable showIcon={true} styleContent={false} copyContent={project.urn}>
      <p class="typo-text-small-mono urn">
        {helpers.truncate(project.urn.replace('rad:git:', ''))}
      </p>
    </Copyable>
  </div>
  <p class="typo-text desc">{project.description}</p>
  <div class="bottom">
    <div class="stats">
      {#if project.stats}
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
      {/if}
    </div>
  </div>
</div>
