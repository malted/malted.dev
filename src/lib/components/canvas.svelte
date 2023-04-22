<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
	import { EffectComposer } from "three/addons/postprocessing/EffectComposer.js";
	import { RenderPass } from "three/addons/postprocessing/RenderPass.js";
	import { OutlinePass } from "three/addons/postprocessing/OutlinePass.js";
	import { FilmPass } from "three/addons/postprocessing/FilmPass.js";

	export let data;

	let canvas;

	const vertexShader = `
		varying vec3 vColor;
		
		void main() {
			vColor = normalize(position);
			gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
		}
	`;

	// Fragment shader
	const fragmentShader = `
		varying vec3 vColor;
		uniform float opacity;
		
		void main() {
			gl_FragColor = vec4(vColor * 0.7 + 0.5, opacity);
		}
	`;

	onMount(() => {
		const scene = new THREE.Scene();
		const camera = new THREE.PerspectiveCamera(75, 2, 0.1, 1000);
		const renderer = new THREE.WebGLRenderer({
			canvas,
			antialias: true,
			alpha: true
		});
		renderer.setPixelRatio(window.devicePixelRatio);
		const composer = new EffectComposer(renderer);
		composer.addPass(new RenderPass(scene, camera));
		const loader = new GLTFLoader();

		//#region Cube
		const cubeGroup = new THREE.Group();
		const boxGeometry = new THREE.BoxGeometry();

		const edgesCube = new THREE.LineSegments(
			new THREE.EdgesGeometry(boxGeometry),
			new THREE.LineBasicMaterial({ color: 0xffffff })
		);
		const insideCube = new THREE.Mesh(
			boxGeometry,
			new THREE.ShaderMaterial({
				vertexShader: vertexShader,
				fragmentShader: fragmentShader,
				transparent: true,
				uniforms: {
					opacity: { value: 0 }
				}
			})
		);
		// Vertex offset to center the cube slightly
		insideCube.scale.setScalar(1.01);

		cubeGroup.add(edgesCube);
		cubeGroup.add(insideCube);
		scene.add(cubeGroup);

		cubeGroup.scale.setScalar(1);
		//#endregion

		const outlinePass = new OutlinePass(
			new THREE.Vector2(window.innerWidth, window.innerHeight),
			scene,
			camera
		);
		outlinePass.edgeStrength = 3;
		outlinePass.edgeGlow = 0;
		outlinePass.edgeThickness = 1;
		outlinePass.visibleEdgeColor.set("#aaaaaa");
		outlinePass.pulsePeriod = 0;
		outlinePass.renderToScreen = true;
		composer.addPass(outlinePass);

		let map;
		loader.load(
			"/old-map.glb",
			function (gltf) {
				gltf.scene.scale.setScalar(0.4);
				gltf.scene.rotation.set(Math.PI / 2.5, Math.PI / -6, Math.PI / 9);
				gltf.scene.position.x = 3;
				const mapTexture = new THREE.TextureLoader().load(data.mapImg);
				mapTexture.flipY = false;
				gltf.scene.traverse((child) => {
					if (child.isMesh) {
						child.material.map = mapTexture;
					}
				});

				scene.add(gltf.scene);
				map = gltf.scene;

				// Apply it to the map
				map && outlinePass.selectedObjects.push(map);
			},
			undefined,
			function (error) {
				console.error(error);
			}
		);

		//#region Lights
		const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
		directionalLight.position.set(-5, -2, 3);
		scene.add(directionalLight);

		const pointLight2 = new THREE.PointLight(0xffff00, 1);
		pointLight2.position.set(2, 0.5, 1);
		scene.add(pointLight2);

		// const pointLightHelper2 = new THREE.PointLightHelper(pointLight2, 0.1);
		// scene.add(pointLightHelper2);

		const pointLight3 = new THREE.PointLight(0xffffff, 3);
		pointLight3.position.set(3, 3, 3);
		scene.add(pointLight3);

		//#endregion

		//#region Postprocessing
		const filmPass = new FilmPass(0.35, 0.1, 648, false);
		filmPass.renderToScreen = true;
		composer.addPass(filmPass);
		// #endregion

		// Detect if the mouse is over the cube
		const raycaster = new THREE.Raycaster();
		const mouse = new THREE.Vector2();

		camera.position.z = 5;

		const clock = new THREE.Clock();

		function animate() {
			requestAnimationFrame(animate);

			const delta = clock.getDelta();

			if (map) {
				map.position.y = Math.sin(clock.getElapsedTime() * 2) * 0.15 + 2;

				if (raycaster.intersectObject(map).length > 0) {
					map.scale.lerp(new THREE.Vector3(0.7, 0.7, 0.7), delta * 20);

					outlinePass.edgeStrength = THREE.MathUtils.lerp(outlinePass.edgeStrength, 3, delta * 20);
				} else {
					map.scale.lerp(new THREE.Vector3(0.4, 0.4, 0.4), delta * 20);

					outlinePass.edgeStrength = THREE.MathUtils.lerp(outlinePass.edgeStrength, 0, delta * 20);
				}
			}

			composer.render();
			// renderer.render(scene, camera);
		}

		animate();

		const resize = () => {
			camera.aspect = window.innerWidth / window.innerHeight;
			camera.updateProjectionMatrix();
			renderer.setSize(window.innerWidth, window.innerHeight);
			composer.setSize(window.innerWidth, window.innerHeight);
		};
		const onMouseMove = (event) => {
			mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
			mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

			// update the picking ray with the camera and mouse position
			raycaster.setFromCamera(mouse, camera);

			if (raycaster.intersectObject(insideCube).length > 0) {
				edgesCube.scale.setScalar(0);
				insideCube.material.uniforms.opacity.value = 1;
			} else {
				edgesCube.scale.setScalar(1);
				insideCube.material.uniforms.opacity.value = 0;
			}

			// Rotate cube towards mouse
			const m = new THREE.Vector3(mouse.x, mouse.y, 0);
			// Get magnitude of mouse vector
			const mag = Math.sqrt(m.x * m.x + m.y * m.y);

			cubeGroup.lookAt(new THREE.Vector3(mouse.x, mouse.y, mag + 0.1));

			if (map) {
				map.rotation.set(Math.PI / 2.5 + -mouse.y * 0.1, Math.PI / -6 + mouse.x * 0.1, Math.PI / 9);
			}
		};

		resize();

		window.addEventListener("resize", resize);
		window.addEventListener("mousemove", onMouseMove, false);

		return () => {
			window.removeEventListener("resize", resize);
			window.removeEventListener("mousemove", onMouseMove, false);
		};
	});
</script>

<canvas bind:this={canvas} />

<style>
	canvas {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}
</style>
