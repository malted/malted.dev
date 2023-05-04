<script>
	import { onMount } from "svelte";

	export let targetText;
	export let top;
	export let speed = 1;

	const padding = 12;
	let text = [];
	let offsetWidth;
	let containerWidth;

	let timeLastFrame = 0;
	const animate = () => {
		text.forEach((line, idx) => {
			// line.pos -= 1;
			line.pos -= ((performance.now() - timeLastFrame) / 5) * speed;
			if (line.pos < -offsetWidth) {
				line.pos = text[(idx - 1 + text.length) % text.length].pos + offsetWidth + padding;
			}
		});
		timeLastFrame = performance.now();

		text = text;

		requestAnimationFrame(animate);
	};

	onMount(() => {
		let textsNeeded = Math.ceil(containerWidth / offsetWidth) + 1;
		for (let i = 0; i < textsNeeded; i++) {
			text.push({
				text: targetText,
				pos: i === 0 ? 0 : text[i - 1].pos + offsetWidth
			});
		}
		animate();
	});
</script>

<div style:top bind:clientWidth={containerWidth} id="container">
	{#each text as line}
		<p style:left={`${line.pos}px`}>
			{line.text}
		</p>
	{/each}
</div>
<div id="reference-text" bind:offsetWidth aria-hidden="true">{targetText}</div>

<style>
	#container {
		width: 100%;
		position: absolute;
		overflow-x: clip;
	}

	#container > p,
	#reference-text {
		position: absolute;
		top: 0;
		left: 0;
		width: fit-content;
		white-space: nowrap;
		margin: 0;
		text-transform: uppercase;
		font-size: 3rem;
		color: #111111;
		font-weight: 900;
	}

	#reference-text {
		visibility: hidden;
	}
</style>
