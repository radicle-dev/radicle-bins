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

<style>
  .list {
    padding-top: 1.5rem;
  }

  .online {
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
</style>

<div class="list online">
  {#each online as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {/each}
</div>
<div class="list seen">
  {#each seen as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {/each}
</div>
