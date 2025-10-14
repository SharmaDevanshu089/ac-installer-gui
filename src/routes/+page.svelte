<script lang="ts">
  import { fly, fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
    import { invoke } from '@tauri-apps/api/core';

	let isModalOpen = false;
	let version = 'Getting Info';
	let fileName = '---------';
	let lastUpdated = '---------';
	let installButton: HTMLButtonElement;
	let conformInstallButtton: HTMLButtonElement;
	let isDownloading = false;
    let isInstalling = false;
    let errorMessage = '';
	let currentMessage = '';
	// let spanForLoading;
	// spanForLoading.classList.add('loading','loading-spinner','text-primary');

  async function initialiseReleaseData() {
	installButton.classList.add('loading','loading-spinner','text-primary');
    console.log("Initialising Release Data;")
    isModalOpen = true;
	setReleaseData();

  }
	const openModal =() => initialiseReleaseData();
	const closeModal = () => initialseCloseModal();
	const handleInstall = () => {
		closeModal();
		initiateInstallModal();
	};
	async function setReleaseData(){
	const releaseData = await invoke('get_release_data');
	version = releaseData.tag_name;
	fileName = releaseData.name;
	lastUpdated = releaseData.published_at;
	conformInstallButtton.classList.remove('btn-disabled');
	}
	function initialseCloseModal(){
		isModalOpen = false;
		installButton.classList.remove('loading','loading-spinner','text-primary');
	}
	function initiateInstallModal(){
		console.log("Install Modal Initiate");
		isDownloading = true;
		currentMessage = "Donwloading AutoCrate";
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
          <button class="btn btn-primary" on:click={openModal} bind:this={installButton}>
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
					<span class="font-semibold">Update Name:</span>
					<span class="truncate font-mono text-accent" title={fileName}>{fileName}</span>
				</div>
				<div class="flex items-center justify-between gap-4">
					<span class="font-semibold">Last Updated:</span>
					<span class="font-mono text-accent">{lastUpdated}</span>
				</div>
			</div>

			<div class="modal-action mt-6">
				<button class="btn" on:click={closeModal}>Cancel</button>

				<button class="btn btn-primary btn-disabled" on:click={handleInstall} bind:this={conformInstallButtton}>
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
{#if isDownloading || isInstalling}
    <div class="modal-overlay">
        <div class="modal-content">
            <div class="spinner-container">
                <div class="spinner" class:downloading={isDownloading} class:installing={isInstalling}></div>
            </div>

            {#key currentMessage}
                <p in:fly={{ y: 20, duration: 300, delay: 300 }}>
                    {currentMessage}
                </p>
            {/key}
        </div>
    </div>
{/if}
<style>
	.modal-backdrop {
		background-color: hsl(var(--b2, var(--b1)) / 0.6); /* Use daisyUI theme color with opacity */
		backdrop-filter: blur(5px);
	}
	.modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 100;
    }

    .modal-content {
        background: white;
        padding: 40px;
        border-radius: 12px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        text-align: center;
        overflow: hidden; /* Important for the text animation */
    }

    .spinner-container {
        margin-bottom: 20px;
    }

    .spinner {
        margin: 0 auto;
        width: 50px;
        height: 50px;
        border-radius: 50%;
        border: 5px solid transparent;
        /* The transition makes the change between states smooth */
        transition: border-style 0.4s ease, border-color 0.4s ease;
        animation: spin 1s linear infinite;
    }

    /* State 1: Downloading 🔵 */
    .spinner.downloading {
        border-style: dashed;
        border-top-color: #3498db; /* Blue */
    }

    /* State 2: Installing 🟢 */
    .spinner.installing {
        border-style: solid;
        border-top-color: #2ecc71; /* Green */
        animation-duration: 0.8s; /* Speed up for a "busier" look */
    }

    p {
        font-family: sans-serif;
        font-size: 1.1em;
        color: #555;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>