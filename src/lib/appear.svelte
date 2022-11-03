<script>
	import { onMount } from "svelte";
	import { fly } from "svelte/transition";

	export let delay;
	export let y;

	let visible = false;
	onMount(() => (visible = true));
</script>

{#if visible}
	<div in:fly={{ delay, y }}><slot /></div>
{/if}

<style>
	div {
		margin: 0;
	}
</style>
