<script>
	export let nav;
	export let nurtureCanvas;

	$: if (nurtureCanvas) {
		nav.style.width = nurtureCanvas.clientWidth / 2 - nurtureCanvas.offsetLeft + "px";
	}
</script>

<div id="container">
	<nav bind:this={nav}>
		<h3>&lt;nav /&gt;</h3>
	</nav>
</div>

<style>
	#container {
		position: sticky;
		top: 0;
		width: 100%;
		margin: 0;
		margin-top: 60rem;
	}
	nav {
		position: absolute;
		right: 0;
		background-color: black;
		height: 100vh;
		display: flex;
		justify-content: center;
	}

	h3 {
		writing-mode: vertical-rl;
		margin: 0;
	}
</style>
