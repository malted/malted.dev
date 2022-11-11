<slot />

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

	@font-face {
		font-family: "Valorant";
		src: url("/fonts/Valorant.ttf") format("truetype");
	}

	:root {
		font-family: "Valorant";
		color: white;
		background-color: black;
	}

	:global(a) {
		text-decoration: none;
		color: white;
	}

	:global(*::selection) {
		color: transparent;
		text-shadow: 0 0 1rem white;
	}
</style>
