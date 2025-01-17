/*
    SpaceLab Websocket Handler
*/
import * as THREE from 'three';

import PlanetData from "../planets/PlanetData";
import { loadTexture } from "../loaders/TexturePreloader"
import { planetHeightMaps, planetNormals, planetTextures, upFaceIdx } from "../loaders/Preloader";
import PlanetParams from "../planets/Params";
import { magicSphereGeometry } from "../geometry/MagicSphereGeometry"
import { pickRandomColor } from '../colors';
import { newText } from '../draw';
import { GenerateRing } from '../geometry/Ring';
import { WebsocketMessage } from '../spaceproto/spaceproto.js';

class SpaceSocket {

    constructor(socketUrl) {
        this.socketUrl = socketUrl;
        this.grids = {};
        this.voxels = {};
        this.ownerColors = {};
        this.connect();
    }

    connect() {
        if (this.conn) {
            this.conn.close();
            this.conn.onopen = null;
            this.conn.onmessage = null;
            this.conn.onerror = null;
            this.conn.onclose = null;
        }

        console.log(`Connecting to ${this.socketUrl}`);
        this.conn = new WebSocket(this.socketUrl);
        this.conn.onopen = this.onOpen.bind(this);
        this.conn.onmessage = this.onMessage.bind(this);
        this.conn.onerror = this.onError.bind(this);
        this.conn.onclose = this.onClose.bind(this);
    }

    getPlanet(name) {
        return this.voxels[name];
    }

