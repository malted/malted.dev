<script>
	import { onMount } from "svelte";
	import { Gradient } from "$lib/gradient.js";

	onMount(() => {
		const gradient = new Gradient();
		gradient.initGradient("#gradient-canvas");
	});
</script>

<canvas id="gradient-canvas" data-transition-in />

<style>
	#gradient-canvas {
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
</style>
