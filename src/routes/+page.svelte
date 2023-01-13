<script>
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";

	let title;
	let nurtureCanvas;
	let nav;

	import Title from "/src/components/title.svelte";
	// import Rotate from "$lib/rotate.svelte";
	// import Balls from "../demos/balls/balls.svelte";
	import Nurture from "/src/components/nurture.svelte";
	import Mesh from "/src/components/mesh.svelte";
	import Time from "/src/components/time.svelte";
	import Sidebar from "/src/components/sidebar.svelte";
	import Background from "../components/background.svelte";
</script>

<main>
	<Mesh />
	<Nurture bind:canvas={nurtureCanvas} {nav} />
	<Title bind:title />
	<!-- <Balls /> -->
	<Time {title} />
	<Sidebar bind:nav {nurtureCanvas} />
	<!-- <Background /> -->
</main>

<style>
	main {
		height: 1000vh;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