    async planetsUpdateCallback(data) {
        const ss = this;

        const loads = Object.keys(data).map(async (name) => {
            const voxelData = data[name];
            if (name != "" && voxelData.DebugName.indexOf("MyPlanet") > 0) {
                const planetNameInfo = name.split("-");
                const instanceName = planetNameInfo[0];
                const basePlanet = planetNameInfo[1] || "";
                const generatorParams = planetNameInfo.length == 4 ? planetNameInfo[3] : planetNameInfo[2];
                console.log(`Detected planet ${instanceName} (${name})`);
                // console.log(JSON.stringify(voxelData, null, 2));
                const planetDataByInstance = PlanetData[basePlanet.toLowerCase()];
                const planetDataByName = PlanetData[instanceName.toLowerCase()];

                if (!planetDataByInstance && !planetDataByName) {
                    console.error(`Base planet ${basePlanet} (${name}) not found in database. Skipping it...`);
                    return;
                }
                const planetData = {
                    basePlanet,
                    instanceName,
                    generatorParams,
                    data: planetDataByInstance ? planetDataByInstance : planetDataByName,
                    voxelData,
                }

                planetData.heightMapTextures = planetHeightMaps(planetData.data.pathPrefix);
                planetData.textures = planetTextures(planetData.data.pathPrefix);
                planetData.normals = planetNormals(planetData.data.pathPrefix);

                planetData.maxHillSize = (1 + voxelData.HillParameters.Max) * voxelData.Size;
                planetData.minHillSize = (1 + voxelData.HillParameters.Min) * voxelData.Size;
                planetData.hillDelta = (planetData.maxHillSize - planetData.minHillSize) / 2;

                planetData.materials = await Promise.all(planetData.textures.map(async (texture, idx) => {
                    let texRotation = 0;
                    if (idx == upFaceIdx) {
                        texRotation = Math.PI;
                    }
                    const heightMapTexture = planetData.heightMapTextures[idx];
                    const hmTex = await loadTexture(heightMapTexture);
                    const normalTex = await loadTexture(planetData.normals[idx]);
                    const texMap = await loadTexture(texture);

                    hmTex.center = new THREE.Vector2(0.5, 0.5);
                    hmTex.rotation = texRotation;
                    hmTex.minFilter = hmTex.magFilter = THREE.LinearFilter;
                    normalTex.center = new THREE.Vector2(0.5, 0.5);
                    normalTex.rotation = texRotation;
                    normalTex.minFilter = normalTex.magFilter = THREE.LinearFilter;
                    texMap.center = new THREE.Vector2(0.5, 0.5);
                    texMap.rotation = texRotation;
                    normalTex.minFilter = normalTex.magFilter = THREE.LinearFilter;

                    return new THREE.MeshPhongMaterial({
                        specular: 0x333333,
                        shininess: 5,
                        map: texMap,
                        displacementMap: hmTex,
                        displacementScale: planetData.hillDelta,
                        //normalMap: normalTex, // Normal map is currently not working due seams
                        normalScale: new THREE.Vector2(1, - 1),
                        bumpMap: hmTex,
                        bumpScale: planetData.hillDelta / 2,
                        wireframe: false
                    });
                }));
                planetData.mesh = new THREE.LOD();
                PlanetParams.planetLOD.forEach((lod) => {
                    const geometry = magicSphereGeometry(planetData.minHillSize / 2, lod.divisions);
                    const mesh = new THREE.Mesh(geometry, planetData.materials);
                    planetData.mesh.addLevel(mesh, (voxelData.Size/2) + lod.distance);
                });
                // planetData.geometry = magicSphereGeometry(planetData.minHillSize / 2, PlanetParams.sphereCubeDivisions);
                // planetData.mesh = new THREE.Mesh(planetData.geometry, planetData.materials);
                // // planetData.mesh.rotation.y = Math.PI/2;
                planetData.mesh.position.x = voxelData.Position.x || 0;
                planetData.mesh.position.y = voxelData.Position.y || 0;
                planetData.mesh.position.z = voxelData.Position.z || 0;
                // console.log(voxelData)
                ss.addMesh(planetData.mesh);

                if (planetData.data.water) {
                    console.log(`Creating water for ${name}`);
                    const waterData = planetData.data.water;
                    // Planet has water
                    planetData.waterGeometry = new THREE.SphereGeometry((planetData.minHillSize * waterData.level) / 2, PlanetParams.waterSphereSegments, PlanetParams.waterSphereSegments);
                    planetData.waterMaterial = new THREE.MeshPhongMaterial({
                        specular: waterData.specular,
                        shininess: waterData.shininess,
                        color: waterData.color,
                        normalScale: new THREE.Vector2(1, - 1),
                        opacity: waterData.opacity,
                        transparent: waterData.transparent,
                    });
                    planetData.waterMesh = new THREE.Mesh(planetData.waterGeometry, planetData.waterMaterial);
                    planetData.waterMesh.rotation.y = 3.14;
                    planetData.waterMesh.position.x = voxelData.Position.x || 0;
                    planetData.waterMesh.position.y = voxelData.Position.y || 0;
                    planetData.waterMesh.position.z = voxelData.Position.z || 0;
                    ss.addMesh(planetData.waterMesh);
                }


                if (voxelData.HasAtmosphere && planetData.data.sky) {
                    const sky = planetData.data.sky;
                    await Promise.all(sky.map(async (skyData) => {
                        const geometry = new THREE.SphereGeometry((voxelData.Size+2*planetData.hillDelta*skyData.altitude) / 2, PlanetParams.waterSphereSegments, PlanetParams.waterSphereSegments);
                        let map=null, alpha=null;
                        if (skyData.texture) {
                            map = await loadTexture(`img/sky/${skyData.texture}`);
                        }
                        if (skyData.alpha) {
                            alpha = await loadTexture(`img/sky/${skyData.alpha}`);
                        }
                        const material = new THREE.MeshPhongMaterial({
                            specular: 0x333333,
                            shininess: 10,
                            color: skyData.color,
                            normalScale: new THREE.Vector2(1, - 1),
                            map,
                            alphaMap: alpha,
                            transparent: true,
                            opacity: 0.6,
                            depthWrite: false,
                        });
                        const skyMesh = new THREE.Mesh(geometry , material);
                        skyMesh.position.x = voxelData.Position.x || 0;
                        skyMesh.position.y = voxelData.Position.y || 0;
                        skyMesh.position.z = voxelData.Position.z || 0;
                        skyMesh.depthWrite = false;

                        skyMesh.rotationAxis = skyData.rotationAxis.clone();

                        skyMesh.rotation.set(
                            skyData.rotationAxis.x * skyData.rotationOffset,
                            skyData.rotationAxis.y * skyData.rotationOffset,
                            skyData.rotationAxis.z * skyData.rotationOffset
                        );
                        skyMesh.rotationSpeed = skyData.rotationSpeed;
                        skyMesh.renderCall = (function(delta, context) {
                            this.rotation.x += this.rotationAxis.x * this.rotationSpeed * delta
                            this.rotation.y += this.rotationAxis.y * this.rotationSpeed * delta
                            this.rotation.z += this.rotationAxis.z * this.rotationSpeed * delta
                        }).bind(skyMesh);
                        ss.addMesh(skyMesh)
                        ss.addRenderCall(skyMesh.renderCall);
                    }));
                }

                if (planetData.data.ring) {
                    const ringData = planetData.data.ring;
                    const innerRadius = (planetData.maxHillSize * 1.5) / 2;
                    const outerRadius = (planetData.maxHillSize * ringData.size) / 2
                    const geometry = GenerateRing(innerRadius, outerRadius, PlanetParams.waterSphereSegments);
                    let map = null, alpha = null;
                    if (ringData.texture) {
                        map = await loadTexture(`img/sky/${ringData.texture}`);
                    }
                    if (ringData.alpha) {
                        alpha = await loadTexture(`img/sky/${ringData.alpha}`);
                    }
                    const material = new THREE.MeshLambertMaterial({
                        color: ringData.color,
                        normalScale: new THREE.Vector2(-1, 1),
                        map,
                        alphaMap: alpha,
                        transparent: true,
                        //opacity: 0.8,
                        depthWrite: true,
                        side: THREE.DoubleSide,
                    });
                    planetData.ring = new THREE.Mesh( geometry, material );
                    planetData.ring.position.x = voxelData.Position.x || 0;
                    planetData.ring.position.y = voxelData.Position.y || 0;
                    planetData.ring.position.z = voxelData.Position.z || 0;
                    // planetData.ring.rotation.z = Math.PI/2;
                    planetData.ring.rotation.x = Math.PI/2;
                    ss.addMesh(planetData.ring);
                }

                planetData.boundingSphere = new THREE.Mesh(
                    new THREE.SphereGeometry((planetData.maxHillSize * 1.1) / 2, PlanetParams.waterSphereSegments/4, PlanetParams.waterSphereSegments/4),
                    new THREE.MeshLambertMaterial( { color: 0xFF0000, transparent: true, opacity: 0.2, wireframe: true } )
                );
                planetData.boundingSphere.position.x = voxelData.Position.x || 0;
                planetData.boundingSphere.position.y = voxelData.Position.y || 0;
                planetData.boundingSphere.position.z = voxelData.Position.z || 0;
                planetData.boundingSphere.visible = false;
                planetData.boundingSphere.planetName = name;
                ss.addMesh(planetData.boundingSphere);

                ss.voxels[name] = planetData;
                document.dispatchEvent(new CustomEvent("new_planet", {
                    detail: name,
                }));
            }
        });
        await Promise.all(loads);
    }

