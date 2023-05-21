<script>
	import { timeUntilOpen } from "../lib/utils";
	import Reveal from "./Reveal.svelte";

	export let tz;

	const timeUntil = timeUntilOpen(tz);
</script>

<main>
	<h1><Reveal text="Closed for business" /></h1>
	<h2>
		<Reveal
			text={`malted.dev is open between 9am and 5pm, every day. Please check back in ${timeUntil}.`}
			initialDelay={0.8}
			bySpaces={true}
		/>
	</h2>

	<p>
		<Reveal text="Out of hours contact:" initialDelay={2} />
		<b><Reveal text="malted@{'{'}this domain{'}'}" initialDelay={2.6} /></b>
	</p>
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3rem;
		height: 100%;
		text-align: center;
	}

	main > * {
		max-width: 50ch;
		letter-spacing: 0.05em;
		padding: 1rem;
		z-index: 11;
	}
</style>
