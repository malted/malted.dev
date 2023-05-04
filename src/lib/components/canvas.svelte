<script>
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
	import { EffectComposer } from "three/addons/postprocessing/EffectComposer.js";
	import { RenderPass } from "three/addons/postprocessing/RenderPass.js";
	import { OutlinePass } from "three/addons/postprocessing/OutlinePass.js";
	import { FilmPass } from "three/addons/postprocessing/FilmPass.js";
	import { TextGeometry } from "three/addons/geometries/TextGeometry.js";
	import { FontLoader } from "three/addons/loaders/FontLoader.js";

	export let data;

	let canvas;
	let fps;

	let cubeOpacity = 1;

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

		//# region Map
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

		let map = new THREE.Group();
		map.scale.setScalar(0.4);
		map.rotation.set(Math.PI / 2.5, Math.PI / -6, Math.PI / 9);
		map.position.x = 3;
		loader.load(
			"/old-map.glb",
			(gltf) => {
				const mapTexture = new THREE.TextureLoader().load(data.mapImg);
				mapTexture.flipY = false;

				let mapMat = new THREE.MeshPhongMaterial({
					map: mapTexture,
					flatShading: false,
					side: THREE.DoubleSide
				});
				gltf.scene.children[0].material = mapMat;
				// gltf.scene.children[0].material.map = mapTexture;

				outlinePass.selectedObjects.push(map);

				map.add(gltf.scene);
			},
			undefined,
			(error) => console.error
		);

		const fontLoader = new FontLoader();
		fontLoader.load("/fonts/Bakemono Variable_Regular.json", (font) => {
			const textGeometry = new TextGeometry("Live", {
				font: font,
				size: 0.3,
				height: 0.05,
				curveSegments: 12,
				bevelEnabled: true,
				bevelThickness: 0.05,
				bevelSize: 0.03,
				bevelOffset: 0,
				bevelSegments: 5
			});
			const textMaterial = new THREE.MeshPhongMaterial({
				color: 0x444444,
				flatShading: false
			});
			const textMesh = new THREE.Mesh(textGeometry, textMaterial);
			textMesh.position.set(1.25, 0, 2);
			textMesh.rotation.set(Math.PI / -2, 0, 0);
			map.add(textMesh);

			const liveIndicator = new THREE.Mesh(
				new THREE.SphereGeometry(0.075, 32, 32),
				new THREE.MeshBasicMaterial({
					color: 0xff0000,
					opacity: 1,
					transparent: true
				})
			);
			liveIndicator.position.set(1, 0, 1.88);
			map.add(liveIndicator);

			scene.add(map);
		});
		//#endregion

		//#region Lights
		const directionalLight = new THREE.DirectionalLight(0xffffff, 3);
		directionalLight.position.set(-5, -2, 3);
		scene.add(directionalLight);
		//#endregion

		//#region Postprocessing
		const filmPass = new FilmPass(0.25, 0, 648, false);
		filmPass.renderToScreen = true;
		composer.addPass(filmPass);
		// #endregion

		// Detect if the mouse is over the cube
		const raycaster = new THREE.Raycaster();
		const mouse = new THREE.Vector2();

		camera.position.z = 5;

		const clock = new THREE.Clock();

		let delta;
		function animate() {
			requestAnimationFrame(animate);

			delta = clock.getDelta();

			if (clock.getElapsedTime() % 0.05 < delta) {
				fps = Math.round(1 / delta);
			}

			if (map.children[2]) {
				map.children[2].material.opacity = Math.sin(clock.getElapsedTime() * 4) * 0.5 + 0.5;
			}

			// map.position.y = Math.sin(clock.getElapsedTime() * 2) * 0.15 + 2;

			if (raycaster.intersectObject(map).length > 0) {
				map.scale.lerp(new THREE.Vector3(0.7, 0.7, 0.7), delta * 20);

				outlinePass.edgeStrength = THREE.MathUtils.lerp(outlinePass.edgeStrength, 3, delta * 20);
			} else {
				map.scale.lerp(new THREE.Vector3(0.4, 0.4, 0.4), delta * 20);

				outlinePass.edgeStrength = THREE.MathUtils.lerp(outlinePass.edgeStrength, 0, delta * 20);
			}

			composer.render();
		}
		requestAnimationFrame(animate);

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
				insideCube.material.uniforms.opacity.value = cubeOpacity;
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
		const onScroll = (event) => {
			let scrollMax = 500;

			// Get scroll percentage
			let scrollPercent = window.scrollY / scrollMax;

			camera.position.z = THREE.MathUtils.lerp(5, 3, scrollPercent);
			// Lerp inner cube opacity
			cubeOpacity = THREE.MathUtils.lerp(1, 0, scrollPercent);

			// Lerp cube edges color between white and black
			edgesCube.material.color.setRGB(
				THREE.MathUtils.lerp(1, 0, scrollPercent),
				THREE.MathUtils.lerp(1, 0, scrollPercent),
				THREE.MathUtils.lerp(1, 0, scrollPercent)
			);

			raycaster.setFromCamera(mouse, camera);
			if (raycaster.intersectObject(insideCube).length > 0) {
				edgesCube.scale.setScalar(0);
				insideCube.material.uniforms.opacity.value = cubeOpacity;
			} else {
				edgesCube.scale.setScalar(1);
				insideCube.material.uniforms.opacity.value = 0;
			}
		};

		resize();

		window.addEventListener("resize", resize);
		window.addEventListener("mousemove", onMouseMove, false);
		window.addEventListener("scroll", onScroll, false);
		return () => {
			window.removeEventListener("resize", resize);
			window.removeEventListener("mousemove", onMouseMove, false);
			window.removeEventListener("scroll", onScroll, false);
		};
	});
</script>

<!-- <div>{fps} FPS</div> -->
<canvas bind:this={canvas} />

<style>
	canvas {
		position: fixed;
		inset: 0;
		width: 100%;
		height: 100%;
	}
</style>
