<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { Line2 } from "three/addons/lines/Line2.js";
	import { LineMaterial } from "three/addons/lines/LineMaterial.js";
	import { LineGeometry } from "three/addons/lines/LineGeometry.js";
	import { createNoise2D } from "simplex-noise";

	const noise = createNoise2D();

	function lerp(a, b, t) {
		return (1 - t) * a + b * t;
	}
	function lerpClamp(a, b, t) {
		const q = (1 - t) * a + b * t;
		return t < 0 ? a : t > 1 ? b : q;
	}
	function invLerp(a, b, value) {
		return (value - a) / (b - a);
	}
	function remap(iMin, iMax, oMin, oMax, v) {
		return lerp(oMin, oMax, invLerp(iMin, iMax, v));
	}

	let canvas;

	onMount(() => {
		const startY = -20;
		const endY = 20;
		const lineRes = 0.01;
		let rotationSpeed = 0.001;
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
			const scroll = document.documentElement.scrollTop;
			const scrollMax = window.innerHeight;
			const scrollDiff = remap(0, scrollMax, 0, 1, scroll / 2);

			material.linewidth = lerpClamp(0.0025, 0.001, scrollDiff / 2) * window.devicePixelRatio;
			rotationSpeed = lerpClamp(0.005, 0.5, scrollDiff);
			line.position.x = lerpClamp(0, window.innerWidth / -100, scrollDiff);

			let index = 0;
			for (let y = startY; y < endY; y += lineRes) {
				let scale = ((19.9 - Math.abs(y)) / 10) ** 3;
				scale = scale * lerpClamp(1, 0, scrollDiff);

				const x = noise(y, scrollDiff) * scale;
				const z = noise(y, scrollDiff + 10) * scale;
				const offsetY = noise(y, scrollDiff + 20) * scale;

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
