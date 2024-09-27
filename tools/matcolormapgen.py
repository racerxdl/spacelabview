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
    {
        "planetData": "assets/Ares/Planet Agaris.sbc",
        "planetName": "Planet Agaris",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Agaris/"
    },
    {
        "planetData": "assets/Ares/Planet Agaris - Lava.sbc",
        "planetName": "Planet Agaris - Lava",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Agaris/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Alien",
        "baseAssetPath": "assets/Alien/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "EarthLike",
        "baseAssetPath": "assets/EarthLike/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Europa",
        "baseAssetPath": "assets/Europa/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Mars",
        "baseAssetPath": "assets/Mars/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Moon",
        "baseAssetPath": "assets/Moon/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Pertam",
        "baseAssetPath": "assets/Pertam/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Titan",
        "baseAssetPath": "assets/Titan/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Triton",
        "baseAssetPath": "assets/Triton/"
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

    with open("./luts/matcolormap.json", "w") as f:
        f.write(MyEncoder().encode(planetDefinitions))