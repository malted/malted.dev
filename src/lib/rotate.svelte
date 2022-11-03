<script>
	import { onMount } from "svelte";

	let l1, l2, l3, l4;

	onMount(() => {
		const warpTextOnScroll = () => {
			const s = window.scrollY;
			l1.style.transform = `rotateY(${s / 20 - 5}deg) rotateX(${s / -30}deg)`;
			l2.style.transform = `rotateY(${s / 22 - 5}deg) rotateX(${s / -32}deg)`;
			l3.style.transform = `rotateY(${s / 24 - 5}deg) rotateX(${s / -34}deg)`;
			l4.style.transform = `rotateY(${s / 26 - 5}deg) rotateX(${s / -36}deg)`;
		};
		warpTextOnScroll();
		document.addEventListener("scroll", warpTextOnScroll);
	});
</script>

<div id="container">
	<p bind:this={l1}>Scroll</p>
	<p bind:this={l2}>Scrollllllll</p>
	<p bind:this={l3}>Scrollllll meeeeee</p>
	<p bind:this={l4}>Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee</p>
</div>

<style>
	#container {
		height: 200vh;
		top: 100vh;
		position: relative;
	}
	p {
		font-size: 3rem;
		font-family: "Gavency";
		text-align: center;
	}
</style>
