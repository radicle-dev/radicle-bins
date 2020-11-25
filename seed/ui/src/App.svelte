<script>
  import Fuse from "fuse.js";
  import { seed, projects, online, seen } from "./state.js";

  import Header from "./Components/Header.svelte";
  import Input from "./Components/Input.svelte";
  import PeerList from "./Components/PeerList.svelte";
  import Project from "./Components/Project.svelte";

  const options = {
    includeScore: true,
    keys: ["name", "description", "urn"],
    threshold: 0.2,
    ignoreLocation: true,
  };

  let projectFilter = "";
  let allProjects = [];
  let filteredProjects = allProjects;

  $: {
    allProjects = $projects.map(project => {
      return { item: project };
    });

    if (projectFilter.length > 0) {
      const fuse = new Fuse($projects, options);
      filteredProjects = fuse.search(projectFilter);
    } else {
      filteredProjects = allProjects;
    }
  }
</script>

<style>
  main {
    grid-area: main;
  }

  aside {
    grid-area: sidebar;
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

  header {
    display: flex;
    align-items: center;
    margin-bottom: 1.5rem;
    height: 2.5rem;
  }

  header h3 {
    margin-right: 1rem;
  }
</style>

<div class="container">
  {#if $seed}
    <Header seed={$seed} projects={$projects} online={$online} />
  {/if}
  <main>
    <header>
      <h3>Projects</h3>
      <Input
        style="width: 100%;"
        disabled={$projects.length === 0}
        bind:value={projectFilter}
        placeholder="Type to filter…" />
    </header>
    {#if $projects.length > 0}
      {#each filteredProjects as project}
        <Project project={project.item} />
      {:else}
        <p style="color: var(--color-foreground-level-5);">
          None of the replicated projects match this query
        </p>
      {/each}
    {:else}
      <p style="color: var(--color-foreground-level-5);">
        No replicated projects
      </p>
    {/if}
  </main>
  <aside>
    <header>
      <h3>Peers</h3>
    </header>
    {#if $online.length > 0 || $seen.length > 0}
      <PeerList online={$online} seen={$seen} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">No peers</p>
    {/if}
  </aside>
</div>
