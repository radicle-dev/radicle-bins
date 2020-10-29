<script>
  import Emoji from "./Emoji.svelte";

  export let avatar = null;
  export let title = null;

  const fmt = bg => `rgb(${bg.r}, ${bg.g}, ${bg.b})`;
</script>

<style>
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

<div
  class="avatar"
  style={`background-color: ${fmt(avatar.background)};`}
  {title}>
  <Emoji size="small" emoji={avatar.emoji} />
</div>
