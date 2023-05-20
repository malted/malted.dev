<script>
	export let text;
	export let initialDelay = 0;
	export let bySpaces = false;

	let totalIndex = initialDelay;
</script>

{#each text.split(" ") as word, wi}
	<span
		class={bySpaces ? "word" : ""}
		style:white-space={bySpaces ? "" : "nowrap"}
		style={bySpaces
			? `--delay: ${initialDelay + wi / text.split(" ").length}s`
			: ""}
	>
		{#each word.split("") as char}
			{#if char !== " "}
				{#if bySpaces}
					{char}
				{:else}
					<span
						class="char"
						style="--delay: {totalIndex++ / text.length}s"
						>{char}</span
					>
				{/if}
			{/if}
		{/each}
	</span>
	{" "}
{/each}

<style>
	@keyframes fadeIn {
		0% {
			translate: 0 0.75em;
			opacity: 0;
		}
		100% {
			translate: 0 0rem;
			opacity: 1;
		}
	}

	.char,
	.word {
		animation: fadeIn 0.5s var(--delay) forwards;
		opacity: 0;

		display: inline-block;
		translate: 0 0.75em;
	}
</style>
