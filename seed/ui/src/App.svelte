<script>
  import { Header, Project, Peer } from "./Components";

  const events = new EventSource("/events");

  import { writable } from "svelte/store";

  $: seed = {
    name: "seedling.radicle.xyz",
    address: "hybh5c...7ye83k@seedling.radicle.xyz:12345",
    desc: "Seedling",
    peers: 0,
    projects: 0,
  };

  const projects = writable([]);
  const peers = writable([]);

  events.onmessage = e => {
    const data = JSON.parse(e.data);

    console.log(data);

    switch (data.type) {
      case "peerConnected": {
        peers.update(ps => {
          ps.push({
            peerId: data.peerId,
            urn: data.urn,
            name: data.name,
            online: true,
            emoji: "",
            color: "#ff00ff",
          });
          return ps;
        });
        seed.peers += 1;

        break;
      }

      case "peerDisconnected": {
        peers.update(ps => {
          ps.forEach(p => {
            if (p.peerId === data.peerId) {
              p.online = false;
            }
          });
          return ps;
        });
        seed.peers -= 1;

        break;
      }

      case "projectTracked": {
        projects.update(ps => {
          ps.push({
            urn: data.urn,
            name: data.name,
            maintainer: data.maintainers[0],
            description: data.description,
          });
          return ps;
        });
        seed.projects += 1;

        break;
      }

      case "snapshot": {
        seed.projects = data.projects.length;

        projects.set(
          data.projects.map(p => {
            return {
              urn: p.urn,
              name: p.name,
              maintainer: p.maintainers[0],
              description: p.description,
            };
          })
        );

        break;
      }
    }
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
  <Header {seed} />
  <main>
    <h3>Projects</h3>
    {#each $projects as project}
      <Project {project} />
    {/each}
  </main>
  <aside>
    <h3>Peers</h3>
    {#each $peers as peer}
      <Peer {peer} />
    {:else}None.{/each}
  </aside>
</div>
