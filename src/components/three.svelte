<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { Line2 } from "three/addons/lines/Line2.js";
	import { LineMaterial } from "three/addons/lines/LineMaterial.js";
	import { LineGeometry } from "three/addons/lines/LineGeometry.js";
	import { TransformControls } from "three/addons/controls/TransformControls.js";
	import { createNoise2D } from "simplex-noise";

	import * as UTILS from "$lib/utils.js";

	const noise = createNoise2D();

	// ˈmɔːltɪd

	export let canvas;
	export let nav;

	onMount(() => {
		const startY = -20;
		const endY = 20;
		const lineRes = 0.01;
		let rotationSpeed = 0.001;
		const cameraZoom = 25;
		const startTime = 1;

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

		camera.position.z = cameraZoom;

		const clock = new THREE.Clock();
		clock.start();

		const lineMaterial = new LineMaterial({ vertexColors: true });
		const lineGeometry = new LineGeometry();
		const line = new Line2(lineGeometry, lineMaterial);

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
			const scrollDiff = UTILS.remap(0, scrollMax, 0, 1, scroll);

			lineMaterial.linewidth = UTILS.lerpClamp(0.0015, 0.004, scrollDiff);
			rotationSpeed = UTILS.lerpClamp(0.005, 0.25, scrollDiff);

			// JavaScript media queries 💀
			canvas.style.left =
				UTILS.lerpClamp(
					-20,
					40 - (canvas.clientWidth < 660 ? 10 : 0),
					UTILS.easeOut(scrollDiff / 5)
				) + "%";
			if (nav) nav.style.opacity = `${UTILS.lerpClamp(0, 0.5, scrollDiff)}`;

			let index = 0;
			let colourIndex = 0;
			for (let y = startY; y < endY; y += lineRes) {
				let scale = ((19.9 - Math.abs(y)) / 10) ** 3;
				scale = scale * UTILS.lerpClamp(1, 0, scrollDiff / 1.5);

				const x = noise(y, scrollDiff) * scale;
				const z = noise(y, scrollDiff + 10) * scale;
				const offsetY = noise(y, scrollDiff + 20) * scale;

				points[index++] = x;
				points[index++] = y + offsetY;
				points[index++] = z;

				let col = Math.abs(y) > UTILS.easeInQuart(scrollDiff) * 5 ? 1 : 0.5; //new THREE.Color(`hsl(${scrollDiff * 30}, 30%, 50%)`);
				const seg = ((y + 20) / 40) * startTime;

				if (first) {
					if (seg > clock.getElapsedTime()) {
						col = new THREE.Color(0x000000);
					} else if (seg > clock.getElapsedTime() - 0.2) {
						col = new THREE.Color((y + 20) / 40, 0, 1);
					} else if (seg > clock.getElapsedTime() - 0.5) {
						col = new THREE.Color(0x000000);
					} else if (seg > clock.getElapsedTime() - 0.8) {
						col = new THREE.Color((-y + 20) / 40, 1, (y + 20) / 40);
					}

					first = clock.getElapsedTime() < startTime + 12;
				}
				colours[colourIndex++] = col.isColor ? col.r : col;
				colours[colourIndex++] = col.isColor ? col.g : col;
				colours[colourIndex++] = col.isColor ? col.b : col;
			}

			lineGeometry.setPositions(points);
			lineGeometry.setColors(colours);
		}
		funkyLine();

		lineGeometry.setPositions(points);
		scene.add(line);

		let squiggleRotation = 0;
		function animate() {
			requestAnimationFrame(animate);

			squiggleRotation += rotationSpeed;
			line.rotation.y = squiggleRotation;

			if (first) funkyLine();

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
