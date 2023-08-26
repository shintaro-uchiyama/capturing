<script lang="ts">
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import ToolBar from "./components/Toolbar.svelte";
  import DrawArea from "./components/DrawArea.svelte";

  appWindow.setSize(new LogicalSize(480, 180));

  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    window.document.body.classList.add("dark");
  } else {
    window.document.body.classList.remove("dark");
  }

  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (event) => {
      if (event.matches) {
        window.document.body.classList.add("dark");
      } else {
        window.document.body.classList.remove("dark");
      }
    });
</script>

<main class="container">
  <ToolBar />
  <DrawArea />
</main>

<style>
  /**
   * ライトモードの際に適用されるスタイル変数設定
  */
  :global(body) {
    --main-bg-color: #efebe9;
    --main-text-color: #212121;

    --sub-bg-color: #d7ccc8;
    --sub-text-color: #212121;
  }

  /**
   * ダークモードの際に適用されるスタイル変数設定
  */
  :global(body.dark) {
    --main-bg-color: #424242;
    --main-text-color: #fafafa;

    --sub-bg-color: #212121;
    --sub-text-color: #fafafa;
  }

  :global(html) {
    height: 100%;
  }
  :global(body) {
    height: 100%;
  }

  .container {
    padding: 0;
    height: 100%;
  }
</style>
