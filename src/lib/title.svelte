<script>
	import { onMount } from "svelte";
	import { fly } from "svelte/transition";

	import BlinkText from "$lib/blinktext.svelte";

	let visible = false;
	onMount(() => (visible = true));
</script>

<div id="container">
	<h1><BlinkText target="malted.dev" /></h1>
	<div>
		{#if visible}
			<h2 in:fly={{ delay: 800, y: -100 }}>Patent pending</h2>
		{/if}
	</div>
</div>

<style>
	#container {
		display: flex;
		align-items: center;
		position: absolute;
		right: 0;
		margin: 2rem;
		gap: 5rem;

		writing-mode: vertical-rl;
	}

	h1,
	h2 {
		margin: 0;
	}

	h1 {
		font-size: 4rem;
	}

	h2 {
		opacity: 0.5;
	}
</style>
