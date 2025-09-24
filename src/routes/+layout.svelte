<script>
  import "../app.css";
  async function openGithub() {
    await open("https://github.com/SharmaDevanshu089/ac-installer-gui");
  }
</script>

<div class="flex flex-col min-h-screen">
  
  <header class="navbar bg-base-200">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl" href="/">Autocrate</a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><a href="#" on:click={() => openGithub()}>Github</a></li>
      </ul>
    </div>
  </header>

  <main class="flex-grow p-4">
    <slot />
  </main>

  <footer class="footer footer-center p-4 bg-base-200 text-base-content">
    <aside>
      <p>Copyright © 2025 - All right reserved by Devanshu</p>
    </aside>
  </footer>
  
</div>