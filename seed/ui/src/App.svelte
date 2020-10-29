<script>
  import { Header, Project, Peer } from "./Components";
  import { seed, projects, peers } from "./events.js";
</script>

<style>
  main {
    grid-area: main;
  }

  aside {
    grid-area: sidebar;
  }

  h3 {
    margin-bottom: 1.5rem;
  }

  .container {
    display: grid;
    grid-template-columns: repeat(6, 8.75rem);
    column-gap: 1.5rem;
    row-gap: 2.5rem;
    grid-template-rows: auto;
    grid-template-areas:
      "header header header header header header"
      "main main main main sidebar sidebar";
    margin: 0 auto;
    max-width: 62rem;
    padding: 4rem 1rem;
  }

  .empty-state {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container">
  <Header seed={$seed} />
  <main>
    <h3>Projects</h3>
    {#each $projects as project}
      <Project {project} />
    {:else}
      <p class="empty-state">No replicated projects</p>
    {/each}
  </main>
  <aside>
    <h3>Peers</h3>
    {#each $peers as peer}
      <Peer {peer} />
    {:else}
      <p class="empty-state">No connected peers</p>
    {/each}
  </aside>
</div>
