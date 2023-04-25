/*
    Preloads required files
*/

import { magicSphereGeometry } from "../planets/MagicSphereGeometry";
import Params from "../planets/Params";

const planets = [
    "agaris", "alien", "earthlike", "europa", "mars", "moon", "pertam", "titan", "triton"
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

function preloadAll() {
    window.imagesToLoadCount = imagesToLoad.length;
    window.imagesLoaded = 0;
    const promises = Promise.all(imagesToLoad.map((img) => preloadTexture(img)))

    return promises;
}

function preGenerate() {
    magicSphereGeometry(100, Params.gridSphereSegments);
    magicSphereGeometry(100, Params.sphereCubeDivisions);
    magicSphereGeometry(100, Params.sphereCubeDivisions/2); // LOD
    magicSphereGeometry(100, Params.sphereCubeDivisions/4); // LOD
    magicSphereGeometry(100, Params.waterSphereSegments);
}

export {
    preloadAll,
    preGenerate,
}