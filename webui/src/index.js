import * as THREE from 'three';

import SpaceSocket from "./spacelab/spacesocket";
import PlanetParams from "./planets/Params";

import { OrbitControls, EffectComposer, RenderPass, FilmPass } from './viewport';
import { preGenerate, preloadAll } from './loaders/Preloader';
import $ from "jquery";

const context = {
	loadedPlanets: [],
	chat: [],
	renderCalls: []
}

window.$ = window.jQuery = $;

async function init() {
	// Early listeners
	document.addEventListener('SpaceError', (event) => {
		if (event.detail instanceof String) {
			console.error(`SpaceError: ${event.detail}`)
		} else if (event.detail.message) {
			console.error(`SpaceError: ${event.detail.message}`)
		} else {
			console.error(`SpaceError: `);
		}
		console.error(event.detail)
	})
	document.addEventListener('item_loaded', () => {
		updateStatus(`<B>Loaded ${window.imagesLoaded}/${window.imagesToLoadCount} images.</B>`);
	})
	document.addEventListener('chatMessage', (event) => {
		onChatMessage(event.detail);
	});
	document.addEventListener('addRenderCall', (event) => {
		context.renderCalls.push(event.detail);
	})
	document.addEventListener('wsConnected', () => {
		updateStatus(`<B>Connected!</B>`);
	})

	updateStatus(`Loading items...`);
	await preloadAll();
	updateStatus(`Pre-generating meshes`);
	await preGenerate();
	updateStatus(`All cached!`);

	// Main Code
	const SCREEN_HEIGHT = window.innerHeight - PlanetParams.viewportMargin * 2;
	const SCREEN_WIDTH = window.innerWidth;
	context.chatMsgList = document.createElement("ul");
	context.chatMsgList.id = "messageList";

	context.centeredOn = false;
	context.textureLoader = new THREE.TextureLoader();
	context.cubeTextureLoader = new THREE.CubeTextureLoader();
	context.clock = new THREE.Clock();

	document.getElementById("chat").innerHTML = "<h2>Chat</h2>";
	document.getElementById("chat").appendChild(context.chatMsgList);
	$("#chat_tab").on('click', () => { $("#chat").animate({ width: 'toggle' }); });
	$("#planets_tab").on('click', () => { $("#planets").animate({ height: 'toggle' }) });
	$("#chat").animate({ width: 'toggle' });
	$("#planets").animate({ height: 'toggle' });

	context.camera = new THREE.PerspectiveCamera(PlanetParams.viewportFov, SCREEN_WIDTH / SCREEN_HEIGHT, PlanetParams.viewportNear, PlanetParams.viewportFar);
	context.camera.position.z = PlanetParams.viewportZPos;

	context.scene = new THREE.Scene();
	context.scene.fog = new THREE.FogExp2(PlanetParams.sceneFogColor, PlanetParams.sceneFogDensity);
	context.scene.add(new THREE.AmbientLight(PlanetParams.sceneAmbientLight));

	context.dirLight = new THREE.DirectionalLight(PlanetParams.sceneSunColor);
	context.dirLight.position.set(-1, 0, 1).normalize();
	context.scene.add(context.dirLight);

	const texture = context.cubeTextureLoader.load([
		'img/BackgroundCube-0.jpg',
		'img/BackgroundCube-1.jpg',
		'img/BackgroundCube-2.jpg',
		'img/BackgroundCube-3.jpg',
		'img/BackgroundCube-4.jpg',
		'img/BackgroundCube-5.jpg',
	]);
	context.scene.background = texture;

	context.renderer = new THREE.WebGLRenderer({ antialias: PlanetParams.rendererAntialias });
	context.renderer.setPixelRatio(window.devicePixelRatio);
	context.renderer.setSize(SCREEN_WIDTH, SCREEN_HEIGHT);
	document.body.appendChild(context.renderer.domElement);

	//

	context.controls = new OrbitControls(context.camera, context.renderer.domElement);
	context.controls.minDistance = PlanetParams.cameraMinDistance;
	context.controls.maxDistance = PlanetParams.cameraMaxDistance;
	context.cameraTargetV = new THREE.Vector3();
	context.cameraPositionV = new THREE.Vector3();
	context.cameraMoving = false;
	//

	// stats = new Stats();
	// document.body.appendChild(stats.dom);


	window.addEventListener('resize', onWindowResize);

	context.renderModel = new RenderPass(context.scene, context.camera);
	//const effectFilm = new FilmPass(0.35, 0.75, 2048, false);

	context.composer = new EffectComposer(context.renderer);
	context.composer.addPass(context.renderModel);
	//context.composer.addPass(effectFilm);


	updateStatus(`<B>Connecting...</B>`);
	context.ss = new SpaceSocket('ws://localhost:3000/ws');
	window.ss = context.ss;
	document.addEventListener('new_planet', (event) => {
		const planetName = event.detail;
		//if (!context.centeredOn) {
		centerOn(planetName);
		//}
		context.loadedPlanets.push(planetName);
		refreshPlanets();
	})

	document.addEventListener('addMesh', (event) => {
		if (event.detail) {
			context.scene.add(event.detail);
		}
	})

	document.addEventListener('deleteMesh', (event) => {
		if (event.detail) {
			context.scene.remove(event.detail);
		}
	})
	document.addEventListener('sunPosition', (event) => {
		const { x, y, z, intensity } = event.detail;
		context.dirLight.position.set(x, y, z)
	})
	document.addEventListener('mousedown', () => {
		if (context.cameraMoving) {
			const deltaControl = context.controls.target.distanceTo(context.cameraTargetV);
			const deltaCamera = context.camera.position.distanceTo(context.cameraPositionV);
			console.log(`Cancelling movement ${deltaControl} ${deltaCamera}`);
			if (deltaControl < 20000 && deltaCamera < 20000) {
				context.cameraMoving = false;
			}
		}
	});

	animate();
}

