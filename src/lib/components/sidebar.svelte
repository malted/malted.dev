<script>
	let x = 0;
	let y = 0;

	function move({ clientX, clientY }) {
		x = Math.round((clientX / window.innerWidth) * 360 - 180);
		y = Math.round((clientY / window.innerHeight) * 180 - 90);
	}
</script>

<svelte:body on:mousemove={move} />

<div id="coords">
	<span>{Math.abs(y)}°{y < 0 ? "N" : "S"}</span>
	<span>{Math.abs(x)}°{x < 0 ? "W" : "E"}</span>
</div>

<style>
	#coords {
		margin: 0;
		position: fixed;
		bottom: 0;
		right: 0;
		writing-mode: vertical-rl;
	}
</style>
