<script>
	let x = 0;
	let y = 0;
	let title;

	function move({ clientX, clientY }) {
		x = Math.round((clientX / window.innerWidth) * 360 - 180);
		y = Math.round((clientY / window.innerHeight) * 180 - 90);

		title = (clientX / window.innerWidth) * 700 + 100;
		console.log((clientX / window.innerWidth) * 700 + 100);
	}
</script>

<svelte:body on:mousemove={move} />

<h1 style={`font-weight: ${title}`}>malted.dev</h1>

<div id="coords">
	<span>{Math.abs(y)}°{y < 0 ? "N" : "S"}</span>
	<span>{Math.abs(x)}°{x < 0 ? "W" : "E"}</span>
</div>

<style>
	* {
		writing-mode: vertical-rl;
	}

	h1,
	#coords {
		margin: 0;
		position: absolute;
		bottom: 0;
		right: 0;
	}

	h1 {
		top: 20%;
		font-size: 4rem;
		font-weight: 100;
	}
</style>
