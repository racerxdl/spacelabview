/*
    Preloads textures so browser can cache it
*/
import * as THREE from 'three';

const cachedTextures = {};
const loader = new THREE.TextureLoader();

async function loadTexture(texturePath) {
    if (cachedTextures[texturePath]) {
        return cachedTextures[texturePath];
    }
    cachedTextures[texturePath] = loader.load(texturePath);
    cachedTextures[texturePath].anisotropy = 8;

    return cachedTextures[texturePath];
}

export {
    loadTexture,
}
