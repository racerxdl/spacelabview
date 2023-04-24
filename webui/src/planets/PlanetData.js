/*
    Since some stuff is hard to get in-game, here we hardcoded few stuff.
*/

const planets = {
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
    },
    "earthlike": {
        name: "Earth Like",
        pathPrefix: "earthlike",
        isVanilla: true,
    },
    "mars": {
        name: "Mars",
        pathPrefix: "mars",
        isVanilla: true,
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
    },
    "triton": {
        name: "Triton",
        pathPrefix: "triton",
        isVanilla: true,
    },
    "titan": {
        name: "Titan",
        pathPrefix: "titan",
        isVanilla: true,
    },
    "pertam": {
        name: "Pertam",
        pathPrefix: "pertam",
        isVanilla: true,
    },
    "moon": {
        name: "Moon",
        pathPrefix: "moon",
        isVanilla: true,
    },
};


export default planets;