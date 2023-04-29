import * as THREE from 'three';

import SpaceSocket from "./spacelab/spacesocket";
import PlanetParams from "./planets/Params";

import { OrbitControls, EffectComposer, RenderPass, FilmPass } from './viewport';
import { preGenerate, preloadAll } from './loaders/Preloader';
import $ from "jquery";
import CameraControls from 'camera-controls';
import * as holdEvent from 'hold-event';

CameraControls.install({ THREE: THREE });

const context = {
	loadedPlanets: [],
	chat: [],
	renderCalls: []
}

window.$ = window.jQuery = $;

const KEYCODE = {
	W: 87,
	A: 65,
	S: 83,
	D: 68,
	ARROW_LEFT: 37,
	ARROW_UP: 38,
	ARROW_RIGHT: 39,
	ARROW_DOWN: 40,
};


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
	$("#chat").animate({ width: 'toggle' });
	$("#planets").animate({ height: 'toggle' });

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
	context.pointer = new THREE.Vector2();

	document.getElementById("chat").innerHTML = "<h2>Chat</h2>";
	document.getElementById("chat").appendChild(context.chatMsgList);
	$("#chat_tab").on('click', () => { $("#chat").animate({ width: 'toggle' }); });
	$("#planets_tab").on('click', () => { $("#planets").animate({ height: 'toggle' }) });

	context.camera = new THREE.PerspectiveCamera(PlanetParams.viewportFov, SCREEN_WIDTH / SCREEN_HEIGHT, PlanetParams.viewportNear, PlanetParams.viewportFar);
	context.camera.position.z = PlanetParams.viewportZPos;

	context.scene = new THREE.Scene();
	context.scene.fog = new THREE.FogExp2(PlanetParams.sceneFogColor, PlanetParams.sceneFogDensity);
	context.scene.add(new THREE.AmbientLight(PlanetParams.sceneAmbientLight));

	context.dirLight = new THREE.DirectionalLight(PlanetParams.sceneSunColor);
	context.dirLight.position.set(-1, 0, 1).normalize();
	context.scene.add(context.dirLight);
	context.planetBoundingSpheres = [];

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

	context.controls = new CameraControls(context.camera, context.renderer.domElement);
	context.controls.minDistance = PlanetParams.cameraMinDistance;
	context.controls.maxDistance = PlanetParams.cameraMaxDistance;
	context.controls.setLookAt(
		0, 0, 0,
		0, 0, 200e3 * PlanetParams.viewportDistanceFactor,
		true);

	context.controls.infinityDolly = true;

	context.controls.dollySpeed = PlanetParams.dollySpeed.normal;

	function isKey(e, key) {
		return e.key == key
	}

	const dollySpeed = (released, controls) => {
		return (event) => {
			if (released && (isKey(event, 'Shift') || isKey(event, 'Control'))) {
				controls.dollySpeed = PlanetParams.dollySpeed.normal;
				return;
			}

			if (isKey(event, 'Shift')) {
				controls.dollySpeed = PlanetParams.dollySpeed.fast;
				return;

			} else if (isKey(event, 'Control')) {
				controls.dollySpeed = PlanetParams.dollySpeed.slow;
				return;
			}
		}
	}

	window.addEventListener('keyup', dollySpeed(true, context.controls));
	window.addEventListener('keydown', dollySpeed(false, context.controls));

	context.cameraTargetV = new THREE.Vector3();
	context.cameraPositionV = new THREE.Vector3();
	context.cameraMoving = false;
	context.raycaster = new THREE.Raycaster();

	// stats = new Stats();
	// document.body.appendChild(stats.dom);


	window.addEventListener('resize', onWindowResize);
	window.context = context;

	context.renderModel = new RenderPass(context.scene, context.camera);
	//const effectFilm = new FilmPass(0.35, 0.75, 2048, false);

	context.composer = new EffectComposer(context.renderer);
	context.composer.addPass(context.renderModel);
	//context.composer.addPass(effectFilm);


	updateStatus(`<B>Connecting...</B>`);
	context.ss = new SpaceSocket(PlanetParams.WebsocketURL);
	window.ss = context.ss;
	document.addEventListener('new_planet', (event) => {
		const planetName = event.detail;
		//centerOn(planetName);
		context.controls.fitToBox( context.scene, true, { paddingLeft: 25, paddingRight: 25, paddingBottom: 25, paddingTop: 25 } )
		context.loadedPlanets.push(planetName);
		context.planetBoundingSpheres.push(context.ss.getPlanet(planetName).boundingSphere);
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

	document.addEventListener('mousemove', onPointerMove);
	context.renderer.domElement.addEventListener('click', () => {
		if (context.INTERSECTED != null) {
			centerOn(context.INTERSECTED.planetName);
			context.INTERSECTED.visible = false;
			context.INTERSECTED.null;
		}
	})
	context.renderer.domElement.addEventListener('dblclick', () => {
		if (context.centeredOn) {
			const planet = context.ss.getPlanet(context.centeredOn);
			context.controls.fitToBox( planet.mesh, true, { paddingLeft: 5, paddingRight: 5, paddingBottom: 5, paddingTop: 5 } );
		}
	})

	const wKey = new holdEvent.KeyboardKeyHold(KEYCODE.W, 16.666);
	const aKey = new holdEvent.KeyboardKeyHold(KEYCODE.A, 16.666);
	const sKey = new holdEvent.KeyboardKeyHold(KEYCODE.S, 16.666);
	const dKey = new holdEvent.KeyboardKeyHold(KEYCODE.D, 16.666);
	aKey.addEventListener('holding', function (event) { context.controls.truck(- PlanetParams.cameraSpeed * event.deltaTime, 0, false) });
	dKey.addEventListener('holding', function (event) { context.controls.truck(PlanetParams.cameraSpeed * event.deltaTime, 0, false) });
	wKey.addEventListener('holding', function (event) { context.controls.forward(PlanetParams.cameraSpeed * event.deltaTime, false) });
	sKey.addEventListener('holding', function (event) { context.controls.forward(- PlanetParams.cameraSpeed * event.deltaTime, false) });

	const leftKey = new holdEvent.KeyboardKeyHold(KEYCODE.ARROW_LEFT, 100);
	const rightKey = new holdEvent.KeyboardKeyHold(KEYCODE.ARROW_RIGHT, 100);
	const upKey = new holdEvent.KeyboardKeyHold(KEYCODE.ARROW_UP, 100);
	const downKey = new holdEvent.KeyboardKeyHold(KEYCODE.ARROW_DOWN, 100);
	leftKey.addEventListener('holding', function (event) { context.controls.rotate(- 0.1 * THREE.MathUtils.DEG2RAD * event.deltaTime, 0, true) });
	rightKey.addEventListener('holding', function (event) { context.controls.rotate(0.1 * THREE.MathUtils.DEG2RAD * event.deltaTime, 0, true) });
	upKey.addEventListener('holding', function (event) { context.controls.rotate(0, - 0.05 * THREE.MathUtils.DEG2RAD * event.deltaTime, true) });
	downKey.addEventListener('holding', function (event) { context.controls.rotate(0, 0.05 * THREE.MathUtils.DEG2RAD * event.deltaTime, true) });


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

function onPointerMove(event) {
	context.pointer.x = (event.clientX / window.innerWidth) * 2 - 1;
	context.pointer.y = - (event.clientY / window.innerHeight) * 2 + 1;
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

	context.cameraTargetV.x = planet.voxelData.X;
	context.cameraTargetV.y = planet.voxelData.Y;
	context.cameraTargetV.z = planet.voxelData.Z;
	const planetOrbit = 5000 + planet.voxelData.Size/2;
	if (context.controls.distance < planetOrbit) {
		context.controls.distance = planetOrbit;
	}
	context.controls.moveTo(context.cameraTargetV.x, context.cameraTargetV.y, context.cameraTargetV.z, true);
	context.controls.fitToBox( planet.mesh, true, { paddingLeft: 5, paddingRight: 5, paddingBottom: 5, paddingTop: 5 } );
}

window.centerOn = centerOn;

function animate() {
	requestAnimationFrame(animate);

	context.raycaster.setFromCamera(context.pointer, context.camera);
	const intersects = context.raycaster.intersectObjects(context.planetBoundingSpheres, false);
	if (intersects.length > 0) {
		const obj = intersects[0].object;
		if (context.INTERSECTED != obj && obj.planetName !== context.centeredOn) {
			if (context.INTERSECTED) {
				context.INTERSECTED.material.emissive.setHex(context.INTERSECTED.currentHex)
				context.INTERSECTED.material.color.setHex(context.INTERSECTED.currentHex)
				context.INTERSECTED.visible = false;
			}

			context.INTERSECTED = intersects[0].object;
			context.INTERSECTED.currentHex = context.INTERSECTED.material.emissive.getHex();
			context.INTERSECTED.material.emissive.setHex(0xff0000);
			context.INTERSECTED.material.color.setHex(0xff0000);
			context.INTERSECTED.visible = true;
			context.renderer.domElement.style.cursor = 'pointer';
		}
	} else {
		if (context.INTERSECTED) {
			context.INTERSECTED.material.emissive.setHex(context.INTERSECTED.currentHex)
			context.INTERSECTED.material.color.setHex(context.INTERSECTED.currentHex)
			context.INTERSECTED.visible = false;
		}
		context.renderer.domElement.style.cursor = '';
		context.INTERSECTED = null;
	}
	if (context.INTERSECTED && context.INTERSECTED.visible) {
		context.INTERSECTED.rotation.y += 0.01;
	}
	render();
	//stats.update();
}

function render() {
	const delta = context.clock.getDelta();
	context.controls.update(delta);
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