<script lang="ts">
  // 1. Import the animation functions from Svelte
  import { fly, fade } from 'svelte/transition';

  function handleInstallClick() {
    my_modal_2.showModal()
  }
</script>

<div class="hero min-h-full">
  <div class="hero-content text-center">
    
    <div
      class="card w-96 bg-base-100 shadow-xl"
      in:fly={{ y: 50, duration: 500, delay: 300, easing: quintOut }}
    >
      <figure class="px-10 pt-10">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-24 h-24 text-primary">
          <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
        </svg>
      </figure>

      <div class="card-body items-center text-center">
        <h2 class="card-title text-2xl">AutoCrate Installer</h2>
        <p>The easiest way to install and manage your AutoCrate projects.</p>
        
        <div class="card-actions mt-4" in:fade={{ duration: 500, delay: 600 }}>
          <button class="btn btn-primary" on:click={handleInstallClick}>
            Install Now
          </button>
        </div>
      </div>
    </div>

  </div>
</div>
<dialog id="my_modal_2" class="modal">
  <div class="modal-box">
    <h3 class="text-lg font-bold">Hello!</h3>
    <p class="py-4">Press ESC key or click outside to close</p>
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>