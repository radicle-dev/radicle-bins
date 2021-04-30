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
  let featuredProjects = [];
  let filteredProjects = allProjects;
  let activeTab = "all";

  $: if ($seed) {
    document.title = `${$seed.name} - ${$seed.publicAddr || $seed.peerId}`;
  }

  $: {
    allProjects = $projects.map(project => {
      return { item: project };
    });

    featuredProjects = $projects
      .filter(project => project.featured)
      .map(project => {
        return { item: project };
      });

    if (activeTab === "feat") {
      if (projectFilter.length > 0) {
        const fuse = new Fuse(
          $projects.filter(project => project.featured),
          options
        );
        filteredProjects = fuse.search(projectFilter);
      } else {
        filteredProjects = featuredProjects;
      }
    } else {
      if (projectFilter.length > 0) {
        const fuse = new Fuse($projects, options);
        filteredProjects = fuse.search(projectFilter);
      } else {
        filteredProjects = allProjects;
      }
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
    min-width: 35rem;
  }

  aside {
    max-width: 20rem;
    min-width: 20rem;
    padding-left: 3rem;
  }

  header {
    display: flex;
    margin-bottom: 1rem;
    flex-direction: column;
    cursor: default;
  }

  .tabs {
    display: flex;
    gap: 2rem;
  }

  .tabs button {
    cursor: pointer;
  }

  h4 {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }

  .active h4 {
    color: var(--color-primary);
  }

  .number {
    background: var(--color-foreground-level-2);
    padding: 0.25rem 0.5rem;
    border-radius: 1rem;
    margin-left: 0.25rem;
  }
</style>

{#if $seed}
  <Header seed={$seed} />
{/if}
<container>
  <main>
    <header>
      <div class="tabs">
        {#if featuredProjects.length > 0}
          <button
            class:active={activeTab === 'feat'}
            on:click={() => (activeTab = 'feat')}>
            <h4>
              Featured projects
              <span
                class="number">{featuredProjects ? featuredProjects.length : 0}</span>
            </h4>
          </button>
        {/if}
        <button
          class:active={featuredProjects.length > 0 && activeTab === 'all'}
          on:click={() => (activeTab = 'all')}>
          <h4>
            {featuredProjects.length > 0 ? 'All projects' : 'Projects'}
            <span class="number">{allProjects ? allProjects.length : 0}</span>
          </h4>
        </button>
      </div>
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
        <span
          class="number">{$online || $seen ? $online.length + $seen.length : 0}</span>
      </h4>
    </header>
    {#if $online.length > 0 || $seen.length > 0}
      <PeerList online={$online} seen={$seen} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">No peers</p>
    {/if}
  </aside>
</container>
