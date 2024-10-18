#!/usr/bin/env python3

import os
import json
import math
import numpy as np
import cv2
import matplotlib.pyplot as plt
from xml.dom.minidom import parse
from PIL import Image, ImageFilter, ImageEnhance
from itertools import repeat
from multiprocessing import Pool, freeze_support
from scipy.ndimage import label, center_of_mass

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
        "planetData": "assets/Ares/Planet Crait.sbc",
        "planetName": "Planet Crait",
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
        "planetData": "assets/Ares/Planet Lorus.sbc",
        "planetName": "Planet Lorus",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Lorus/"
    },
    {
        "planetData": "assets/Ares/Planet Thora 4.sbc",
        "planetName": "Planet Thora 4",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Planet Thora 4/"
    },
    {
        "planetData": "assets/Ares/PlanetGeneratorDefinitions.sbc",
        "planetName": "Agaris II",
        "baseAssetPath": "assets/Ares/PlanetDataFiles/Agaris II/"
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
        "planetData": "assets/Vanilla/Pertam.sbc",
        "planetName": "Pertam",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Pertam/"
    },
    {
        "planetData": "assets/Vanilla/PlanetGeneratorDefinitions.sbc",
        "planetName": "Titan",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Titan/"
    },
    {
        "planetData": "assets/Vanilla/Triton.sbc",
        "planetName": "Triton",
        "baseAssetPath": "assets/Vanilla/PlanetDataFiles/Triton/"
    }
]

def get_ore_sites(imgfile):
    img = Image.open(imgfile)
    img = img.convert("RGB")

    # Convert image to numpy array
    img_array = np.array(img)

    # Extract the blue channel
    blue_channel = img_array[:, :, 2]

    # Create a mask of ore sites (where blue_channel > 0)
    ore_mask = (blue_channel > 0) & (blue_channel < 255)
    ore_mask = ore_mask.astype(np.uint8)
    num_labels, labels, stats, centroids = cv2.connectedComponentsWithStats(ore_mask, connectivity=8)
    # Initialize an array to store ore types
    ore_types = np.zeros(num_labels - 1, dtype=int)  # Exclude background label 0

    # Flatten the labels and blue_channel arrays for efficient processing
    labels_flat = labels.flatten()
    blue_channel_flat = blue_channel.flatten()

    # For each label, find the most frequent blue value (ore type)
    for label in range(1, num_labels):  # Skip label 0 (background)
        # Create a mask for the current label
        mask = labels_flat == label

        # Extract blue channel values corresponding to the current label
        blue_values = blue_channel_flat[mask]

        # Find the most frequent blue value (ore type) in the component
        if blue_values.size > 0:
            ore_type = np.bincount(blue_values).argmax()
            ore_types[label - 1] = ore_type
        else:
            ore_types[label - 1] = 0  # Default or handle as needed

    sites = []

    for centroid, ore_type in zip(centroids[1:], ore_types):
        #print(f"Ore site at (x: {centroid[0]:.2f}, y: {centroid[1]:.2f}), ore type: {ore_type}")
        sites.append((centroid[0], centroid[1], ore_type))

    #plt.imshow(img)
    #plt.scatter(centroids[1:, 0], centroids[1:, 1], c='gold', marker='x')
    #plt.title('Ore Sites with Centroids')
    #plt.show()

    return sites

if __name__ == "__main__":
    freeze_support()

    for planet in planets:
        planetData = planet["planetData"]
        planetName = planet["planetName"]
        baseAssetPath = planet["baseAssetPath"]

        planetMeta = {}
        planetMeta["name"] = planetName
        planetMeta["ores"] = {}
        planetMeta["declared_ores"] = {}

        print (f"Processing {planetName}...")

        dom1 = parse(planetData)
        planetDefinitionsXml = dom1.getElementsByTagName("Definition")

        if len(planetDefinitionsXml) == 0:
            planetDefinitionsXml = dom1.getElementsByTagName("PlanetGeneratorDefinitions")

        #planetDefinitions = {}
        for planetDefinition in planetDefinitionsXml:
            pd = PlanetDefinition.from_xml_element(planetDefinition)
            #print(pd.Name)
            if pd.Name == planetName:
                planetMeta["declared_ores"] = pd.Ores

        found_ores = {}
        planetMeta["oresites"] = []

        # Open Textures and check ores
        for root, dirs, files in os.walk(baseAssetPath):
            for file in files:
                #if file.endswith("_mat.png"):
                if file.endswith("_mat.png"):
                    print(f"    Processing {file}...")
                    oresites = get_ore_sites(os.path.join(root, file))
                    for site in oresites:
                        x, y, ore = site
                        ore = int(ore)
                        planetMeta["oresites"].append([x, y, ore])
                        if ore in planetMeta["declared_ores"]:
                            if ore in found_ores:
                                found_ores[ore]["sites"] += 1
                            else:
                                found_ores[ore] = planetMeta["declared_ores"][ore].__dict__
                                found_ores[ore]["sites"] = 1
                                print(f"        Found {found_ores[ore]['Type']}")

        planetMeta["ores"] = found_ores

        #print(planetMeta)

        # Save to name_planetmeta.json
        filename = planetName.replace(" ", "_").lower()
        filename = f"planetmeta/{filename}_planetmeta.json"
        with open(filename, "w") as f:
            json.dump(planetMeta, f, indent=4, default=lambda o: o.__dict__)