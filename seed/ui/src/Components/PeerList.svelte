<script>
  import { quintOut } from "svelte/easing";
  import { crossfade } from "svelte/transition";

  import Peer from "./Peer.svelte";

  export let online = [];
  export let seen = [];

  const [send, receive] = crossfade({
    duration: d => Math.sqrt(d * 360),

    fallback(node, _params) {
      const style = getComputedStyle(node);
      const transform = style.transform === "none" ? "" : style.transform;

      return {
        duration: 1200,
        easing: quintOut,
        css: t => `
          transform: ${transform} scale(${t});
          opacity: ${t}
				`,
      };
    },
  });
</script>

<div>
  {#each online as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {/each}
  {#each seen as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {/each}
</div>
