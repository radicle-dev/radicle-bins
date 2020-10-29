<script>
  import { seed, projects, online, seen } from "./state.js";

  import Header from "./Components/Header.svelte";
  import PeerList from "./Components/PeerList.svelte";
  import Project from "./Components/Project.svelte";
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
</style>

<div class="container">
  <Header seed={$seed} />
  <main>
    <h3>Projects</h3>
    {#each $projects as project}
      <Project {project} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">
        No replicated projects
      </p>
    {/each}
  </main>
  <aside>
    <h3>Peers</h3>
    {#if $online.length > 0 || $seen.length > 0}
      <PeerList online={$online} seen={$seen} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">No peers</p>
    {/if}
  </aside>
</div>
