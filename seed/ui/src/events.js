import { derived, writable } from "svelte/store";

const projectStore = writable([]);
const peerStore = writable([]);
const seedStore = writable({
  name: "seedling.radicle.xyz",
  address: "hybh5c...7ye83k@seedling.radicle.xyz:12345",
  desc: "Seedling",
  peers: 0,
  projects: 0,
});

export const projects = derived(projectStore, a => a);
export const peers = derived(peerStore, a => a);
export const seed = derived(seedStore, a => a);

const eventSource = new EventSource("/events");

eventSource.onmessage = e => {
  const data = JSON.parse(e.data);

  switch (data.type) {
    case "peerConnected": {
      fetch("/peers")
        .then(resp => resp.json())
        .then(result => peerStore.set(result));

      seedStore.update(s => {
        s.peers += 1;
        return s;
      });

      break;
    }

    case "peerDisconnected": {
      fetch("/peers")
        .then(resp => resp.json())
        .then(result => peerStore.set(result));

      seedStore.update(s => {
        s.peers -= 1;
        return s;
      });

      break;
    }

    case "projectTracked": {
      fetch("/projects")
        .then(resp => resp.json())
        .then(result =>
          projectStore.set(
            result.map(p => {
              return {
                urn: p.urn,
                name: p.name,
                maintainer: p.maintainers[0],
                description: p.description,
              };
            })
          )
        );

      seedStore.update(s => {
        seed.projects += 1;
        return s;
      });

      break;
    }

    case "snapshot": {
      seedStore.update(s => {
        s.projects = data.projects.length;
        s.peers = data.peers.length;
        return s;
      });

      projectStore.set(
        data.projects.map(p => {
          return {
            urn: p.urn,
            name: p.name,
            maintainer: p.maintainers[0],
            description: p.description,
          };
        })
      );
      peerStore.set(data.peers);

      break;
    }
  }
};
