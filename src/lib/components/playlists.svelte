<script lang="ts">
	//TODO: Make a jukebox-type thing with the playlists
	// https://developer.spotify.com/console/get-playlist-images

	const playlists = [
		{ title: "January 2022", id: "0tqr8HyjlfVyFZ0r4p72Q8" },
		{ title: "February 2022", id: "70gUrjQHIlKecIq7ilmyqh" },
		{ title: "March 2022", id: "0x6LZHplNo7y7BZWFDi2G9" },
		{ title: "April 2022", id: "6PAzo4gTR02QIlaa96Y4cL" },
		{ title: "May 2022", id: "2N40IxOjQjqNWOmW4U1XsI" },
		{ title: "June 2022", id: "5SzBjwL9n7Y6l1pRG38yN1" },
		{ title: "July 2022", id: "7zHhEaRNdx1y6Q11KiY5pI" },
		{ title: "July 2022 Alternate", id: "3UK4PSSdLUcBZV0SrhWGT9" },
		{ title: "August 2022", id: "3aXmxvh3kXFdMebb78fRNX" },
		{ title: "August 2022 Alternate", id: "62ZazgaZaDjGOMLoeSqLdW" },
		{ title: "September 2022", id: "5v2fZkMTLb0N3hTFBDZW7G" },
		{ title: "October 2022", id: "25gVns4ROeQ7MdcQkGIHAG" },
		{ title: "November 2022", id: "4fvASZSMYi4EiBozdGsKMD" },
		{ title: "November 2022 Alternate", id: "5x8jjUXqC9lAVqRrCZdfyN" },
		{ title: "December 2022", id: "40LgfUS8cXW2BSf5tPsa3j" },
		{ title: "January 2023", id: "0MBNEh3dGUGA37oWZxHTcZ" },
		{ title: "February 2023", id: "4bT7dIwY15N98Su9HJxd9f" }
	];

	const sizes = { compact: 152, normal: 352 };
	const themes = { boring: 0, colourful: 1 };
</script>

<div id="container">
	{#each playlists as playlist}
		<iframe
			title={playlist.title}
			src="https://open.spotify.com/embed/playlist/{playlist.id}?utm_source=generator&theme={themes.colourful}"
			height={sizes.compact}
			frameBorder={0}
			allowfullscreen={false}
			allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
			loading="lazy"
		/>
	{/each}
</div>

<style>
	#container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		max-width: 40rem;
	}

	iframe {
		border-radius: 12px;
		width: 100%;
	}
</style>
