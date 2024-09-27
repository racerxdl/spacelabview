import os
import json
import math
from xml.dom.minidom import parse
from PIL import Image, ImageFilter, ImageEnhance
from itertools import repeat
from multiprocessing import Pool, freeze_support

from libs.datatypes import *
from libs.mathtool import *

planets = [
    # Ares at War
    {
        "planetData": "assets/Ares/Planet Agaris.sbc",
        "planetName": "Planet Agaris",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Agaris/"
    },
    {
        "planetData": "assets/Ares/PlanetBylen.sbc",
        "planetName": "PlanetBylen",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/PlanetBylen/"
    },
    {
        "planetData": "assets/Ares/PlanetBylen - Lava.sbc",
        "planetName": "PlanetBylen - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Agaris/"
    },
    {
        "planetData": "assets/Ares/Planet Crait.sbc",
        "planetName": "Planet Crait",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Crait/"
    },
    {
        "planetData": "assets/Ares/Planet Crait - Lava.sbc",
        "planetName": "Planet Crait - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Crait/"
    },
    {
        "planetData": "assets/Ares/Planet Kamino.sbc",
        "planetName": "Planet Kamino",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Kamino/"
    },
    {
        "planetData": "assets/Ares/Planet Lezuno.sbc",
        "planetName": "Planet Lezuno",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Lezuno/"
    },
    {
        "planetData": "assets/Ares/Planet Lezuno - Lava.sbc",
        "planetName": "Planet Lezuno - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Lezuno/"
    },
    {
        "planetData": "assets/Ares/Planet Lorus.sbc",
        "planetName": "Planet Lorus",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Lorus/"
    },
    {
        "planetData": "assets/Ares/Planet Lorus - Lava.sbc",
        "planetName": "Planet Lorus - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Lorus/"
    },
    {
        "planetData": "assets/Ares/Planet Thora 4.sbc",
        "planetName": "Planet Thora 4",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Thora 4/"
    },
    {
        "planetData": "assets/Ares/Planet Thora 4 - Lava.sbc",
        "planetName": "Planet Thora 4 - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Thora 4/"
    },
    # Vanilla
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Alien",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Alien/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "EarthLike",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/EarthLike/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Europa",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Europa/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Mars",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Mars/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Moon",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Moon/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Pertam",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Pertam/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Titan",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Titan/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Triton",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Triton/"
    }
]

if __name__ == "__main__":
    freeze_support()
    # Load material files and color averages

    with open("./luts/matfiles.json") as f:
        matfiles = json.loads(f.read())

    with open("./luts/matcoloravg.json") as f:
        matcoloravg = json.loads(f.read())

    with open("./luts/matcolormap.json") as f:
        planetDefinitions = json.loads(f.read())

    # Build Material ID Map from planet SBC data
    materialIdMap = {}
    for planet in planets:
        planetData = planet["planetData"]
        planetName = planet["planetName"]
        baseAssetPath = planet["baseAssetPath"]

        print(f"Processing {planetName}")

        dom1 = parse(planetData)
        planetDefinitionsXml = dom1.getElementsByTagName("Definition")

        if len(planetDefinitionsXml) == 0:
            planetDefinitionsXml = dom1.getElementsByTagName("PlanetGeneratorDefinitions")

        #planetDefinitions = {}
        for planetDefinition in planetDefinitionsXml:
            pd = PlanetDefinition.from_xml_element(planetDefinition)
            #print(pd.Name)
            if pd.Name == planetName:
                pd.cache(matfiles, matcoloravg)
                planetDefinitions[pd.Name] = pd
            pd.BaseFolder = baseAssetPath

    with open("./luts/matcolormap.json", "w") as f:
        f.write(MyEncoder().encode(planetDefinitions))