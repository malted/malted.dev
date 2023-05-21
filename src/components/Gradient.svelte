<script>
	import { onMount } from "svelte";
	import { Gradient } from "../lib/gradient.js";

	onMount(() => {
		new Gradient().initGradient("#gradient-canvas");
	});
</script>

<canvas id="gradient-canvas" data-transition-in />

<style>
	#gradient-canvas {
		width: 100%;
		height: 100%;
		position: fixed;
		inset: 0;
		z-index: -10;
		--gradient-color-1: #25273c;
		--gradient-color-2: #262730;
		--gradient-color-3: #0a0a0d;
		--gradient-color-4: #2b1212;
	}
</style>
