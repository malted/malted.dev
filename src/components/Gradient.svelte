<script>
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";

	import { Gradient } from "../lib/gradient.js";

	export let delay = 0;
	let showCanvas = false;

	onMount(() => {
		const gradient = new Gradient();
		gradient.initGradient("#gradient-canvas");

		setTimeout(() => (showCanvas = true), delay * 1000);
	});
</script>

{#if !showCanvas}
	<div id="cover" out:fade />
{/if}
<canvas id="gradient-canvas" data-transition-in />

<style>
	#gradient-canvas,
	#cover {
		width: 100vw;
		height: 100vh;
		position: fixed;
		inset: 0;
		z-index: -10;
		--gradient-color-1: #25273c;
		--gradient-color-2: #262730;
		--gradient-color-3: #0a0a0d;
		--gradient-color-4: #2b1212;
	}

	#cover {
		z-index: 10;
		background: black;
	}
</style>
