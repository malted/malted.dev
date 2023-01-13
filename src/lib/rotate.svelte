<script>
	import { onMount } from "svelte";
	import * as UTILS from "$lib/utils.js";

	let container;

	// The difference in rotation from one element to the next, in degrees.
	const offset = 10;
	const scaleFactor = 10;

	// How much less the element rotation is on the x axis than the y axis;
	const xFactor = 0.5;

	onMount(() => {
		const warpTextOnScroll = () => {
			const s = window.scrollY;

			Object.values(container.children).forEach((el, idx) => {
				el.style.transform = `rotateY(${s / scaleFactor + idx * offset}deg)`;
			});

			// console.log(UTILS.easeOutIn(UTILS.lerpClamp(0, 1, window.scrollY / window.innerHeight)));
			container.style.top =
				UTILS.easeOut(UTILS.lerpClamp(0, 1, window.scrollY / window.innerHeight)) *
					window.innerHeight +
				"px";
		};
		warpTextOnScroll();
		document.addEventListener("scroll", warpTextOnScroll);
	});
</script>

<div id="container" bind:this={container}>
	<p>Scroll</p>
	<p>Scrollllllll</p>
	<p>Scrollllll meeeeee</p>
	<p>Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee</p>
</div>

<style>
	#container {
		position: fixed;
		top: 100vh;
	}
	#container > * {
		font-size: 2rem;
	}
</style>
