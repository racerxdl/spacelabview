/*
    Planet Mesh Generation Params
*/


const Params = {
    WebsocketURL: "wss://spacelab.lucasteske.dev/ws",
    // Planet Mesh
    sphereCubeDivisions: 256, // Number of divisions on Cube Spheres used for Planet Mesh
    waterSphereSegments: 64,
    planetLOD: [
        { distance:   25000, divisions: 512 },
        { distance:   50000, divisions: 256 },
        { distance:  100000, divisions: 128 },
        { distance:  200000, divisions: 64 },
        { distance:  400000, divisions: 32 },
        { distance:  800000, divisions: 16 },
        { distance: 1600000, divisions: 8 },
        { distance: 3200000, divisions: 4 },
    ],
    // Grid Mesh
    gridSphereDiameterFactor: 100,
    gridSphereSegments: 8,
    gridSphereSpecularColor: 0x333333,
    gridSphereSpecularShininess: 0x333333,
    gridSpriteLabelRadiusOffset: 1.02,
    // Viewport
    viewportNear: 1000,
    viewportFar: 8000000,
    viewportFov: 70,
    viewportZPos: 100e3 * 5,
    viewportDistanceFactor: 2,
    viewportMargin: 0,
    // Camera
    cameraMinDistance: 1000,
    cameraMaxDistance: 4000000,
    cameraSpeed: 100,
    dollySpeed: {
        slow: 0.01,
        normal: 0.1,
        fast: 1,
    },
    // Scene
    sceneFogColor: 0x000000,
    sceneFogDensity: 0.00000015,
    sceneAmbientLight: 0x444444,
    sceneSunColor: 0xfff0f0,
    // Renderer:
    rendererAntialias: true,
}

export default Params;