<script>
	import { fly } from "svelte/transition";

	import BlinkText from "$lib/blinktext.svelte";

	import { ready } from "$lib/stores.js";

	export let title;

	let showTitle = false;
	ready.subscribe((value) => {
		showTitle = value;
	});
</script>

{#if showTitle}
	<div id="container" bind:this={title}>
		<h1><BlinkText delay="100" target="malted.dev" /></h1>
		<div>
			<h2 in:fly={{ delay: 800, y: -100 }}>Patent pending</h2>
		</div>
	</div>
{/if}

<style>
	#container {
		display: flex;
		align-items: center;
		position: absolute;
		right: 0;
		margin: 2rem;
		gap: 5rem;
		white-space: nowrap;

		writing-mode: vertical-rl;
	}

	h1,
	h2 {
		margin: 0;
	}

	h1 {
		font-size: 4rem;
	}

	h2 {
		opacity: 0.6;
	}
</style>
