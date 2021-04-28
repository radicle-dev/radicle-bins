<script>
  import * as helpers from "../helpers";

  import Icon from "./Icon";
  import Copyable from "./Copyable.svelte";
  import Avatar from "./Avatar.svelte";
  import Badge from "./Badge.svelte";

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

  .maintainer {
    display: flex;
    margin-top: 1.5rem;
    align-items: center;
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
  <div class="maintainer">
    {#each project.maintainers as maintainer}
      <Avatar avatar={maintainer.avatar} />
      {#if maintainer.name}
        <p class="typo-text-bold">{maintainer.name}</p>
        <Badge caption="maintainer" style="margin-left: .5rem;" />
      {/if}
    {/each}
  </div>
  <a class="button" href={`radicle://link/v0/${project.urn}`}>
    <Icon.ArrowBoxUpRight style="margin-right: 0.8rem;" />View in Upstream
  </a>
</div>
