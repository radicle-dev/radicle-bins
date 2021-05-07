<script>
  import Peer from "./Peer.svelte";

  export let online = [];
  export let seen = [];
</script>

<div>
  {#each online as peer (peer.peerId)}
    <div>
      <Peer {peer} />
    </div>
  {/each}
  {#each seen as peer (peer.peerId)}
    <div>
      <Peer {peer} />
    </div>
  {/each}
</div>
