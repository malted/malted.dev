<script>
	import { onMount, onDestroy } from "svelte";
	import { fade } from "svelte/transition";

	function elementsOverlap(el1, el2) {
		if (!el1 || !el2) return true;
		const domRect1 = el1.getBoundingClientRect();
		const domRect2 = el2.getBoundingClientRect();
		console.log(domRect2);

		return el1.top;
	}

	let display;
	export let title;

	let scrollY;
	let shouldDisplay;
	$: if (scrollY) {
		if (title) {
			shouldDisplay = display.getBoundingClientRect().top > title.getBoundingClientRect().bottom;
			console.log(
				`d${display.getBoundingClientRect().top} t: ${title.getBoundingClientRect().bottom}`
			);
		}
	}

	let time;
	let day;
	function getTime() {
		const date = new Date();
		time = date.toLocaleString("en-GB", {
			timeZone: "Europe/London",
			hour: "numeric",
			hourCycle: "h12",
			dayPeriod: "short"
		});
		day = date.toLocaleString("en-GB", {
			timeZone: "Europe/London",
			weekday: "long"
		});
	}
	getTime();

	const interval = setInterval(() => getTime(), 60_000);
	onDestroy(() => clearInterval(interval));
</script>

<svelte:window bind:scrollY />

<div bind:this={display}>
	{#if time && day && shouldDisplay}
		<p in:fade out:fade>It's {time} on {day} for me</p>
	{/if}
</div>

<style>
	div {
		position: fixed;
		right: 0;
		bottom: 0;
		margin: 1rem;
		width: 50%;
	}

	p {
		font-size: 0.8rem;
		margin: 0;
		opacity: 0.4;
		text-align: right;
	}
</style>
