<script>
	import { fade } from "svelte/transition";

	import BlinkText from "$lib/blinktext.svelte";
	import Appear from "$lib/appear.svelte";
	import Rotate from "$lib/rotate.svelte";
	import Balls from "../demos/balls/balls.svelte";
	import { onMount } from "svelte";

	let ballsVisible = false;
	onMount(() => {
		setTimeout(() => {
			ballsVisible = true;
		}, 1200);
	});
</script>

<main>
	<h1><BlinkText target="malted.dev" /></h1>
	<Appear delay="800" y="-100"><h2>Patent pending.</h2></Appear>

	{#if ballsVisible}
		<div in:fade><Balls /></div>
	{/if}
	<Rotate />
</main>

<style>
	main {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	h1 {
		font-size: 5rem;
	}
	h2 {
		font-size: 1.2rem;
	}
</style>
