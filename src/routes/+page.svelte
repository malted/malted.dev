<script>
	import Sidebar from "../lib/components/sidebar.svelte";
	import Barcode from "$lib/components/barcode.svelte";
	import Qr from "$lib/components/qr.svelte";
	import Canvas from "../lib/components/canvas.svelte";

	export let data;
	let loaded = false;
</script>

<Sidebar />
<!-- <Barcode /> -->
<Qr />
<Canvas {data} />
