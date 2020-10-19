import App from "./App.svelte";

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
      maintainer: {
        color: "#ff55ff",
        emoji: "😎",
        handle: "julien",
        peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      },
      name: "upstream",
      desc: "an amazing p2p app",
      urn:
        "rad:git:hwd1yre8ugp55p37domyqoi9m44ptwqrixioxgdic9kaduo8es5xhzhbaty",
      stats: {
        commits: 12,
        branches: 32,
        contributors: 1,
      },
    },
    {
      maintainer: {
        color: "#ffff55",
        emoji: "🏀",
        handle: "billy",
        peerID: "hyb8kud543qkfdxkge6ecj6zziud543qkfdxkge6ecj6zzissd",
      },
      name: "kevin",
      desc:
        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Quas deleniti repudiandae sint adipisci eveniet, inventore ad atque cum. Incidunt, excepturi! Doloribus vitae laboriosam enim, ipsa ullam est vel voluptatibus quidem.",
      urn:
        "rad:git:hwd1yre8ugp55p37domyqoi9m44ptwqrixioxgdic9kaduo8es5xhzhbaty",
      stats: {
        commits: 123312,
        branches: 31232,
        contributors: 1111,
      },
    },
  ],
  peers: [
    {
      color: "#ff55ff",
      emoji: "😎",
      handle: "julien",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: true,
    },
    {
      color: "#0DBC79",
      emoji: "⚽️",
      handle: "tino",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: true,
    },
    {
      color: "#BC3FBC",
      emoji: "⛺️",
      handle: "milly",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: false,
    },
    {
      color: "#519ABA",
      emoji: "🚨",
      handle: "wonka",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: false,
    },
    {
      color: "#FFFBD6",
      emoji: "👁",
      handle: "zehlen",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: false,
    },
    {
      color: "#003EAA",
      emoji: "🦙",
      handle: "lima",
      peerID: "hyb8kud543qkfdxkge6ecj6zziudsfsdsam6zziudsfsdsam6",
      online: false,
    },
  ],
};

const app = new App({
  target: document.body,
  props: {
    data: data,
  },
});

export default app;
