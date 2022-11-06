<script>
	import { onMount } from "svelte";

	const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));

	export let target;

	export const blinkCount = 2;
	export const blinkSpeed = 80;
	export const interDelay = 40;

	const uids = Array.from(Array(target.length), () => {
		return crypto.randomUUID();
	});

	onMount(async () => {
		// https://en.wikipedia.org/wiki/Fisher-Yates_shuffle
		const scrambled = uids;
		for (let i = scrambled.length - 1; i > 1; i--) {
			const j = Math.floor(Math.random() * i);
			const temp = scrambled[j];
			scrambled[j] = scrambled[i];
			scrambled[i] = temp;
		}

		for (let i = 0; i < scrambled.length; i++) {
			setTimeout(async () => {
				for (let j = 0; j < blinkCount * 2; j++) {
					document.getElementById(`letter-${scrambled[i]}`).style.opacity = j % 2 === 0 ? "0" : "1";
					await sleep(blinkSpeed);
				}
			});
			await sleep(interDelay);
		}
	});
</script>

{#each target as letter, idx}
	<span id="letter-{uids[idx]}">{letter}</span>
{/each}

<style>
	span {
		opacity: 0;
	}
</style>
