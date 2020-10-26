<script>
  export let user = null;
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
  <!-- TODO
  <div class="avatar" style={`background-color: ${user.color};`}>
    <Emoji size="small" emoji={user.emoji} />
  </div>
  -->
  <p class="typo-text-bold">{user.name}</p>
</div>
