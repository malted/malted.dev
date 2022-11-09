<script>
	import { fade } from "svelte/transition";

	import Title from "$lib/title.svelte";
	import Rotate from "$lib/rotate.svelte";
	import Balls from "../demos/balls/balls.svelte";
	import Nurture from "$lib/nurture.svelte";
	import Mesh from "$lib/mesh.svelte";
</script>

<main>
	<Mesh />
	<Nurture />
	<!-- <Title /> -->
	<!-- <Balls /> -->
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