    newChatCallback(data) {
        document.dispatchEvent(new CustomEvent('chatMessage', { detail: data }));
    }

    gridsCallback(data) {
        const ss = this;
        Object.keys(data).forEach((k) => {
            // console.log(data[k])
            ss.gridUpdateCallback({
                Grid: data[k],
                IsNew: true,
            });
        })
    }

    pickNewOwnerColor() {
        let found = false;
        let value;
        const ss = this;
        do {
            value = pickRandomColor();
            found = Object.keys(ss.ownerColors)
                .map(k => ss.ownerColors[k])
                .map(v => v == value)
                .reduce((a, v) => a || v, false);
        } while (found);
        return value;
    }

    gridUpdateCallback(data) {
        const gridData = data.Grid || data.Player;
        gridData.EntityId = gridData.EntityId || gridData.Id;
        if (data.IsNew) {
            // console.log(`New Grid: ${gridData.Name}, ${gridData.EntityId}`, gridData)
            if (!this.ownerColors[gridData.Faction]) {
                this.ownerColors[gridData.Faction] = this.pickNewOwnerColor();
            }
            const gridMaterial = new THREE.MeshLambertMaterial( {
                color: this.ownerColors[gridData.Faction], emissive: this.ownerColors[gridData.Faction]
            } )
            // const geometry = new THREE.SphereGeometry(1000, 8, 8);
            const geometry = new THREE.SphereGeometry(PlanetParams.gridSphereDiameterFactor, PlanetParams.gridSphereSegments, PlanetParams.gridSphereSegments);
            const gridMesh = new THREE.Mesh(geometry, gridMaterial);
            gridMesh.position.x = gridData.Position.x || 0;
            gridMesh.position.y = gridData.Position.y || 0;
            gridMesh.position.z = gridData.Position.z || 0;

            const gridText = newText(gridData.Name);
            gridText.position.x = gridData.Position.x || 0;
            gridText.position.y = gridData.Position.y || 0;
            gridText.position.z = gridData.Position.z || 0;
            gridMesh.spriteText = gridText;
            gridMesh.spriteText.visible = false;
            gridMesh.spriteText.position.multiplyScalar(PlanetParams.gridSpriteLabelRadiusOffset);
            gridMesh.name = `${gridData.Name} (${gridData.EntityId})`
            gridMesh.simpleName = gridData.Name;
            gridMesh.faction = gridData.Faction;
            gridMesh.factionTag = gridData.FactionTag || gridData.Faction || 'Player';
            gridMesh.ownerColor = '#' + this.ownerColors[gridData.Faction].toString(16).padStart(6, '0');
            const ng = {
                gridData: gridData,
                mesh: gridMesh,
                text: gridText,
            };
            this.grids[gridData.EntityId] = ng;
            this.addMesh(ng.mesh);
            this.addMesh(ng.text);
            document.dispatchEvent(new CustomEvent("newGrid", {
                detail: {
                    id: gridData.EntityId,
                    name: gridData.Name,
                    faction: gridData.Faction,
                    factionTag: gridData.FactionTag,
                    mesh: gridMesh,
                }
            }));
        } else if (data.IsDeleted) {
            const grid = this.grids[gridData.EntityId];
            if (grid) {
                this.removeMesh(grid.mesh);
                this.removeMesh(grid.text);
                delete (this.grids[gridData.EntityId]);
            }
        } else { // Update
            if (!this.grids[gridData.EntityId]) {
                // For some reason, we didnt added it.
                data.IsNew = true;
                return this.gridUpdateCallback(data);
            }
            const gridI = this.grids[gridData.EntityId];
            if (gridData.Name != gridI.gridData.Name) {
                console.log(`Grid ${gridData.EntityId} renamed from ${gridI.gridData.Name} to ${gridData.Name}`);
                document.dispatchEvent(new CustomEvent("deleteGrid", {
                    detail: {
                        id: gridData.EntityId,
                        mesh: grid.mesh,
                        text: grid.text,
                    }
                }));
                gridI.name = `${gridData.Name} (${gridData.EntityId})`
                this.removeMesh(gridI.text);
                const visibleState = gridI.text.visible;
                gridI.text = newText(gridData.Name);
                gridData.spriteText = gridI.text;
                gridI.text.visible = visibleState;
                this.addMesh(gridI.text);
            }
            const { mesh, text } = this.grids[gridData.EntityId];
            mesh.position.x = gridData.Position.x || 0;
            mesh.position.y = gridData.Position.y || 0;
            mesh.position.z = gridData.Position.z || 0;
            text.position.x = gridData.Position.x || 0;
            text.position.y = gridData.Position.y || 0;
            text.position.z = gridData.Position.z || 0;
            text.position.multiplyScalar(PlanetParams.gridSpriteLabelRadiusOffset);

            this.grids[gridData.EntityId].gridData = gridData;
        }
    }

