<script>
	export let nav;
	export let nurtureCanvas;

	$: if (nurtureCanvas) {
		nav.style.width = nurtureCanvas.clientWidth / 2 - nurtureCanvas.offsetLeft + "px";
	}
</script>

<div id="container">
	<nav bind:this={nav}>
		<div id="internal">
			<h3>&lt;nav /&gt;</h3>
		</div>
	</nav>
</div>

<style>
	#container {
		position: sticky;
		top: 0;
		width: 100%;
		margin: 0;
		margin-top: 60rem;
		z-index: -8;
	}
	nav {
		position: absolute;
		right: 0;
		background-color: black;
		/* box-shadow: 0 -5rem 1rem black; */
		opacity: 0;
		height: 100vh;
	}

	#internal {
		display: flex;
		justify-content: center;
		margin-top: 2rem;
	}

	h3 {
		writing-mode: vertical-rl;
	}
</style>
