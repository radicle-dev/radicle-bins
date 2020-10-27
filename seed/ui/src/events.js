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

  console.log(data);

  switch (data.type) {
    case "peerConnected": {
      peerStore.update(ps => {
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
      seedStore.update(s => {
        s.peers += 1;
        return s;
      });

      break;
    }

    case "peerDisconnected": {
      peerStore.update(ps => {
        ps.forEach(p => {
          if (p.peerId === data.peerId) {
            p.online = false;
          }
        });
        return ps;
      });
      seedStore.update(s => {
        s.peers -= 1;
        return s;
      });

      break;
    }

    case "projectTracked": {
      projectStore.update(ps => {
        ps.push({
          urn: data.urn,
          name: data.name,
          maintainer: data.maintainers[0],
          description: data.description,
        });
        return ps;
      });
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
