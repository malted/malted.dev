<script>
	import { onMount, onDestroy } from "svelte";
	import { fade } from "svelte/transition";

	let display;
	export let title;

	let shouldDisplay;
	function scroll() {
		if (title) {
			shouldDisplay = display.getBoundingClientRect().top > title.getBoundingClientRect().bottom;
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

<svelte:window on:scroll={scroll} />

<div
	bind:this={display}
	on:focus
	on:mouseenter={() => (shouldDisplay = false)}
	on:mouseleave={() => (shouldDisplay = true)}
>
	<p in:fade out:fade style:opacity={shouldDisplay ? 0.4 : 0}>
		It's {time} on {day}<br />
		MST (Malted Standard Time)
	</p>
</div>

<style>
	div {
		position: fixed;
		right: 0;
		bottom: 0;
		margin: 1rem;
		width: fit-content;
	}

	p {
		font-size: 0.8rem;
		margin: 0;
		opacity: 0.4;
		text-align: right;
		transition: 0.25s;
	}
</style>
