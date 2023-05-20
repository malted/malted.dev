<script>
	import { onMount } from "svelte";
	import { calculatePointsOnArc } from "../lib/utils";
	import Projects from "../lib/projects.json";

	export let angleOffset = 0;

	onMount(() => {
		let clock = performance.now();
		const rotate = () => {
			const delta = performance.now() - clock;
			angleOffset += 0.01 * delta;
			requestAnimationFrame(rotate);
			clock = performance.now();
		};
		rotate();
	});

	const imgLength = 6.5; // In REM
	const angle = 360;
	const gap = (imgLength * 2.02) / (angle / 360);
	$: points = calculatePointsOnArc(
		Projects.length + 1,
		angle,
		gap,
		angleOffset
	);
</script>

<ul>
	{#each Projects as project, idx}
		<li
			style:translate={`50% ${points[idx].y}rem ${points[idx].x}rem`}
			style:transform={`rotateX(${-points[idx].tangentAngle}deg)`}
			style:opacity={`${1 + points[idx].x / 6}`}
		>
			<a href={project.url}>
				<div>
					<p class="project-name">{project.name}</p>
					<p>{project.desc}</p>
				</div>
				<img
					height="100"
					width="auto"
					src={`/projects/images/${project.name}.webp`}
					alt={`Preview for ${project.name}`}
				/>
			</a>
		</li>
	{/each}
</ul>

<style>
	ul {
		position: relative;
		perspective: 400px;
		transform-style: preserve-3d;
		list-style: none;

		--width: 70ch;
		width: var(--width);
	}
	li {
		position: absolute;
		top: 0;
		left: 0;
		text-align: center;
		width: calc(var(--width) / 2);

		display: flex;
		flex-direction: column;
		align-items: center;
	}
	a {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}
	.project-name {
		font-size: 2rem;
	}
</style>
