/*
    Planet Mesh Generation Params
*/


const Params = {
    // Planet Mesh
    sphereCubeDivisions: 256, // Number of divisions on Cube Spheres used for Planet Mesh
    waterSphereSegments: 64,
    // Grid Mesh
    gridSphereDiameterFactor: 100,
    gridSphereSegments: 8,
    gridSphereSpecularColor: 0x333333,
    gridSphereSpecularShininess: 0x333333,
    // Viewport
    viewportNear: 10000,
    viewportFar: 8500000,
    viewportFov: 25,
    viewportZPos: 100e3 * 5,
    viewportMargin: 0,
    // Camera
    cameraMinDistance: 0,
    cameraMaxDistance: 200e6 * 20,
    // Scene
    sceneFogColor: 0x000000,
    sceneFogDensity: 0.00000025,
    sceneAmbientLight: 0x443333,
    sceneSunColor: 0xffffff,
    // Renderer:
    rendererAntialias: true,
}

export default Params;