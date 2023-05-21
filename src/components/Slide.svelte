<script>
	export let delay = 0;
	let clientWidth, clientHeight;

	$: translate = `-${clientWidth}px 0`;
</script>

<div
	bind:clientWidth
	bind:clientHeight
	style:translate
	style="--w:-{clientWidth}px; --h:-{clientHeight}; --delay: {delay}s"
>
	<slot />
</div>

<style>
	@keyframes a {
		0% {
			translate: var(--w);
		}
		100% {
			translate: 0 0;
		}
	}

	div {
		height: fit-content;
		width: fit-content;
		animation: a 1s var(--ease) var(--delay) forwards;
	}
</style>