    addRenderCall(call) {
        document.dispatchEvent(new CustomEvent("addRenderCall", {
            detail: call,
        }));
    }

    globalInfoCallback(data) {
        this.globalInfo = data;
        document.dispatchEvent(new CustomEvent("sunPosition", {
            detail: {
                x: data.SunNormalized.x || 0,
                y: data.SunNormalized.y || 0,
                z: data.SunNormalized.z || 0,
                intensity: data.SunIntensity,
            }
        }))
    }

    addMesh(mesh) {
        document.dispatchEvent(new CustomEvent("addMesh", {
            detail: mesh,
        }));
    }

    removeMesh(mesh) {
        document.dispatchEvent(new CustomEvent("deleteMesh", {
            detail: mesh,
        }));
    }

    async onMessage(msg) {
        try {
            const wsMsg = WebsocketMessage.decode(new Uint8Array(await msg.data.arrayBuffer()));
            const data = WebsocketMessage.toObject(wsMsg);
            // console.log(data);
            switch (data.Type) {
                case "planets": this.planetsUpdateCallback(data.PlanetList.Planets); break;
                case "gridUpdate": this.gridUpdateCallback(data.GridUpdate); break;
                case "playerUpdate": this.gridUpdateCallback(data.PlayerUpdate); break;
                case "chat": this.newChatCallback(data.SpaceMessage); break;
                case "grids": this.gridsCallback(data.GridList.Grids); break;
                case "players": this.gridsCallback(data.Players.Players); break;
                case "globalInfo": this.globalInfoCallback(data.GlobalInfo); break;
            }
        } catch (e) {
            document.dispatchEvent(new CustomEvent("SpaceError", {
                detail: {
                    message: 'Error parsing server data:' + e,
                    msg,
                    e,
                }
            }));
            //console.error(`Error parsing server data: ${e}`);
        }
    }
    onError(error) {
        document.dispatchEvent(new CustomEvent("SpaceError", {
            detail: {
                message: 'Websocket Error:' + error,
                error,
            }
        }));
        this.connect();
        //console.error(`Websocket Error: ${error}`);
    }
    onOpen() {
        this.conn.send('ping'); // Send the message 'Ping' to the server
        document.dispatchEvent(new CustomEvent("wsConnected", {}));
    }

    onClose() {
        document.dispatchEvent(new CustomEvent("wsDisconnected", {}));
        this.connect();
    }
}


export default SpaceSocket;
export { SpaceSocket }