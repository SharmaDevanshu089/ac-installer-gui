<script lang="ts">
  import { fly, fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
    import { invoke } from '@tauri-apps/api/core';
	  import { getCurrentWindow } from '@tauri-apps/api/window';
	    import { openUrl } from '@tauri-apps/plugin-opener';


	let isModalOpen = false;
	let DoneDialog = false;
	let version = 'Getting Info';
	let fileName = '---------';
	let lastUpdated = '---------';
	let installButton: HTMLButtonElement;
	let conformInstallButtton: HTMLButtonElement;
	let isDownloading = false;
    let isInstalling = false;
    let errorMessage = '';
	let currentMessage = '';
	const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
	// let spanForLoading;
	// spanForLoading.classList.add('loading','loading-spinner','text-primary');


  async function openGithub() {
    await openUrl("https://github.com/SharmaDevanshu089/ac-installer-gui");
    // await console.log("The Button is Pressed");
  }
  async function initiateClose() {
    const win = getCurrentWindow();
    await win.close();
  }
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
	async function initiateInstallModal(){
		console.log("Install Modal Initiate");
		isDownloading = true;
		currentMessage = "Donwloading...";
		await console.log(invoke('download_binary'));
		await wait(5000);
		console.log("5 seconds passed. Switching state...");
        isDownloading = false;
        isInstalling = true;
        currentMessage = "Installing...";
		await invoke('add_to_path');
		await wait(1000);
		currentMessage = "Done...";
		await wait(1000);
		isInstalling = false;
		await wait(50);
		DoneDialog = true;
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
<dialog class="modal modal-bottom sm:modal-middle" open={isDownloading || isInstalling}>
    <div class="modal-box text-center">
        <span class="loading loading-spinner loading-lg mb-4"
              class:text-primary={isDownloading}
              class:text-success={isInstalling}
        ></span>

        {#key currentMessage}
            <p class="font-semibold text-lg" in:fly={{ y: 20, duration: 300, delay: 300 }}>
                {currentMessage}
            </p>
        {/key}
    </div>
    <form method="dialog" class="modal-backdrop">
        </form>
</dialog>
{#if DoneDialog}
	<dialog
		class="modal modal-bottom sm:modal-middle"
		open
		transition:fly|global={{ y: -50, duration: 400, easing: quintOut }}
	>
		<div class="modal-box text-center">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-16 h-16 mx-auto text-success"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/>
			</svg>

			<h3 class="font-bold text-2xl mt-4">Installation Complete!</h3>

			<p class="py-4">
				You may open Command Prompt and type "autocrate" to setup AutoCrate. Please Star on Github.
			</p>

			<div class="modal-action justify-center gap-2">
				<button class="btn btn-ghost" on:click={openGithub}>Open on GitHub</button>
				<button class="btn btn-primary" on:click={initiateClose}>Close !</button>
			</div>
		</div>

		<form method="dialog" class="modal-backdrop">
			<button on:click={closeSuccessDialog}></button>
		</form>
	</dialog>
{/if}
<style>
	.modal-backdrop {
		background-color: hsl(var(--b2, var(--b1)) / 0.6); /* Use daisyUI theme color with opacity */
		backdrop-filter: blur(5px);
	}
</style>