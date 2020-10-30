<script>
  import Icon from "./Icon";

  export let style = "";
  export let copyContent = "";
  export let iconBeforeCopy = Icon.CopySmall;
  export let iconAfterCopy = Icon.CheckSmall;

  export let styleContent = true;
  export let showIcon = true;

  let slotContent = "";
  let copyIcon = iconBeforeCopy;

  let copied = false;

  const copy = () => {
    if (copied) return;

    const content = copyContent.length ? copyContent : slotContent;
    if (content) copyToClipboard(content.trim());

    copied = true;

    copyIcon = Icon.CheckSmall;
    setTimeout(() => {
      copyIcon = Icon.CopySmall;
      copied = false;
    }, 1000);
  };

  const copyToClipboard = (function initClipboardText() {
    const textarea = document.createElement("textarea");

    // Move it off-screen.
    textarea.style.cssText = "position: absolute; left: -99999em";

    // Set to readonly to prevent mobile devices opening a keyboard when
    // text is .select()'ed.
    textarea.setAttribute("readonly", true);

    document.body.appendChild(textarea);

    return function setClipboardText(text) {
      textarea.value = text;

      // Check if there is any content selected previously.
      const selected =
        document.getSelection().rangeCount > 0
          ? document.getSelection().getRangeAt(0)
          : false;

      // iOS Safari blocks programmatic execCommand copying normally, without this hack.
      // https://stackoverflow.com/questions/34045777/copy-to-clipboard-using-javascript-in-ios
      if (navigator.userAgent.match(/ipad|ipod|iphone/i)) {
        const editable = textarea.contentEditable;
        textarea.contentEditable = true;
        const range = document.createRange();
        range.selectNodeContents(textarea);
        const sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
        textarea.setSelectionRange(0, 999999);
        textarea.contentEditable = editable;
      } else {
        textarea.select();
      }

      try {
        const result = document.execCommand("copy");

        // Restore previous selection.
        if (selected) {
          document.getSelection().removeAllRanges();
          document.getSelection().addRange(selected);
        }

        return result;
      } catch (err) {
        console.error(err);
        return false;
      }
    };
  })();
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: inline-flex;
    white-space: nowrap;
    width: 100%;
  }

  .basic {
    display: flex;
    min-height: 24px;
    width: 100%;
  }

  .content {
    align-items: center;
    background-color: var(--color-foreground-level-2);
    padding: 0 4px;
    margin-left: -4px;
    border-radius: 4px;
    color: var(--color-foreground-level-6);
  }
</style>

<div class="wrapper" on:click|stopPropagation={copy}>
  <span
    class="basic"
    class:content={styleContent}
    bind:this={slotContent}
    {style}>
    <slot />
    {#if showIcon && iconBeforeCopy && iconAfterCopy}
      <svelte:component
        this={copyIcon}
        style="display: flex; margin-left: 0.25rem; min-width: 24px;" />
    {/if}
  </span>
</div>
