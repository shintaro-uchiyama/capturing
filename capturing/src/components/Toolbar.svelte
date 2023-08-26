<script lang="ts">
  import { faChevronRight, faPlus } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  let showSelectTypeOptions: boolean;

  const onClickSelectType = () => {
    showSelectTypeOptions = true;
  };
</script>

<div class="wrapper">
  <div class="container">
    <div class="new">
      <Fa icon={faPlus} size="sm" />
      <span class="new-text"> New </span>
    </div>
    <div class="divider" />
    <div class="select-type">
      <div class="drag-range" />
      <button class="select-appearance" on:click={onClickSelectType}>
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
    margin-left: 14px;

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
    margin-left: 14px;
    height: 30px;
    width: 2px;
    background-color: #424242;
    border-radius: 4px;
  }

  .select-type {
    width: 80px;
    height: 36px;
    margin-left: 14px;
    padding-right: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .drag-range {
    width: 22px;
    height: 18px;
    border: 1px dashed #fafafa;
    border-radius: 2px;
  }

  .select-appearance {
    margin-left: 12px;
  }
</style>
