<script>
    
  import { getCurrentWindow } from '@tauri-apps/api/window';
  let currentWindow = getCurrentWindow();
  const close = async () => {
  await currentWindow.close();
};

</script>

<div class="titlebar">
  <div class="drag-region"></div>
  <div class="buttons">
    <button on:click={close}>×</button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 32px;
    -webkit-user-select: none;
  }

  .drag-region {
    flex: 1;
    -webkit-app-region: drag;
  }

  .buttons button {
    -webkit-app-region: no-drag;
    background: transparent;
    border: none;
    font-size: 14px;
    width: 40px;
    height: 100%;
    cursor: pointer;
  }

  .buttons button:hover {
    background: #333;
  }
</style>