function updateStatus(msg) {
	document.getElementById('connstatus').innerHTML = msg;
}

function onChatMessage(msg) {
	context.chat.push(msg);

	if (context.chat.length > 10) {
		context.chat.splice(0, 1);
	}

	while (context.chatMsgList.childNodes.length != context.chat.length) {
		context.chatMsgList.appendChild(document.createElement("li"));
	}

	for (let i = 0; i < context.chat.length; i++) {
		const msg = context.chat[i];
		context.chatMsgList.childNodes[context.chat.length - 1 - i].innerHTML = `<B>${msg.From}</B>: ${msg.Message}`;
	}
}

function refreshPlanets() {
	context.loadedPlanets.sort();
	// Update Planet List
	const planetOl = document.createElement("ol");
	context.loadedPlanets.map((planetName) => {
		const p = document.createElement("li");
		p.onclick = () => { centerOn(planetName); }
		p.innerText = planetName.split("-")[0];
		return p;
	}).forEach((p) => planetOl.appendChild(p));

	document.getElementById('planets').innerHTML = '';
	document.getElementById('planets').appendChild(planetOl);
}

function centerOn(voxelName) {
	console.log(`Centering on ${voxelName}`)
	const planet = context.ss.getPlanet(voxelName);
	if (!planet) {
		console.error(`Planet ${voxelName} not found`);
		return;
	}
	$("#planets").animate({ height: 'hide' });
	context.centeredOn = voxelName;
	document.getElementById('focusname').innerText = (`Focused on ${planet.instanceName}`)

	const radius = planet.maxHillSize / 2;

	context.cameraPositionV.x = planet.voxelData.X;
	context.cameraPositionV.y = planet.voxelData.Y;
	context.cameraPositionV.z = planet.voxelData.Z + radius * PlanetParams.viewportDistanceFactor;

	context.cameraTargetV.x = planet.voxelData.X;
	context.cameraTargetV.y = planet.voxelData.Y;
	context.cameraTargetV.z = planet.voxelData.Z;
	context.cameraMoving = true;
}

window.centerOn = centerOn;

function animate() {
	requestAnimationFrame(animate);
	if (context.cameraMoving) {
		const deltaControl = context.controls.target.distanceTo(context.cameraTargetV);
		const deltaCamera = context.camera.position.distanceTo(context.cameraPositionV);
		context.controls.update();
		context.controls.target.lerp(context.cameraTargetV, 0.1);
		context.camera.position.lerp(context.cameraPositionV, 0.1);
		if (deltaControl < 500 && deltaCamera < 500) {
			context.cameraMoving = false;
		}
	}
	render();
	//stats.update();
}

function render() {
	const delta = context.clock.getDelta();
	context.renderCalls.forEach((call) => call(delta, context));
	context.composer.render(delta);

}

function onWindowResize() {
	const SCREEN_HEIGHT = window.innerHeight - PlanetParams.viewportMargin * 2;
	const SCREEN_WIDTH = window.innerWidth;

	context.camera.aspect = SCREEN_WIDTH / SCREEN_HEIGHT;
	context.camera.updateProjectionMatrix();

	context.renderer.setSize(SCREEN_WIDTH, SCREEN_HEIGHT);
	context.composer.setSize(SCREEN_WIDTH, SCREEN_HEIGHT);
}

window.init = init;