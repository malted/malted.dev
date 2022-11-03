<script>
	import { onMount } from "svelte";

	const sleep = (time) => new Promise((resolve) => setTimeout(resolve, time));

	export let target;

	export const blinkCount = 2;
	export const blinkSpeed = 80;
	export const interDelay = 40;
	export const offCol = "black";
	export const onCol = "white";

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
					document.getElementById(`letter-${scrambled[i]}`).style.color =
						j % 2 === 0 ? onCol : offCol;
					await sleep(blinkSpeed);
				}
				document.getElementById(`letter-${scrambled[i]}`).style.color = onCol;
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
		color: black;
	}
</style>
