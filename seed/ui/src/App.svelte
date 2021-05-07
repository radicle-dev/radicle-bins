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

  let searchQuery = "";
  let searchResults = [];

  $: allProjects = $projects;
  $: featuredProjects = allProjects.filter(project => project.featured);
  $: activeTab = featuredProjects.length > 0 ? "featured" : "all";

  $: {
    const searchCollection =
      activeTab === "all" ? allProjects : featuredProjects;
    const fuse = new Fuse(searchCollection, options);

    if (searchQuery.length > 0) {
      searchResults = fuse.search(searchQuery).map(result => result.item);
    } else {
      searchResults = searchCollection;
    }
  }

  $: if ($seed) {
    document.title = `${$seed.name}`;
  }

  $: peerCount = $online || $seen ? $online.length + $seen.length : 0;

  poll();
</script>

<style>
  .container {
    margin: 0 auto;
    padding: 0 4rem 2rem;
    max-width: 90rem;
    display: flex;
  }

  main {
    width: 100%;
  }

  aside {
    max-width: 20rem;
    padding-left: 3rem;
    visibility: visible;
    margin-top: -1.9rem;
  }

  header {
    display: flex;
    margin-bottom: 1rem;
    flex-direction: column;
    cursor: default;
  }

  .tabs-container {
    display: flex;
    padding: 1rem 0 0 4rem;
  }

  .tabs {
    display: flex;
    gap: 2rem;
  }

  .tabs button {
    cursor: pointer;
  }

  .tabs button:focus-visible {
    outline: none;
  }

  .tabs button:focus-visible h4 {
    color: var(--color-foreground-level-4);
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

  .mobile-peers {
    visibility: hidden;
  }

  @media screen and (max-width: 63rem) {
    .container {
      flex-direction: column;
      padding: 0 2rem 2rem 2rem;
      overflow: hidden;
    }

    aside {
      visibility: hidden;
      display: none;
    }

    .tabs-container {
      padding-left: 2rem;
      overflow-x: scroll;
      -webkit-overflow-scrolling: touch;
    }

    .tabs {
      display: flex;
      flex-direction: row;
      gap: 1rem;
      width: max-content;
      padding: 0.25rem 1rem 0.5rem 0;
    }

    .tabs button {
      width: max-content;
    }

    .mobile-peers {
      visibility: visible;
    }
  }
</style>

{#if $seed}
  <Header seed={$seed} />
{/if}
<div class="tabs-container">
  <div class="tabs">
    {#if featuredProjects.length > 0}
      <button
        class:active={activeTab === 'featured'}
        on:click={() => (activeTab = 'featured')}>
        <h4>
          Featured projects
          <span class="number">{featuredProjects.length}</span>
        </h4>
      </button>
    {/if}
    <button
      class:active={activeTab === 'all'}
      on:click={() => (activeTab = 'all')}>
      <h4>
        {featuredProjects.length > 0 ? 'All projects' : 'Projects'}
        <span class="number">{allProjects.length}</span>
      </h4>
    </button>
    <button
      class="mobile-peers"
      class:active={activeTab === 'peers'}
      on:click={() => (activeTab = 'peers')}>
      <h4>Peers <span class="number">{peerCount}</span></h4>
    </button>
  </div>
</div>
<div class="container">
  <main>
    <header>
      {#if activeTab !== 'peers'}
        <Input
          style="width: 100%;"
          disabled={allProjects.length === 0}
          bind:value={searchQuery}
          placeholder="Type to filter…" />
      {/if}
    </header>
    {#if activeTab === 'peers'}
      <div class="mobile-peers">
        {#if $online.length > 0 || $seen.length > 0}
          <PeerList online={$online} seen={$seen} />
        {:else}
          <p style="color: var(--color-foreground-level-5);">No peers</p>
        {/if}
      </div>
    {:else if allProjects.length > 0}
      {#each searchResults as result}
        <Project project={result} />
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
      <h4>Peers <span class="number">{peerCount}</span></h4>
    </header>
    {#if $online.length > 0 || $seen.length > 0}
      <PeerList online={$online} seen={$seen} />
    {:else}
      <p style="color: var(--color-foreground-level-5);">No peers</p>
    {/if}
  </aside>
</div>
