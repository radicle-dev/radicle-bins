<script>
  import Emoji from "./Emoji.svelte";

  export let user = null;

  const fmt = bg => `rgb(${bg.r}, ${bg.g}, ${bg.b})`;
</script>

<style>
  .user {
    display: flex;
  }
  .avatar {
    height: 1.5rem;
    width: 1.5rem;
    border-radius: 0.75rem;

    display: flex;
    justify-content: center;
    align-self: center;

    margin-right: 0.5rem;
  }
</style>

<div class="user">
  <div
    class="avatar"
    style={`background-color: ${fmt(user.avatar.background)};`}>
    <Emoji size="small" emoji={user.avatar.emoji} />
  </div>
  <p class="typo-text-bold">{user.name}</p>
</div>
