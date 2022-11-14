<script>
	import { fade } from "svelte/transition";

	let title;
	let nurtureCanvas;
	let nav;

	import Title from "$lib/title.svelte";
	import Rotate from "$lib/rotate.svelte";
	import Balls from "../demos/balls/balls.svelte";
	import Nurture from "$lib/nurture.svelte";
	import Mesh from "$lib/mesh.svelte";
	import Time from "$lib/time.svelte";
	import Desc from "$lib/desc.svelte";
	import Sidebar from "$lib/sidebar.svelte";
</script>

<main>
	<Mesh />
	<Nurture bind:canvas={nurtureCanvas} {nav} />
	<Title bind:title />
	<!-- <Balls /> -->
	<Time {title} />
	<Desc />
	<Sidebar bind:nav {nurtureCanvas} />
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
