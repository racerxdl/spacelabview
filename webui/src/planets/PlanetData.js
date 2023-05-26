/*
    Since some stuff is hard to get in-game, here we hardcoded few stuff.
*/

import * as THREE from 'three';

const planets = {
    "planet crait": {
        name: "Planet Crait",
        pathPrefix: "planet_crait",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "agaris ii": {
        name: "Planet Agaris II",
        pathPrefix: "agaris2",
        isVanilla: false,
        sky: []
    },
    "bylen": {
        name: "Planet Bylen",
        color: 0x222222,
        isVanilla: false,
        pathPrefix: "bylen",
        gasPlanet: true,
        ring: {
            color: 0xFFFFFF,
            alpha: 'bylenring_alpha.jpg',
            texture: 'bylenring.jpg',
            rotation: new THREE.Vector3(0, 0, 0),
            size: 2.2,
        },
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 3.2,
                altitude: 1,
                texture: 'bylenclouds.jpg',
            },
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0002,
                rotationAxis: new THREE.Vector3(1, 0.2, -0.2),
                rotationOffset: 2.5,
                altitude: 2,
                texture: 'bylenclouds.jpg',
            },
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0002,
                rotationAxis: new THREE.Vector3(-0.2, 0.2, 1),
                rotationOffset: 4.2,
                altitude: 3,
                texture: 'bylenclouds.jpg',
            },
            // {
            //     ring: true,
            //     color: 0xFFFFFF,
            //     rotationSpeed: 0.0002,
            //     rotationAxis: new THREE.Vector3(-0.2, 0.2, 1),
            //     rotationOffset: 4.2,
            //     altitude: 1,
            //     alpha: 'bylenring_alpha.jpg',
            //     texture: 'bylenring.jpg',
            // },
        ]
    },
    "planet lezuno": {
        name: "Planet Lezuno",
        pathPrefix: "planet_lezuno",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "planet thora iv": {
        name: "Planet Thora IV",
        pathPrefix: "planet_thora_4",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "planet thora 4": {
        name: "Planet Thora 4",
        pathPrefix: "planet_thora_4",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "planet lorus": {
        name: "Planet Lorus",
        pathPrefix: "planet_lorus",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "planet agaris": {
        name: "Planet Agaris",
        water: {
            level: 1.013,
            color: 0x1c526a,
            specular: 0x555555,
            shininess: 25,
            opacity: 0.8,
            transparent: true,
        },
        pathPrefix: "agaris",
        isVanilla: false,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "earthlike": {
        name: "Earth Like",
        pathPrefix: "earthlike",
        isVanilla: true,
        sky: [
            {
                color: 0xdddddd,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(-0.2, 1, 0.2),
                rotationOffset: 1.2,
                altitude: 0.9,
                alpha: 'earthlike.jpg',
            },
        ]
    },
    "mars": {
        name: "Mars",
        pathPrefix: "mars",
        isVanilla: true,
        sky: [
            {
                color: 0xf0ba7a,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 1.2,
                altitude: 1,
                alpha: 'mars.jpg',
            },
        ]
    },
    "europa": {
        name: "Europa",
        pathPrefix: "europa",
        isVanilla: true,
    },
    "alien": {
        name: "Alien",
        pathPrefix: "alien",
        isVanilla: true,
        atmosphere: {
            color: 0xcfdb6b,
            specular: 0x555555,
            shininess: 25,
            opacity: 0.1,
            transparent: true,
            rotationOffset: 1.2,
        },
        sky: [
            {
                color: 0xcfdb6b,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(-0.2, 1, 0.2),
                rotationOffset: 1.2,
                altitude: 1,
                alpha: 'alien.jpg',
            },
        ]
    },
    "triton": {
        name: "Triton",
        pathPrefix: "triton",
        isVanilla: true,
        sky: [
            {
                color: 0xffffff,
                rotationSpeed: 0.000373112,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 1,
                altitude: 0.15,
                alpha: 'cloud.jpg',
            },
            {
                color: 0xffffff,
                rotationSpeed: -0.00026112,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 5,
                altitude: 0.21,
                alpha: 'cloud.jpg',
            },
            {
                color: 0xffffff,
                rotationSpeed: 0.0004812112,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 3,
                altitude: 0.51,
                alpha: 'cloud.jpg',
            },
            {
                color: 0xffffff,
                rotationSpeed: -0.000396112,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 7,
                altitude: 0.57,
                alpha: 'cloud.jpg',
            },
            {
                color: 0xffffff,
                rotationSpeed: 0.000286112,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: -3,
                altitude: 0.63,
                alpha: 'cloud.jpg',
            },
            {
                color: 0xffffff,
                rotationSpeed: 0.15,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: -3,
                altitude: 1,
                alpha: 'cloud.jpg',
            },
        ]
    },
    "titan": {
        name: "Titan",
        pathPrefix: "titan",
        isVanilla: true,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.000085,
                rotationAxis: new THREE.Vector3(1, 0.2, -0.2),
                rotationOffset: 2.5,
                altitude: 1,
                alpha: 'titan_alpha.jpg',
                texture: 'titan_tex.jpg',
            },
        ]
    },
    "pertam": {
        name: "Pertam",
        pathPrefix: "pertam",
        isVanilla: true,
        sky: [
            {
                color: 0xFFFFFF,
                rotationSpeed: 0.0006,
                rotationAxis: new THREE.Vector3(0, 1, 0),
                rotationOffset: 0.15,
                altitude: 1,
                alpha: 'cloud.jpg',
            },
        ]
    },
    "moon": {
        name: "Moon",
        pathPrefix: "moon",
        isVanilla: true,
    },
};


export default planets;