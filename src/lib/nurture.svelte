<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { MeshLine, MeshLineMaterial, MeshLineRaycast } from "three.meshline";
	import { createNoise2D } from "simplex-noise";

	const noise = createNoise2D();

	let canvas;

	onMount(() => {
		const startY = -20;
		const endY = 20;
		const lineRes = 0.01;
		const rotationSpeed = 0.001;
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

		let material = new MeshLineMaterial({ color: 0xffffff, lineWidth: 0.15 });
		const geometry = new THREE.BufferGeometry();

		let mesh;
		function funkyLine() {
			const scroll = document.scrollingElement.scrollTop / 10000;

			const points = [];
			for (let y = startY; y < endY; y += lineRes) {
				const scale = ((19.9 - Math.abs(y)) / 10) ** 3;

				const x = noise(y, scroll) * scale;
				const z = noise(y, scroll + 10) * scale;
				const offsetY = noise(y, scroll + 20) * scale;

				points.push(new THREE.Vector3(x, y + offsetY, z));
			}

			geometry.setFromPoints(points);

			if (mesh) scene.remove(mesh);
			const line = new MeshLine();
			line.setGeometry(geometry);
			mesh = new THREE.Mesh(line, material);
			scene.add(mesh);
		}

		const clock = new THREE.Clock();
		function animate() {
			requestAnimationFrame(animate);

			mesh.rotation.y += rotationSpeed;

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
