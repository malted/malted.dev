<script>
	let weight = 400;
	let mono = 0;
	let spacing = 0.294;

	function move({ clientX, clientY }) {
		weight = (clientX / window.innerWidth) * 700 + 100;
		mono = (clientY / window.innerHeight) * 1000;

		spacing = (mono / 1000) * -0.294 + 0.294;
	}
</script>

<svelte:body on:mousemove={move} />

<nav>
	<div id="left">
		<h1
			style={`
                font-weight: ${weight}; 
                font-variation-settings: "MONO" ${mono}; 
                letter-spacing: ${spacing}rem
            `}
		>
			malted.dev
		</h1>
	</div>
</nav>

<style>
	nav {
		position: sticky;
		top: 0;
		left: 0;
		width: 100%;
		z-index: 2;
		background-color: #111111;
	}

	#left {
		padding: 1rem 1rem 0.8rem 1rem;
	}

	h1 {
		font-size: 3rem;
		letter-spacing: 0.294rem;
		margin: 0;
	}
</style>
