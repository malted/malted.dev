<script>
	import { onMount } from "svelte";

	let crudBarcode;
	let timeBarcode;

	function setupCrudBarcode() {
		const barcodeStrings = ["CREATE", "READ", "UPDATE", "DESTROY"];
		let interval = 250;

		setInterval(() => {
			JsBarcode(crudBarcode, barcodeStrings[barcodeIndex], {
				width: 2,
				height: 100,
				background: "transparent"
			});
			barcodeIndex = (barcodeIndex + 1) % barcodeStrings.length;
		}, interval);
	}

	function setupTimeBarcode() {
		function generate() {
			const time = performance.now();

			JsBarcode(timeBarcode, time, {
				width: 2,
				height: 15,
				textPosition: "top",
				background: "transparent"
			});

			requestAnimationFrame(generate);
		}
		requestAnimationFrame(generate);
	}

	onMount(() => {
		setupCrudBarcode();
		setupTimeBarcode();
	});
</script>

<svelte:head>
	<script src="src/lib/barcode.js"></script>
</svelte:head>

<div id="crud-barcode">
	<svg bind:this={crudBarcode} />
</div>
<div id="timestamp-barcode">
	<svg bind:this={timeBarcode} />
</div>

<style>
	div {
		display: flex;
		justify-content: center;
	}

	#timestamp-barcode {
		position: absolute;
		right: 0;
		bottom: 0;
	}
</style>
