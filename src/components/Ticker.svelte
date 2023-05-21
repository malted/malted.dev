<script>
	import { onMount } from "svelte";
	export let targetText;
	export let speed = 0.05;

	let offset = -300;
	let offsetHeight;

	onMount(() => {
		let clock = 0;
		const animate = () => {
			const delta = performance.now() - clock;
			offset -= delta * speed;

			if (offset <= -offsetHeight / 2) {
				offset = 0;
			}

			clock = performance.now();
			requestAnimationFrame(animate);
		};
		requestAnimationFrame(animate);
	});
</script>

<div id="parent">
	<p style:translate={`0 ${offset}px`}>
		{targetText}&nbsp;{targetText}&nbsp;
	</p>
</div>

<div id="reference-text" bind:offsetHeight aria-hidden="true">
	{targetText}&nbsp;{targetText}&nbsp;
</div>

<style>
	#parent {
		height: 10ch;
		overflow-y: clip;
		height: 20ch;

		border-top: 2px solid #ffffff30;
		border-bottom: 2px solid #ffffff30;
		/* border-radius: 0.5rem; */
	}

	p,
	#reference-text {
		white-space: nowrap;
		color: #ffffff30;
	}

	#reference-text {
		visibility: hidden;
		position: absolute;
	}
</style>
