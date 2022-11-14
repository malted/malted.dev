<script>
	import { onMount } from "svelte";
	import font from "three/examples/fonts/helvetiker_regular.typeface.json";

	let container;

	onMount(() => {
		const loader = new THREE.FontLoader();
		loader.load("/fonts/foo.typeface.json", () => {
			const text = new THREE.TextGeometry("Sample text!", {
				font,
				size: 80,
				height: 5
			});
		});
	});

	function scroll() {
		const s = document.documentElement.scrollTop;
		container.style.transform = `scale(${s / 10}px)`;
	}
</script>

<svelte:window on:scroll={scroll} />

<div id="container" bind:this={container}>
	<!-- <h3>ˈmɔːltɪd</h3>
		<h4>noun.</h4> -->
	<h3>Line 1</h3>
</div>

<style>
	#container {
		position: fixed;
	}

	#container > * {
		font-family: "monospace";
		font-weight: 900;
	}
	h4 {
		font-size: 1rem;
	}
</style>
