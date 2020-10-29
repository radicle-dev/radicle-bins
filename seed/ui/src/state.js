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
                maintainers: p.maintainers,
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
            maintainers: p.maintainers,
            description: p.description,
          };
        })
      );

      break;
    }
  }
};

const data = {
  seed: {
    name: "seedling.radicle.xyz",
    address: "hybh5c...7ye83k@seedling.radicle.xyz:12345",
    desc:
      "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempora quod inventore cumque doloribus sapiente maiores! Sed cumque iste, nisi amet, officiis illo eum veniam ducimus rem minus voluptates, quaerat vero.",
    peers: 12,
    projects: 1000,
  },
  projects: [
    {
      maintainers: [
        {
          avatar: {
            background: {
              r: 255,
              g: 85,
              b: 255,
            },
            emoji: "😎",
          },
          name: "julien",
          urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
        },
      ],
      name: "upstream",
      description: "an amazing p2p app",
      urn:
        "rad:git:hwd1yre8ugp55p37domyqoi9m44ptwqrixioxgdic9kaduo8es5xhzhbaty",
      stats: {
        commits: 12,
        branches: 32,
        contributors: 1,
      },
    },
    {
      maintainers: [
        {
          avatar: {
            background: {
              r: 255,
              g: 255,
              b: 85,
            },
            emoji: "🏀",
          },
          name: "billy",
          urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziud543qkfdxkge6ecj6zzissd",
        },
      ],
      name: "kevin",
      description:
        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Quas deleniti repudiandae sint adipisci eveniet, inventore ad atque cum. Incidunt, excepturi! Doloribus vitae laboriosam enim, ipsa ullam est vel voluptatibus quidem.",
      urn:
        "rad:git:hwd1yre8ugp55p37domyqoi9m44ptwqrixioxgdic9kaduo8es5xhzhbaty",
    },
  ],
  peers: [
    {
      user: {
        avatar: {
          background: {
            r: 255,
            g: 85,
            b: 255,
          },
          emoji: "😎",
        },
        name: "julien",
        urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam1",
      },
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam1",
      state: { type: "connected" },
    },
    {
      user: {
        avatar: {
          background: {
            r: 13,
            g: 188,
            b: 121,
          },
          emoji: "⚽️",
        },
        name: "tino",
        urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam2",
      },
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam2",
      state: { type: "connected" },
    },
    {
      user: {
        avatar: {
          background: {
            r: 188,
            g: 63,
            b: 188,
          },
          emoji: "⛺️",
        },
        name: "milly",
        urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam3",
      },
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam3",
      state: { type: "connected" },
    },
    {
      user: {
        avatar: {
          background: {
            r: 81,
            g: 154,
            b: 186,
          },
          emoji: "🚨",
        },
        name: "wonka",
        urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam4",
      },
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam4",
      state: {
        type: "disconnected",
        since: { secs_since_epoch: Date.now() / 1000 },
      },
    },
    {
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam5",
      state: {
        type: "disconnected",
        since: { secs_since_epoch: Date.now() / 1000 },
      },
    },
    {
      user: {
        avatar: {
          background: {
            r: 0,
            g: 62,
            b: 170,
          },
          emoji: "🦙",
          urn: "rad:git:hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
        },
        name: "lima",
      },
      peerId: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      state: {
        type: "disconnected",
        since: { secs_since_epoch: Date.now() / 1000 },
      },
    },
  ],
};

if (process.env === "dev") {
  peerStore.set(data.peers);
  projectStore.set(data.projects);
}
