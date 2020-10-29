import { derived, writable } from "svelte/store";

const peerStore = writable([]);
const projectStore = writable([]);

export const projects = derived(projectStore, a => a);
export const seed = derived([peerStore, projectStore], ([peers, projects]) => {
  return {
    name: "seedling.radicle.xyz",
    address: "hybh5c...7ye83k@seedling.radicle.xyz:12345",
    desc: "Seedling",
    peers: peers.length,
    projects: projects.length,
  };
});

export const online = derived(peerStore, peers => {
  return peers
    .filter(peer => peer.state.type === "connected")
    .sort((a, b) => {
      if (a.name < b.name) {
        return -1;
      }
      if (a.name > b.name) {
        return 1;
      }
      return 0;
    });
});
export const seen = derived(peerStore, peers => {
  return peers
    .filter(peer => peer.state.type === "disconnected")
    .map(peer => {
      peer.state.since = new Date(peer.state.since.secs_since_epoch * 1000);
      return peer;
    })
    .sort((a, b) => b.state.since - a.state.since);
});

const eventSource = new EventSource("/events");

eventSource.onmessage = e => {
  const data = JSON.parse(e.data);

  switch (data.type) {
    case "peerConnected": {
      fetch("/peers")
        .then(resp => resp.json())
        .then(peerStore.set);

      break;
    }

    case "peerDisconnected": {
      fetch("/peers")
        .then(resp => resp.json())
        .then(peerStore.set);

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
      break;
    }

    case "snapshot": {
      peerStore.set(data.peers);
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

      break;
    }
  }
};
