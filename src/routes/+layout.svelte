<slot />
<div id="img" />

<style>
	@font-face {
		font-family: "Gavency";
		src: url("/fonts/Gavency.otf") format("opentype");
	}
	@font-face {
		font-family: "Neue Outline";
		src: url("/fonts/NeueOutlineBlack.otf") format("opentype");
	}
	@font-face {
		font-family: "Neue";
		src: url("/fonts/NeueSemiBold.otf") format("opentype");
	}

	:root {
		font-family: "Neue";
		color: white;
		background-color: black;
	}

	:global(a) {
		text-decoration: none;
		color: white;
	}

	#img {
		background-image: url("/noise.png");
		z-index: -10;
		position: absolute;
		inset: 0;
		width: 100vw;
		height: 100vh;
		filter: contrast(60%);
	}
</style>
