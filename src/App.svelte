<script lang="ts">
  import Board from "./lib/Board.svelte";

  let boardComponent: Board;
</script>

<main class="container">
  <div>
    <Board bind:this={boardComponent} />
  </div>
  <div class="panel">
    <h3>Controls</h3>
    <button on:click={() => boardComponent.reset_board()}>Reset board</button>
  </div>
</main>

<style>
  .panel {
    text-align: left;
    width: 30%;
    margin-top: 5px;
    display: flex;
    flex-direction: column;
  }

  .panel button {
    margin-bottom: 4px;
  }
</style>
