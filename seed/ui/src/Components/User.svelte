<script>
  import Avatar from "./Avatar.svelte";
  export let user = null;
</script>

<style>
  .user {
    display: flex;
  }
</style>

<div class="user">
  {#if user}
    <Avatar avatar={user.avatar} />
    <p class="typo-text-bold">{user.name}</p>
  {:else}
    <p>🖥</p>
  {/if}
</div>
