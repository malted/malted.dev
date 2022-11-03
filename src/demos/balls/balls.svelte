<script>
	import { onMount } from "svelte";

	const speed = 0.02;
	const magnitude = 50;
	const perspective = 150;

	let counter = 0;

	let ball1, ball2;
	let shadow1, shadow2;

	let x, z;

	function rotateBalls() {
		requestAnimationFrame(rotateBalls);

		counter++;

		x = Math.sin(counter * speed) * magnitude;
		z = Math.cos(counter * speed) * magnitude;

		ball1.style.transform = `
            perspective(${perspective}px)
            translateX(calc(${x}px - 50%))
            translateZ(${z}px)
        `;

		ball2.style.transform = `
            perspective(${perspective}px)
            translateX(calc(${-x}px - 50%))
            translateZ(${-z}px)
        `;

		shadow1.style.transform = `
            perspective(${perspective}px)
            translateX(calc(${x}px - 50%))
            translateY(3rem)
            translateZ(${z}px)
            rotateX(90deg)
        `;

		shadow2.style.transform = `
            perspective(${perspective}px)
            translateX(calc(${-x}px - 50%))
            translateY(3rem)
            translateZ(${-z}px)
            rotateX(90deg)
        `;
	}
	onMount(() => rotateBalls());

	function over(event) {
		event.target.style.borderRadius = "0";
		(event.target === ball1 ? shadow1 : shadow2).style.borderRadius = "0";
		(event.target === ball1 ? shadow1 : shadow2).style.filter = "none";

		event.target.style.boxShadow = "none";

		event.target.style.background = "#F0F";
		(event.target === ball1 ? shadow1 : shadow2).style.background = "#F0F";
	}
	function leave(event) {
		event.target.style.borderRadius = "50%";
		(event.target === ball1 ? shadow1 : shadow2).style.borderRadius = "50%";
		(event.target === ball1 ? shadow1 : shadow2).style.filter = "blur(1.5rem)";

		event.target.style.boxShadow = "inset 0 1rem 1rem rgba(0, 0, 0, 0.8)";

		event.target.style.background = event.target === ball1 ? "#4f46e5" : "#dc2626";
		(event.target === ball1 ? shadow1 : shadow2).style.background =
			event.target === ball1 ? "#6366f1" : "#ef4444";
	}
</script>

<div id="container">
	<div bind:this={ball1} on:focus on:mouseover={over} on:mouseleave={leave} id="ball-1" />
	<div bind:this={ball2} on:focus on:mouseover={over} on:mouseleave={leave} id="ball-2" />
	<div bind:this={shadow1} id="shadow-1" />
	<div bind:this={shadow2} id="shadow-2" />
</div>

<style>
	* {
		--ball-height: 3rem;
		--top: 5rem;
	}

	#container {
		transform-style: preserve-3d;
	}

	#container > * {
		width: var(--ball-height);
		height: var(--ball-height);
		border-radius: 50%;
		position: absolute;
		left: 50%;
		top: var(--top);
	}

	#ball-1,
	#ball-2 {
		box-shadow: inset 0 1rem 1rem rgba(0, 0, 0, 0.8);
	}

	#ball-1 {
		background: #4f46e5;
	}

	#ball-2 {
		background: #dc2626;
	}

	#shadow-1 {
		background: #6366f1;
	}
	#shadow-2 {
		background: #ef4444;
	}

	#shadow-1,
	#shadow-2 {
		filter: blur(1.5rem);
	}
</style>
