<script>
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";

	import { Gradient } from "$lib/gradient.js";

	import { ready } from "$lib/stores.js";

	let showCanvas = false;
	ready.subscribe((value) => {
		showCanvas = value;
	});

	onMount(() => {
		const gradient = new Gradient();
		gradient.initGradient("#gradient-canvas");
		ready.set(true);
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
		z-index: -50;
		--gradient-color-1: #1d1d29;
		--gradient-color-2: #111118;
		--gradient-color-3: #0c0c11;
		--gradient-color-4: #0a0a0d;
	}

	#cover {
		z-index: 40;
		background: black;
	}
</style>
