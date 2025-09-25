<script>
  import "../app.css";
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  async function openGithub() {
    await openUrl("https://github.com/SharmaDevanshu089/ac-installer-gui");
    // await console.log("The Button is Pressed");
  }
  async function initiateClose() {
    const win = getCurrentWindow();
    await win.close();
  }
</script>

<div class="flex flex-col min-h-screen">
  
  <header class="navbar bg-base-200">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl" href="/" style="-webkit-app-region: drag;">Autocrate</a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1" style="-webkit-app-region: no-drag;">
        <li><button href="#" on:click={() => initiateClose()} >Close</button></li>
      </ul>
    </div>
  </header>

  <main class="flex-grow p-4">
    <slot />
  </main>

<footer class="footer footer-center py-4 bg-base-200 text-base-content relative">
  <p class="mx-auto">Copyright © 2025 - Made With 🖤 By Devanshu</p>
  <a 
    href="#" 
    on:click={() => openGithub()} 
    class="btn btn-link absolute right-4 top-1/2 -translate-y-1/2"
  >
    Github
  </a>
</footer>



  
</div>