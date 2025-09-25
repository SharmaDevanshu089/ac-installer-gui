<script lang="ts">
  import { fly, fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	let isModalOpen = false;
	let version = 'Getting Info';
	let fileName = '---------';
	let lastUpdated = '---------';

  async function initialiseReleaseData() {
    console.log("Initialising Release Data;")
    isModalOpen = true;
  }
	const openModal =() => initialiseReleaseData();
	const closeModal = () => (isModalOpen = false);
	const handleInstall = () => {
		console.log('Installation confirmed!');
		//to be added 
		closeModal();
	};
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
          <button class="btn btn-primary" on:click={openModal}>
            Install Now
          </button>
        </div>
      </div>
    </div>

  </div>
</div>{#if isModalOpen}
	<div class="modal-backdrop" on:click={closeModal}></div>

	<dialog
		class="modal modal-bottom sm:modal-middle modal-open"
		transition:fly={{ y: -60, duration: 400, easing: quintOut }}
	>
		<div class="modal-box">
			<h3 class="text-lg font-bold">Confirm Download Details</h3>
			<p class="py-4">Please verify the following details before proceeding.</p>

			<div class="space-y-3 p-2 rounded-md bg-base-200">
				<div class="flex items-center justify-between gap-4">
					<span class="font-semibold">Version:</span>
					<span class="font-mono text-accent">{version}</span>
				</div>
				<div class="flex items-center justify-between gap-4">
					<span class="font-semibold">File Name:</span>
					<span class="truncate font-mono text-accent" title={fileName}>{fileName}</span>
				</div>
				<div class="flex items-center justify-between gap-4">
					<span class="font-semibold">Last Updated:</span>
					<span class="font-mono text-accent">{lastUpdated}</span>
				</div>
			</div>

			<div class="modal-action mt-6">
				<button class="btn" on:click={closeModal}>Cancel</button>

				<button class="btn btn-primary" on:click={handleInstall}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 20 20"
						fill="currentColor"
						class="h-5 w-5"
					>
						<path
							d="M10.75 2.75a.75.75 0 0 0-1.5 0v8.614L6.295 8.235a.75.75 0 1 0-1.09 1.03l4.25 4.5a.75.75 0 0 0 1.09 0l4.25-4.5a.75.75 0 0 0-1.09-1.03l-2.955 3.129V2.75Z"
						/>
						<path
							d="M3.5 12.75a.75.75 0 0 0-1.5 0v2.5A2.75 2.75 0 0 0 4.75 18h10.5A2.75 2.75 0 0 0 18 15.25v-2.5a.75.75 0 0 0-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5Z"
						/>
					</svg>
					Install
				</button>
			</div>
		</div>
	</dialog>
{/if}

<style>
	.modal-backdrop {
		background-color: hsl(var(--b2, var(--b1)) / 0.6); /* Use daisyUI theme color with opacity */
		backdrop-filter: blur(5px);
	}
</style>