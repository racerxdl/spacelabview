/*
    Planet Mesh Generation Params
*/


const Params = {
    // WebsocketURL: "wss://spacelab.lucasteske.dev/ws",
    WebsocketURL: "ws://localhost:3000/ws",
    // Planet Mesh
    sphereCubeDivisions: 256, // Number of divisions on Cube Spheres used for Planet Mesh
    waterSphereSegments: 64,
    planetLOD: [
        { distance:     500, divisions: 512 },
        { distance:    2000, divisions: 256 },
        { distance:    4000, divisions: 128 },
        { distance:   16000, divisions: 64 },
        { distance:   64000, divisions: 32 },
        { distance:  256000, divisions: 16 },
        { distance: 1024000, divisions: 8 },
        { distance: 4096000, divisions: 4 },
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