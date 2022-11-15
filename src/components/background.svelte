<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import * as UTILS from "$lib/utils.js";

	let canvas;

	const sideLength = 12;
	const spread = 1;
	let points;
	let material;
	let geometry;
	let colours = [];

	onMount(() => {
		//#region THREE setup
		const scene = new THREE.Scene();

		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);

		const ratio = window.devicePixelRatio || 1;
		const renderer = new THREE.WebGLRenderer({
			canvas,
			antialias: true,
			alpha: true
		});
		const desiredWidth = (canvas.clientWidth * ratio) | 0;
		const desiredHeight = (canvas.clientHeight * ratio) | 0;
		renderer.setSize(desiredWidth, desiredHeight, false);
		document.body.appendChild(renderer.domElement);
		//#endregion

		geometry = new THREE.BufferGeometry();

		const positions = [];
		let colIndex = 0;
		for (let i = 0; i < sideLength ** 3; i++) {
			// Positions
			const x = i % sideLength;
			const y = Math.floor(i / sideLength ** 2);
			const z = Math.floor(i / sideLength) % sideLength;
			positions.push(
				(x - sideLength / 2) * spread,
				(y - sideLength / 2) * spread,
				(z - sideLength / 2) * spread
			);

			// Colours
			colours.push(x / sideLength, y / sideLength, z / sideLength, 0);
		}

		geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
		geometry.setAttribute("color", new THREE.Float32BufferAttribute(colours, 4));

		geometry.computeBoundingSphere();

		material = new THREE.PointsMaterial({
			size: 0.15,
			vertexColors: true,
			transparent: true
		});

		points = new THREE.Points(geometry, material);
		scene.add(points);
		points.position.z = -20;
		points.rotation.x = 0.4;

		camera.position.z = 0;

		function animate() {
			requestAnimationFrame(animate);

			points.rotation.y += 0.005;
			points.rotation.z += 0.005;

			renderer.render(scene, camera);
		}
		animate();
	});

	function scroll() {
		const scroll = document.documentElement.scrollTop - document.scrollingElement.clientHeight;
		const scrollMax = document.body.clientHeight - document.scrollingElement.clientHeight;
		const scrollDiff = UTILS.remap(0, scrollMax, 0, 1, scroll);

		let c = 3;
		for (let i = 0; i < sideLength ** 3 * 4; i += 1) {
			colours[(c += 4)] = (scrollDiff * sideLength ** 3 * 4) / i < 1 ? 0 : 1;
		}
		geometry.setAttribute("color", new THREE.Float32BufferAttribute(colours, 4));

		geometry.computeBoundingSphere();
	}
</script>

<svelte:window on:scroll={scroll} />

<canvas bind:this={canvas} />

<style>
	canvas {
		width: 100%;
		height: 100vh;
		top: 0;
		position: fixed;
		z-index: -10;
	}
</style>
