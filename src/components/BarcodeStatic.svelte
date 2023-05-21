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
		background-color: #ffffff10;
		width: 1rem;
		height: 6rem;

		display: flex;
		flex-direction: row;
		justify-content: space-between;
		padding: 0.5rem;
	}

	@keyframes blink {
		50% {
			opacity: 0;
		}
	}

	.bar {
		width: 100%;
		background-color: #ffffff30;
		height: 0.3rem;
		animation: blink var(--interval) step-start 0s infinite;
	}
</style>
