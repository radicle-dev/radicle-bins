<script lang="javascript">
  import Fuse from "fuse.js";
  import { seed, projects, online, poll, seen } from "./state.js";

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

  poll();
</script>

<style>
  container {
    margin: 0 auto;
    padding: 2rem 4rem;
    max-width: 90rem;
    display: flex;
  }

  main {
    width: 100%;
  }

  aside {
    max-width: 20rem;
    padding-left: 3rem;
  }

  header {
    display: flex;
    margin-bottom: 1rem;
    flex-direction: column;
    cursor: default;
  }

  header h2 {
    margin-right: 1rem;
  }

  .number {
    background: var(--color-foreground-level-2);
    padding: 0.25rem 0.5rem;
    border-radius: 1rem;
    margin-left: 0.25rem;
  }

  h4 {
    color: var(--color-foreground-level-6);
  }
</style>

{#if $seed}
  <Header seed={$seed} projects={$projects} online={$online} />
{/if}
<container>
  <main>
    <header>
      <h4>
        Projects
        <span class="number"><!-- {online ? online.length : 0} -->1297</span>
      </h4>
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
      <h4>
        Peers
        <span class="number"><!-- {projects ? projects.length : 0} -->21</span>
      </h4>
    </header>
    {#if $online.length > 0 || $seen.length > 0}
      <PeerList online={$online} seen={$seen} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">No peers</p>
    {/if}
  </aside>
</container>
