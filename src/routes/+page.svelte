<script>
	import { fade } from "svelte/transition";

	let title;

	import Title from "$lib/title.svelte";
	import Rotate from "$lib/rotate.svelte";
	import Balls from "../demos/balls/balls.svelte";
	import Nurture from "$lib/nurture.svelte";
	import Mesh from "$lib/mesh.svelte";
	import Time from "$lib/time.svelte";
</script>

<main>
	<Mesh />
	<Nurture />
	<Title bind:title />
	<!-- <Balls /> -->
	<Time {title} />
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
