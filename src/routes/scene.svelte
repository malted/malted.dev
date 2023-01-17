<script lang="ts">
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
	import { EffectComposer } from "three/examples/jsm/postprocessing/EffectComposer.js";
	import { ShaderPass } from "three/examples/jsm/postprocessing/ShaderPass.js";
	import { RenderPass } from "three/examples/jsm/postprocessing/RenderPass.js";
	import { FilmPass } from "three/examples/jsm/postprocessing/FilmPass.js";
	import { SobelOperatorShader } from "three/examples/jsm/shaders/SobelOperatorShader.js";
	import { RGBShiftShader } from "three/examples/jsm/shaders/RGBShiftShader.js";
	import { HalftonePass } from "three/examples/jsm/postprocessing/HalftonePass.js";
	import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

	let canvas: HTMLCanvasElement;

	onMount(() => {
		const scene = new THREE.Scene();
		const camera = new THREE.PerspectiveCamera(
			75,
			window.innerWidth / window.innerHeight,
			0.1,
			1000
		);
		camera.position.set(0, 1, 3);

		const renderer = new THREE.WebGLRenderer({
			canvas,
			alpha: true,
			antialias: true,
			powerPreference: "high-performance"
		});
		renderer.setSize(canvas.clientWidth, canvas.clientHeight);
		renderer.setClearColor(0x00ff00, 0);

		// Ambient light
		const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
		scene.add(ambientLight);

		// Directional light
		const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
		directionalLight.position.set(-1, 1, 5);
		scene.add(directionalLight);

		const composer = new EffectComposer(renderer);
		composer.addPass(new RenderPass(scene, camera));

		const rgbShift = new ShaderPass(RGBShiftShader);
		rgbShift.uniforms["amount"].value = 0.0035 * (window.devicePixelRatio || 1);
		composer.addPass(rgbShift);
		const halftonePass = new HalftonePass(window.innerWidth, window.innerHeight, {
			shape: 1,
			radius: 5,
			rotateR: Math.PI / 12,
			rotateB: (Math.PI / 12) * 2,
			rotateG: (Math.PI / 12) * 3,
			scatter: 0,
			blending: 1,
			blendingMode: 1,
			greyscale: false,
			disable: false
		});
		composer.addPass(halftonePass);
		const effectSobel = new ShaderPass(SobelOperatorShader);
		effectSobel.uniforms["resolution"].value.x = window.innerWidth * window.devicePixelRatio;
		effectSobel.uniforms["resolution"].value.y = window.innerHeight * window.devicePixelRatio;
		composer.addPass(effectSobel);
		// const filmPass = new FilmPass(0.2, 0.25, 648, 0);
		// composer.addPass(filmPass);

		new OrbitControls(camera, renderer.domElement);

		// Load textures from /static/terminal
		const textureLoader = new THREE.TextureLoader();
		const flipY = (t: any) => (t.flipY = false);
		const albedo = textureLoader.load("/terminal/textures/TerminalMaterial_baseColor.png", flipY);
		const emissive = textureLoader.load("/terminal/textures/TerminalMaterial_emissive.png", flipY);
		const roughness = textureLoader.load(
			"/terminal/textures/TerminalMaterial_metallicRoughness.png",
			flipY
		);
		const normal = textureLoader.load("/terminal/textures/TerminalMaterial_normal.png", flipY);

		let terminal: THREE.Object3D;
		const loader = new GLTFLoader();
		loader.load(
			"/terminal/terminal.glb",
			(gltf) => {
				gltf.scene.traverse((child) => {
					if (child instanceof THREE.Mesh) {
						child.material = new THREE.MeshStandardMaterial({
							map: albedo,
							emissiveMap: emissive,
							roughnessMap: roughness,
							normalMap: normal
						});
					}
				});
				terminal = gltf.scene;
				scene.add(terminal);
				animate();
			},
			(xhr) => {
				console.log((xhr.loaded / xhr.total) * 100 + "% loaded");
			},
			(error) => {
				console.log("An error happened");
			}
		);

		setInterval(() => {
			// rgbShift.uniforms["angle"].value = Math.random() * Math.PI * 2;
		}, 100);

		function animate() {
			requestAnimationFrame(animate);

			terminal.rotation.y += 0.01;
			// dotScreen.uniforms["angle"].value += 0.00025;

			composer.render();
		}

		const resizeHandler = () => {
			camera.aspect = canvas.clientWidth / canvas.clientHeight;
			camera.updateProjectionMatrix();
			renderer.setSize(canvas.clientWidth, canvas.clientHeight);
			composer.setSize(canvas.clientWidth, canvas.clientHeight);
		};
		resizeHandler();
		window.addEventListener("resize", resizeHandler);
	});
</script>

<canvas bind:this={canvas} />

<style>
	canvas {
		width: 100%;
		height: 100%;
	}
</style>
