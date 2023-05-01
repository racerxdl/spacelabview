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

    return cachedTextures[texturePath];
}

export {
    loadTexture,
}
