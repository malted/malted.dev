<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { Line2 } from "three/addons/lines/Line2.js";
	import { LineMaterial } from "three/addons/lines/LineMaterial.js";
	import { LineGeometry } from "three/addons/lines/LineGeometry.js";
	import { createNoise2D } from "simplex-noise";

	const noise = createNoise2D();

	let canvas;

	onMount(() => {
		const startY = -20;
		const endY = 20;
		const lineRes = 0.01;
		const rotationSpeed = 0.005;
		const cameraZoom = 25;

		const scene = new THREE.Scene();
		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		camera.position.z = cameraZoom;

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

		const material = new LineMaterial({
			color: 0xffffff
		});

		const geometry = new LineGeometry();

		const line = new Line2(geometry, material);

		let points = [];
		function funkyLine() {
			const scroll = document.documentElement.scrollTop / 10000;
			const scrollMax = (document.documentElement.scrollHeight - window.innerHeight) / 10000;
			const scrollDiff = scrollMax - scroll;

			material.linewidth = scrollDiff * 0.003;

			let index = 0;
			for (let y = startY; y < endY; y += lineRes) {
				let scale = ((19.9 - Math.abs(y)) / 10) ** 3;
				scale = scale * scrollDiff;

				const x = noise(y, scroll) * scale;
				const z = noise(y, scroll + 10) * scale;
				const offsetY = noise(y, scroll + 20) * scale;

				// points[index++] = new THREE.Vector3(x, y + offsetY, z); //helo :D
				points[index++] = x;
				points[index++] = y + offsetY;
				points[index++] = z;
			}
			geometry.setPositions(points);
		}
		funkyLine();

		geometry.setPositions(points);
		scene.add(line);

		const clock = new THREE.Clock();
		let squiggleRotation = 0;
		function animate() {
			requestAnimationFrame(animate);

			squiggleRotation += rotationSpeed;
			line.rotation.y = squiggleRotation;

			renderer.render(scene, camera);
		}

		funkyLine();
		animate();

		window.addEventListener("scroll", () => funkyLine());
	});
</script>

<canvas bind:this={canvas} />

<style>
	canvas {
		width: 100vw;
		height: 100vh;
		top: 0;
		left: -20%;
		position: fixed;
		z-index: -10;
	}
</style>
