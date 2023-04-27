/*
    SpaceLab Websocket Handler
*/
import * as THREE from 'three';

import PlanetData from "../planets/PlanetData";
import {
    preloadPlanetTexture,
    planetHeightMaps,
    planetTextures,
} from "../loaders/TexturePreloader"
import PlanetParams from "../planets/Params";
import { magicSphereGeometry } from "../planets/MagicSphereGeometry"
import { pickRandomColor } from '../colors';
import { newText } from '../draw';

class SpaceSocket {

    constructor(socketUrl) {
        this.socketUrl = socketUrl;
        this.conn = new WebSocket(socketUrl);
        this.conn.onopen = this.onOpen.bind(this);
        this.conn.onmessage = this.onMessage.bind(this);
        this.conn.onerror = this.onError.bind(this);
        this.grids = {};
        this.voxels = {};
        this.textureLoader = new THREE.TextureLoader();
        this.ownerColors = {};
    }

    getPlanet(name) {
        return this.voxels[name];
    }

    planetsUpdateCallback(data) {
        const ss = this;

        Object.keys(data).forEach((name) => {
            const voxelData = data[name];
            if (name != "" && voxelData.DebugName.indexOf("MyPlanet") > 0) {
                const planetNameInfo = name.split("-");
                const instanceName = planetNameInfo[0];
                const basePlanet = planetNameInfo[1];
                const generatorParams = planetNameInfo.length == 4 ? planetNameInfo[3] : planetNameInfo[2];
                console.log(`Detected planet ${instanceName} (${name})`);
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
                preloadPlanetTexture(planetData.data.pathPrefix);
                planetData.heightMapTextures = planetHeightMaps(planetData.data.pathPrefix);
                planetData.textures = planetTextures(planetData.data.pathPrefix);

                planetData.maxHillSize = (1 + voxelData.HillParameters.Item2) * voxelData.Size;
                planetData.minHillSize = (1 + voxelData.HillParameters.Item1) * voxelData.Size;
                planetData.hillDelta = (planetData.maxHillSize - planetData.minHillSize) / 2;

                planetData.materials = planetData.textures.map((texture, idx) => {
                    const heightMapTexture = planetData.heightMapTextures[idx];
                    const hmTex = ss.textureLoader.load(heightMapTexture);
                    return new THREE.MeshPhongMaterial({
                        specular: 0x333333,
                        shininess: 5,
                        map: ss.textureLoader.load(texture),
                        displacementMap: hmTex,
                        displacementScale: planetData.hillDelta,
                        normalScale: new THREE.Vector2(1, - 1),
                        bumpMap: hmTex,
                        bumpScale: planetData.hillDelta / 2,
                    });
                });
                planetData.geometry = magicSphereGeometry(planetData.minHillSize / 2, PlanetParams.sphereCubeDivisions);
                planetData.mesh = new THREE.Mesh(planetData.geometry, planetData.materials);
                // planetData.mesh.rotation.y = Math.PI/2;
                planetData.mesh.position.x = voxelData.X;
                planetData.mesh.position.y = voxelData.Y;
                planetData.mesh.position.z = voxelData.Z;
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
                    planetData.waterMesh.position.x = voxelData.X;
                    planetData.waterMesh.position.y = voxelData.Y;
                    planetData.waterMesh.position.z = voxelData.Z;
                    ss.addMesh(planetData.waterMesh);
                }


                if (voxelData.HasAtmosphere && planetData.data.sky) {
                    const sky = planetData.data.sky;
                    sky.forEach((skyData) => {
                        const geometry = new THREE.SphereGeometry((voxelData.Size+2*planetData.hillDelta*skyData.altitude) / 2, PlanetParams.waterSphereSegments, PlanetParams.waterSphereSegments);
                        let map=null, alpha=null;
                        if (skyData.texture) {
                            map = ss.textureLoader.load(`img/sky/${skyData.texture}`);
                        }
                        if (skyData.alpha) {
                            alpha = ss.textureLoader.load(`img/sky/${skyData.alpha}`);
                        }
                        const material = new THREE.MeshPhongMaterial({
                            specular: 0x333333,
                            shininess: 10,
                            color: skyData.color,
                            normalScale: new THREE.Vector2(1, - 1),
                            map,
                            alphaMap: alpha,
                            transparent: true,
                            opacity: 0.5
                        });
                        const skyMesh = new THREE.Mesh(geometry , material);
                        skyMesh.position.x = voxelData.X;
                        skyMesh.position.y = voxelData.Y;
                        skyMesh.position.z = voxelData.Z;

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
                    })
                }

                planetData.boundingSphere = new THREE.Mesh(
                    new THREE.SphereGeometry((planetData.minHillSize * 1.5) / 2, PlanetParams.waterSphereSegments/4, PlanetParams.waterSphereSegments/4),
                    new THREE.MeshLambertMaterial( { color: 0xFF0000, transparent: true, opacity: 0.2, wireframe: true } )
                );
                planetData.boundingSphere.position.x = voxelData.X;
                planetData.boundingSphere.position.y = voxelData.Y;
                planetData.boundingSphere.position.z = voxelData.Z;
                planetData.boundingSphere.visible = false;
                planetData.boundingSphere.planetName = name;
                ss.addMesh(planetData.boundingSphere);

                ss.voxels[name] = planetData;
                document.dispatchEvent(new CustomEvent("new_planet", {
                    detail: name,
                }));
            }
        });
    }

    newChatCallback(data) {
        document.dispatchEvent(new CustomEvent('chatMessage', { detail: data }));
    }

    gridsCallback(data) {
        const ss = this;
        Object.keys(data).forEach((k) => {
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
        const gridData = data.Grid;
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
            gridMesh.position.x = gridData.X;
            gridMesh.position.y = gridData.Y;
            gridMesh.position.z = gridData.Z;

            const gridText = newText(gridData.Name);
            gridText.position.x = gridData.X;
            gridText.position.y = gridData.Y;
            gridText.position.z = gridData.Z;

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
                this.removeMesh(gridI.text);
                gridI.text = newText(gridData.Name);
                this.addMesh(gridI.text);
            }
            const { mesh, text } = this.grids[gridData.EntityId];
            mesh.position.x = gridData.X;
            mesh.position.y = gridData.Y;
            mesh.position.z = gridData.Z;
            text.position.x = gridData.X;
            text.position.y = gridData.Y;
            text.position.z = gridData.Z;
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
                x: data.SunNormalizedX,
                y: data.SunNormalizedY,
                z: data.SunNormalizedZ,
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

    onMessage(msg) {
        try {
            const data = JSON.parse(msg.data);
            switch (data.Type) {
                case "planets": this.planetsUpdateCallback(data.Content); break;
                case "gridUpdate": this.gridUpdateCallback(data.Content); break;
                case "chat": this.newChatCallback(data.Content); break;
                case "grids": this.gridsCallback(data.Content); break;
                case "globalInfo": this.globalInfoCallback(data.Content); break;
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
        //console.error(`Websocket Error: ${error}`);
    }
    onOpen() {
        this.conn.send('ping'); // Send the message 'Ping' to the server
        document.dispatchEvent(new CustomEvent("wsConnected", {}));
    }
}


export default SpaceSocket;
export { SpaceSocket }