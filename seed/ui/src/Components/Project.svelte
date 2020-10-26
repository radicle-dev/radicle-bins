<script>
  import Icon from "./Icon";
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
    margin-bottom: 0.5rem;
  }

  .urn {
    color: var(--color-foreground-level-5);
  }

  .desc {
    margin: 1rem 0 0rem;
  }

  .stats,
  .stat {
    display: flex;
    align-items: center;
    margin-right: 2rem;
    margin-top: 1.5rem;
  }
</style>

<div class="container">
  <p class="name typo-text-bold">
    <!-- TODO
    <User user={project.maintainer} />
    <span>&nbsp;/&nbsp;</span>
    -->
    {project.name}
  </p>
  <p class="typo-text-small-mono urn">{project.urn}</p>
  <p class="typo-text desc">{project.description}</p>
  {#if project.stats}
    <div class="stats">
      <p class="typo-text typo-mono-bold stat">
        <Icon.Commit style="margin-right: 0.8rem;" />{project.stats.commits}
      </p>
      <p class="typo-text typo-mono-bold stat">
        <Icon.Branch style="margin-right: 0.8rem;" />{project.stats.branches}
      </p>
      <p class="typo-text typo-mono-bold stat">
        <Icon.User style="margin-right: 0.8rem;" />{project.stats.contributors}
      </p>
    </div>
  {/if}
</div>
