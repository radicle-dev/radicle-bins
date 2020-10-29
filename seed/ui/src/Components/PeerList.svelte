<script>
  import { quintOut } from "svelte/easing";
  import { crossfade } from "svelte/transition";

  import { online, seen } from "../state.js";

  import Peer from "./Peer.svelte";

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

<h3>Peers</h3>
<div class="list online">
  {#each $online as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {:else}No connected peers{/each}
</div>
<div class="list seen">
  {#each $seen as peer (peer.peerId)}
    <div in:receive={{ key: peer.peerId }} out:send={{ key: peer.peerId }}>
      <Peer {peer} />
    </div>
  {:else}No recently seen peers{/each}
</div>
