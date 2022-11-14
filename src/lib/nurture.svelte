<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { Line2 } from "three/addons/lines/Line2.js";
	import { LineMaterial } from "three/addons/lines/LineMaterial.js";
	import { LineGeometry } from "three/addons/lines/LineGeometry.js";
	import { createNoise2D } from "simplex-noise";

	import { TransformControls } from "three/addons/controls/TransformControls.js";

	const noise = createNoise2D();

	function clamp(t) {
		return t < 0 ? 0 : t > 1 ? 1 : t;
	}
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
	function easeOut(x) {
		return 1 - Math.pow(1 - x, 3);
	}
	function easeInQuart(x) {
		return x * 2;
	}

	export let canvas;
	export let nav;

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

		const clock = new THREE.Clock();
		clock.start();

		const material = new LineMaterial({
			vertexColors: true
		});
		const geometry = new LineGeometry();
		const line = new Line2(geometry, material);

		// const cube = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshNormalMaterial());
		// scene.add(cube);

		// const transformControl = new TransformControls(camera, renderer.domElement);
		// transformControl.attach(cube);
		// scene.add(transformControl);

		let points = [];
		let colours = [];

		let first = true;
		function funkyLine() {
			const scroll = document.documentElement.scrollTop;
			const scrollMax = 1_000;
			const scrollDiff = remap(0, scrollMax, 0, 1, scroll);

			material.linewidth = lerpClamp(0.0015, 0.0035, scrollDiff);
			rotationSpeed = lerpClamp(0.005, 0.25, scrollDiff);

			// JavaScript media queries 💀
			canvas.style.left =
				lerpClamp(-20, 40 - (canvas.clientWidth < 660 ? 10 : 0), easeOut(scrollDiff / 2)) + "%";
			if (nav) nav.style.opacity = `${lerpClamp(0, 0.5, scrollDiff)}`;

			let index = 0;
			let colourIndex = 0;
			for (let y = startY; y < endY; y += lineRes) {
				let scale = ((19.9 - Math.abs(y)) / 10) ** 3;
				scale = scale * lerpClamp(1, 0, scrollDiff / 1.5);

				const x = noise(y, scrollDiff) * scale;
				const z = noise(y, scrollDiff + 10) * scale;
				const offsetY = noise(y, scrollDiff + 20) * scale;

				points[index++] = x;
				points[index++] = y + offsetY;
				points[index++] = z;

				colours[colourIndex++] =
					colours[colourIndex++] =
					colours[colourIndex++] =
						Math.abs(y) > scrollDiff * 20 ? 1 : 0.5;
			}
			geometry.setPositions(points);
			geometry.setColors(colours);
		}
		funkyLine();

		geometry.setPositions(points);
		scene.add(line);

		let squiggleRotation = 0;
		function animate() {
			requestAnimationFrame(animate);

			squiggleRotation += rotationSpeed;
			line.rotation.y = squiggleRotation;

			renderer.render(scene, camera);
		}

		animate();

		window.addEventListener("scroll", () => funkyLine());
	});
</script>

<canvas bind:this={canvas} />

<style>
	canvas {
		width: 100%;
		height: 100vh;
		top: 0;
		position: fixed;
		z-index: -5;
	}
</style>
