<script>
	export let text;
	export let initialDelay = 0;
	export let bySpaces = false;
	export let speed = 1.5;

	let totalIndex = 0;
</script>

{#each text.split(" ") as word, wi}
	<span
		class={(bySpaces ? "word" : "") + " vercel-lookin-ass"}
		style:white-space={bySpaces ? "" : "nowrap"}
		style={bySpaces
			? `--delay: ${
					initialDelay + wi / (text.split(" ").length * speed)
			  }s`
			: ""}
	>
		{#each word.split("") as char}
			{#if char !== " "}
				{#if bySpaces}
					{char}
				{:else}
					<span
						class="char vercel-lookin-ass"
						style="--delay: {initialDelay +
							totalIndex++ / (text.length * speed)}s">{char}</span
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
