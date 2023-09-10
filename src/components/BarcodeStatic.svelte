<script>
	export let count = 10;
</script>

<div id="parent">
	{#each Array(count) as _, idx}
		<div class="bar" style={`--interval: ${2 ** idx / 10}s`} />
	{/each}
</div>

<style>
	#parent {
		background-color: var(--translucent);

		width: 1rem;
		height: 6rem;

		display: flex;
		flex-direction: row;
		justify-content: space-between;
		padding: 0.5rem;

		box-sizing:content-box;
	}

	@keyframes blink {
		50% {
			opacity: 0;
		}
	}

	.bar {
		width: 100%;
		background-color: var(--translucenter);
		height: 0.3rem;
		animation: blink var(--interval) step-start 0s infinite;
	}
</style>
