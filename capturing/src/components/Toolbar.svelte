<script lang="ts">
  import Fa from "svelte-fa";
  import { faPlus, faChevronRight } from "@fortawesome/free-solid-svg-icons";
  import { bubble } from "svelte/internal";

  let selectTypeRef: HTMLSelectElement;

  function onClickSelectAppearance(): void {
    console.log("11111", selectTypeRef);
    const selectedIndex = selectTypeRef.selectedIndex;
    if (selectedIndex >= 0) {
      selectTypeRef.selectedIndex = selectedIndex === 2 ? 0 : selectedIndex + 1;
    }
  }
</script>

<div class="wrapper">
  <div class="container">
    <div class="new">
      <span>
        <Fa icon={faPlus} size="sm" />
      </span>
      <span class="new-text"> New </span>
    </div>
    <div class="divider" />
    <div class="select-type">
      <select
        class="select-type-select"
        bind:this={selectTypeRef}
        id="testtest"
      >
        <option value="サンプル1">サンプル1</option>
        <option value="サンプル2">サンプル2</option>
        <option value="サンプル3">サンプル3</option>
      </select>
      <button class="select-appearance" on:click={onClickSelectAppearance}>
        <Fa icon={faChevronRight} size="xs" rotate={90} />
      </button>
    </div>
  </div>
</div>

<style>
  .wrapper {
    height: 50px;
    background: var(--sub-bg-color);
    color: var(--sub-text-color);
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    justify-items: start;
    align-items: center;
    gap: 10px;
  }

  .container {
    display: flex;
    align-items: center;
  }

  .new {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 12px;

    cursor: pointer;
    width: 74px;
    height: 36px;
    background-color: #424242;
    border-radius: 4px;
  }

  .new-text {
    margin-left: 4px;
  }

  .divider {
    margin-left: 10px;
    height: 30px;
    width: 2px;
    background-color: #424242;
    border-radius: 4px;
  }

  .select-type {
    margin-left: 10px;
    padding-right: 8px;
    cursor: pointer;
  }

  .select-type-select {
    cursor: pointer;
  }

  .select-appearance {
    margin-left: 5px;
  }
</style>
