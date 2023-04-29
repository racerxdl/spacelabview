/*
    Preloads required files
*/

import { magicSphereGeometry } from "../planets/MagicSphereGeometry";
import Params from "../planets/Params";

const planets = [
    // Vanilla
    "alien", "earthlike", "europa", "mars", "moon", "pertam", "titan", "triton",
    // Ares At War
    "agaris", "agaris2", "bylen", "planet_crait", "planet_lezuno", "planet_lorus", "planet_thora_4"
]
const cubeMapFaces = ["front", "right", "up", "down", "back", "left"];

const imagesToLoad = [
    "img/BackgroundCube-0.jpg",
    "img/BackgroundCube-1.jpg",
    "img/BackgroundCube-2.jpg",
    "img/BackgroundCube-3.jpg",
    "img/BackgroundCube-4.jpg",
    "img/BackgroundCube-5.jpg",
    "img/cloud.png",
    "img/sky/alien.jpg",
    "img/sky/cloud.jpg",
    "img/sky/earthlike.jpg",
    "img/sky/mars.jpg",
    "img/sky/titan_alpha.jpg",
    "img/sky/titan_tex.jpg",
    "img/sky/alien.jpg",
    "img/sky/alien.jpg",
]

async function preloadTexture(texturePath) {
    const img = new Image();
    img.src = texturePath;
    return new Promise((resolve, reject) => {
        img.onload = () => {
            window.imagesLoaded++;
            document.dispatchEvent(new Event("item_loaded"));
            resolve(img.src);
        }
    })
}

planets.forEach((planet) => {
    cubeMapFaces.forEach((face) => {
        imagesToLoad.push(`img/planets-cube/${planet}/${face}.jpg`)
        imagesToLoad.push(`img/planets-cube/${planet}/hm/${face}.jpg`)
    })
})

const chunkSize = 10;

function preloadAll() {
    window.imagesToLoadCount = imagesToLoad.length;
    window.imagesLoaded = 0;

    return new Promise(async (resolve) => {
        // Chunks so the browser doesn't freeze before loading all images
        const chunks = Math.round(imagesToLoad.length / chunkSize);
        for (let i = 0; i < chunks; i++) {
            const chunk = imagesToLoad.slice(i * chunkSize, (i + 1) * chunkSize);
            console.log(`Loading ${i * chunkSize} -> ${(i + 1) * chunkSize} images...`);
            await Promise.all(chunk.map((img) => preloadTexture(img)));
        }
        resolve();

    });
    // const promises = Promise.all(imagesToLoad.map((img) => preloadTexture(img)))
}

function preGenerate() {
    magicSphereGeometry(100, Params.gridSphereSegments);
    Params.planetLOD.forEach((lod) => {
        magicSphereGeometry(100, lod.divisions);
    });
    // magicSphereGeometry(100, Params.sphereCubeDivisions*2);
    // magicSphereGeometry(100, Params.sphereCubeDivisions);
    // magicSphereGeometry(100, Params.sphereCubeDivisions/2); // LOD
    // magicSphereGeometry(100, Params.sphereCubeDivisions/4); // LOD
    // magicSphereGeometry(100, Params.sphereCubeDivisions/8); // LOD
    // magicSphereGeometry(100, Params.sphereCubeDivisions/16); // LOD
    magicSphereGeometry(100, Params.waterSphereSegments);
}

export {
    preloadAll,
    preGenerate,
}