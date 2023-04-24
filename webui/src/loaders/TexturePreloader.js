/*
    Preloads textures so browser can cache it
*/

window.itemsToLoad = 0;
window.itemsLoaded = 0;

function preloadTexture(texturePath) {
    const img = new Image();
    window.itemsToLoad++;
    img.src = texturePath;
    img.onload = () => {
        window.itemsLoaded++;
        document.dispatchEvent(new Event("item_loaded"));
        // console.log(`Image ${texturePath} loaded...`);
    }
}

const cubeMapFaces = ["front", "right", "up", "down", "back", "left"];

function planetHeightMaps(planetPrefix) {
    return cubeMapFaces.map((face) => `img/planets-cube/${planetPrefix}/hm/${face}.jpg`);
}
function planetTextures(planetPrefix) {
    return cubeMapFaces.map((face) => `img/planets-cube/${planetPrefix}/${face}.jpg`);
}

function preloadPlanetTexture(planetPrefix) {
    // Preload textures
    planetTextures(planetPrefix).map(preloadTexture)
    // Preload heightmaps
    planetHeightMaps(planetPrefix).map(preloadTexture)
}


export {
    preloadTexture,
    preloadPlanetTexture,
    planetHeightMaps,
    planetTextures,
}
