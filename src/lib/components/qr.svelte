<script>
	import QRCode from "qrcode-svg";
	import { onMount } from "svelte";

	let qr;

	function randomRange(min, max) {
		return Math.floor(Math.random() * (max - min + 1)) + min;
	}

	onMount(() => {
		const code = new QRCode({
			content: "https://malted.dev",
			padding: 4,
			width: 256,
			height: 256,
			color: "#000000",
			background: "transparent",
			ecclevel: "M"
		}).svg();

		qr.innerHTML = code;

		qr.querySelectorAll("rect")[0].remove();
		// Translate every rect to the left edge of the svg by -31px, and down by 31px, to compensate for the removed rect
		qr.querySelectorAll("rect").forEach((rect, idx) => {
			rect.setAttribute("x", parseInt(rect.getAttribute("x")) - 31);
			rect.setAttribute("y", parseInt(rect.getAttribute("y")) - 31);
		});
		qr.querySelector("svg").setAttribute("width", 194);
		qr.querySelector("svg").setAttribute("height", 194);

		function foo() {
			const randomSquare =
				qr.querySelectorAll("rect")[Math.floor(Math.random() * qr.querySelectorAll("rect").length)];

			randomSquare.style.fill = `red`;

			setTimeout(() => {
				randomSquare.style.fill = "black";
			}, randomRange(100, 500));
		}

		function randomSquare() {
			foo();
			requestAnimationFrame(randomSquare);
		}
		requestAnimationFrame(randomSquare);
	});
</script>

<div bind:this={qr} />

<style>
	div {
		position: absolute;
		bottom: 0;
		left: 0;
		margin: 0;
		line-height: 0;
	}
</style>
