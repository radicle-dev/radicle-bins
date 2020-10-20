<script>
  import { Header, Project, Peer } from "./Components";
  export let data = null;

  const events = new EventSource("/events");

  events.onmessage = e => {
    console.log(e);
  };
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
  <Header seed={data.seed} />
  <main>
    <h3>Replicated projects</h3>
    {#each data.projects as project}
      <Project {project} />
    {/each}
  </main>
  <aside>
    <h3>Connected peers</h3>
    {#each data.peers as peer}
      <Peer {peer} />
    {/each}
  </aside>
</div>
