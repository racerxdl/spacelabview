#!/usr/bin/env python3

import os
import json
import math
from xml.dom.minidom import parse
from PIL import Image, ImageFilter, ImageEnhance
from itertools import repeat
from multiprocessing import Pool, freeze_support

from libs.datatypes import *
from libs.mathtool import *


planetData = "assets/Ares/Planet Agaris.sbc"
planetName = "Planet Agaris"
baseAssetPath = "assets/Ares/PlanetDataFiles/Planet Agaris/"
# planetData = "assets/PlanetGeneratorDefinitions.sbc"
# planetName = "Europa"
# baseAssetPath = "assets/Europa/"
oreElementShort = {
    "Iron_01": "Fe",
    "Nickel_01": "Ni",
    "Silicon_01": "Si",
    "Magnesium_01": "Mg",
    "Cobalt_01": "Co",
    "Silver_01": "Ag",
    "Gold_01": "Au",
    "Platinum_01": "Pt",
    "Uraninite_01": "U",
    "Copper": "Cu",
    "Bauxite": "Al",
    "Coal": "C",
    "Titanium": "Ti",
    "OilSand": "Oil",
    "Sulfur": "S",
    "Lithium": "Li",
    "Tantalum": "Ta",
    "Cronyx": "Cr",
    "Dorium": "Dor"
}
oreColors = {
    "Iron_01": (255,216,0),
    "Nickel_01": (239,166,117),
    "Silicon_01": (216,107,128),
    "Magnesium_01": (0, 255, 255),
    "Cobalt_01": (181,254,0),
    "Silver_01": (145,145,145),
    "Gold_01": (255,0,220),
    "Platinum_01": (219,249,255),
    "Uraninite_01": (155,114,241),
    "Copper": (184,115,51),
    "Bauxite": (249,166,64),
    "Coal": (145,145,145),
    "Titanium": (81,127,84),
    "OilSand": (183,0,3),
    "Sulfur": (190,167,151),
    "Lithium": (172,80,141),
    "Tantalum": (255,0,0),
    "Cronyx": (184,115,51),
    "Dorium": (181,254,0)
}

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
    dom1 = parse(planetData)
    planetDefinitionsXml = dom1.getElementsByTagName("Definition")

    if len(planetDefinitionsXml) == 0:
        planetDefinitionsXml = dom1.getElementsByTagName("PlanetGeneratorDefinitions")

    #planetDefinitions = {}
    for planetDefinition in planetDefinitionsXml:
        pd = PlanetDefinition.from_xml_element(planetDefinition)
        print(pd.Name)
        if pd.Name == planetName:
            pd.cache(matfiles, matcoloravg)
            planetDefinitions[pd.Name] = pd

    with open("./luts/matcolormap.json", "w") as f:
        f.write(MyEncoder().encode(planetDefinitions))

    currentPlanet = planetDefinitions[planetName]
    print(currentPlanet.ComplexMaterials)

    for p in cubemap:
        hmPath = os.path.join(baseAssetPath, f"{p}.png")
        imPath = os.path.join(baseAssetPath, f"{p}_mat.png")
        latLutPath = f"./luts/latlut_{p}.png"

        im = Image.open(imPath).convert('RGB')
        hm = Image.open(hmPath)
        hm = hm.point(lambda i: i/256)
        hm = hm.convert('RGB')

        width, height = im.size

        if not os.path.exists(latLutPath):
            print("Computing LUT")
            latlutimg = Image.new(mode="RGB", size=im.size)
            for y in range(0, height):
                if y % 100 == 0:
                    print(f"Computing y={y}")
                with Pool(processes=20) as pool:
                    for x, _, lat in pool.starmap(compute_lut, zip(range(0, width), repeat(y), repeat(width), repeat(height), repeat(p))):
                        lat = abs(int(lat))
                        latlutimg.putpixel((x,y), (lat, lat, lat))
            latlutimg.save(latLutPath)
            print("Done!")
        else:
            print(f"Cached LUT at {latLutPath}")
            latlutimg = Image.open(latLutPath)


        # Pre-compute slope
        # make a plane with x0,y0,x0+1,y0+1 and calculate angle in relation to plane xy
        # the "fast" way to do this is to calculate the angle of the (x0,y0) -> (x1,y1) line
        slopePath = os.path.join(baseAssetPath, f"{p}_slope.png")
        if not os.path.exists(slopePath):
            sm = Image.new(mode="RGB", size=im.size)
            for y0 in range(0, height-1):
                for x0 in range(0, width-1):
                    x1 = x0+1
                    y1 = y0+1

                    x = x1 - x0
                    y = y1 - y0

                    if x1 > width:
                        x1 -= width
                    if y1 > height:
                        y1 -= height

                    z0, _, _ = hm.getpixel((x0, y0))
                    z1, _, _ = hm.getpixel((x1, y1))
                    z = z1 - z0
                    a = int(
                        abs(math.asin(z / math.sqrt(x*x + y*y + z*z)) * rad2deg))
                    sm.putpixel((x0, y0), (a, a, a))
            #for i in range(3):
            #    sm = sm.filter(ImageFilter.GaussianBlur(radius=2))
            #    sm = ImageEnhance.Sharpness(sm).enhance(8)
            sm.save(slopePath)
        else:
            print(f"Cached slope at {slopePath}")
            sm = Image.open(slopePath).convert('RGB')

        tex = Image.new(mode="RGB", size=im.size)
        #r, _,  _ = im.split()  # Red Channel contains biome

        for y in range(0, height):
            if y % 100 == 0:
                print(f"Line {y} from {height}")
            for x in range(0, width):
                v, _, ore = im.getpixel((x, y))
                if ore == 0 and not ore in pd.Ores:
                    h, _, _ = hm.getpixel((x, y))
                    s, _, _ = sm.getpixel((x, y))
                    h /= 255.0
                    lat,_,_ = latlutimg.getpixel((x,y))
                    color = currentPlanet.get_color(v, h, lat, s)
                    tex.putpixel((x, y), color)
                else:
                    ore = pd.Ores[ore]
                    if oreColors[ore.Type]:
                        tex.putpixel((x, y), oreColors[ore.Type])
                    else:
                        tex.putpixel((x, y), ore.TargetColor)


        #idsNotFound = ",".join(idsNotFound)
        # print(f"IDs not found: {idsNotFound}")
        #tex = tex.filter(ImageFilter.GaussianBlur(radius=2))
        #tex = ImageEnhance.Sharpness(tex).enhance(8)
        tex.save(f"{p}.jpg")
        break
