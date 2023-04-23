#!/usr/bin/env python3

import os
import json
import math
import vectormath as vmath
from xml.dom.minidom import parse, parseString
from json import JSONEncoder
from PIL import Image, ImageFilter, ImageEnhance
import multiprocessing
from itertools import repeat
from multiprocessing import Pool, freeze_support
from numba import jit

class MyEncoder(JSONEncoder):
    def __init__(self,  *args, **kwargs):
        super(MyEncoder, self).__init__(*args, **kwargs)
        self.indent = 3

    def default(self, o):
        return o.__dict__


class MaterialLayer:
    Material = str
    Depth = int
    R, G, B = int, int, int

    @classmethod
    def from_dom_element(cls, element):
        layer = MaterialLayer()
        layer.R, layer.G, layer.B = -1, -1, -1
        layer.Material = element.attributes["Material"].nodeValue
        layer.Depth = 0
        if "Depth" in element.attributes:
            layer.Depth = int(element.attributes["Depth"].nodeValue)

        return layer

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)


class MaterialRule:
    Layers = [MaterialLayer]
    MinHeight = float
    MaxHeight = float
    LatitudeMin = float
    LatitudeMax = float
    SlopeMin = float
    SlopeMax = float

    def matches(self, height=-999, lat=-999, slope=-999):
        #print(self, height, lat, slope)
        if height != -999 and (height < self.MinHeight or height > self.MaxHeight):
            return False
        if lat != -999 and (lat < self.LatitudeMin or lat > self.LatitudeMax):
            return False
        if slope != -999 and (slope < self.SlopeMin or slope > self.SlopeMax):
            return False
        return True

    def get_first_layer_color(self):
        # if len(self.Layers) == 1:
        #     l = self.Layers[0]
        #     return l.R, l.G, l.B

        if len(self.Layers) > 0:
            l = self.Layers[0]
            return l.R, l.G, l.B

        return -1, -1, -1

    def get_lowest_depth_color(self):
        lowestLayer = None
        for layer in self.Layers:
            if lowestLayer == None:
                lowestLayer = layer
                continue
            if lowestLayer.Depth > layer.Depth:
                lowestLayer = layer
        if lowestLayer == None:
            return 0, 0, 0
        return lowestLayer.R, lowestLayer.G, lowestLayer.B

    # def get_highest_depth_color(self):
    #     highestLayer = None
    #     for layer in self.Layers:
    #         if highestLayer == None:
    #             highestLayer = layer
    #             continue
    #         if highestLayer.Depth < layer.Depth:
    #             highestLayer = layer
    #     if highestLayer == None:
    #         return 0, 0, 0
    #     return highestLayer.R, highestLayer.G, highestLayer.B

    # def get_lowest_depth_layer(self):
    #     lowestLayer = None
    #     for layer in self.Layers:
    #         if lowestLayer == None:
    #             lowestLayer = layer
    #             continue
    #         if lowestLayer.Depth > layer.Depth:
    #             lowestLayer = layer
    #     if lowestLayer == None:
    #         return None
    #     return lowestLayer.Material

    # def get_highest_depth_layer(self):
    #     highestLayer = None
    #     for layer in self.Layers:
    #         if highestLayer == None:
    #             highestLayer = layer
    #             continue
    #         if highestLayer.Depth < layer.Depth:
    #             highestLayer = layer
    #     if highestLayer == None:
    #         return 0, 0, 0
    #     return highestLayer.R, highestLayer.G, highestLayer.B

    def __str__(self):
        return f"MaterialRule(SlopeMax={self.SlopeMax}, SlopeMin={self.SlopeMin}, LatitudeMax={self.LatitudeMax}, LatitudeMin={self.LatitudeMin}, Layers={len(self.Layers)})"

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)

    @classmethod
    def from_dom_element(cls, element):
        mr = MaterialRule()
        h = element.getElementsByTagName("Height")
        lat = element.getElementsByTagName("Latitude")
        sl = element.getElementsByTagName("Slope")
        if len(h) > 0:
            mr.MinHeight = float(h[0].attributes["Min"].nodeValue)
            mr.MaxHeight = float(h[0].attributes["Max"].nodeValue)
        if len(lat) > 0:
            mr.LatitudeMin = float(lat[0].attributes["Min"].nodeValue)
            mr.LatitudeMax = float(lat[0].attributes["Max"].nodeValue)
        if len(sl) > 0:
            mr.SlopeMin = float(sl[0].attributes["Min"].nodeValue)
            mr.SlopeMax = float(sl[0].attributes["Max"].nodeValue)
        layers = element.getElementsByTagName("Layers")
        if len(layers) > 0:
            mr.Layers = [MaterialLayer.from_dom_element(
                x) for x in layers[0].getElementsByTagName("Layer")]
        return mr


class VoxelMaterial:
    id = str
    name = str
    rules = [MaterialRule]

    @classmethod
    def from_xml_element(cls, element):
        mat = VoxelMaterial()
        mat.id = int(element.attributes["Value"].nodeValue)
        mat.name = element.attributes["Name"].nodeValue
        mat.rules = [MaterialRule.from_dom_element(
            x) for x in element.getElementsByTagName("Rule")]
        return mat

    def get_layer(self, height=-999, lat=-999, slope=-999):
        for rule in self.rules:
            if rule.matches(height, lat, slope):
                return rule, rule.get_first_layer_color()
        return None, None

    # def get_color(self, rule, layername):
    #     if rule == None or layername == None:
    #         return -1, -1, -1
    #     return self.rules[rule].get_layer_color(layername)

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)


class PlanetDefinition:
    Name = str
    DefaultMaterial = MaterialLayer
    SimpleMaterials = {}
    ComplexMaterials = {}

    @classmethod
    def from_xml_element(cls, element):
        pd = PlanetDefinition()
        pd.Name = element.getElementsByTagName("Id")[0].getElementsByTagName("SubtypeId")[0].firstChild.nodeValue
        pd.DefaultMaterial = MaterialLayer()
        pd.SimpleMaterials = {}
        pd.ComplexMaterials = {}
        cplxmat = element.getElementsByTagName("ComplexMaterials")
        if len(cplxmat) > 0:
            matgroups = cplxmat[0].getElementsByTagName("MaterialGroup")
            for xmlmat in matgroups:
                mat = VoxelMaterial.from_xml_element(xmlmat)
                pd.ComplexMaterials[int(mat.id)] = mat

        custommat = element.getElementsByTagName("CustomMaterialTable")
        if len(custommat) > 0:
            materials = custommat[0].getElementsByTagName("Material")
            for material in materials:
                value = int(material.attributes["Value"].nodeValue)
                pd.SimpleMaterials[value] = MaterialLayer.from_dom_element(
                    material)

        pd.DefaultMaterial = MaterialLayer()
        defaultSurface = element.getElementsByTagName("DefaultSurfaceMaterial")
        if len(defaultSurface) > 0:
            pd.DefaultMaterial = MaterialLayer.from_dom_element(
                defaultSurface[0])

        return pd

    def get_color(self, value, height=-999, lat=-999, slope=-999):
        if value in self.ComplexMaterials:
            li, c = self.ComplexMaterials[value].get_layer(height, lat, slope)
            if li != None and c != (-1, -1, -1):
                return c

        if value in self.SimpleMaterials:
            mat = self.SimpleMaterials[value]
            return mat.R, mat.G, mat.B

        if 0 in self.ComplexMaterials:
            li, c = self.ComplexMaterials[0].get_layer(height, lat, slope)
            if li != None and c != (-1, -1, -1):
                return c

        #print(f"404 color: [v={value}, h={height}, lat={lat}, s={slope}]")
        return self.DefaultMaterial.R, self.DefaultMaterial.G, self.DefaultMaterial.B

    def cache(self, matfiles, matcoloravg):
        # Cache Complex
        for matid in self.ComplexMaterials:
            for rule in self.ComplexMaterials[matid].rules:
                for layer in rule.Layers:
                    if not layer.Material in matfiles:
                        print(f"404: {layer.Material}")
                        continue
                    file = matfiles[layer.Material]
                    if not file in matcoloravg:
                        print(f"404 color: {file}")
                        continue
                    layer.R, layer.G, layer.B = matcoloravg[file]
        # Cache Simple
        for matid in self.SimpleMaterials:
            layer = self.SimpleMaterials[matid]
            if not layer.Material in matfiles:
                print(f"404: {layer.Material}")
                continue
            file = matfiles[layer.Material]
            if not file in matcoloravg:
                print(f"404 color: {file}")
                continue
            layer.R, layer.G, layer.B = matcoloravg[file]
        # Cache default
        if not self.DefaultMaterial.Material in matfiles:
            return
        file = matfiles[self.DefaultMaterial.Material]
        if not file in matcoloravg:
            return
        self.DefaultMaterial.R, self.DefaultMaterial.G, self.DefaultMaterial.B = matcoloravg[
            file]


def get_closest_material(id):
    '''
        Gets the closes ID possible, assuming the image was linear interpolated
    '''
    closest_material = None
    delta = 10000000
    for matid in materialIdMap:
        if closest_material == None:
            closest_material = materialIdMap[matid]
            delta = abs(int(id)-int(matid))
            continue

        newdelta = abs(int(id) - int(matid))
        if newdelta < delta:
            delta = newdelta
            closest_material = materialIdMap[matid]

    # Cache it, so we dont need to run this expensive thing again
    materialIdMap[id] = closest_material
    return closest_material

def pixel_to_latitude(face, x_pixel, y_pixel, face_texture_width, face_texture_height):
    u = (x_pixel + 0.5) / face_texture_width * 2.0 - 1.0
    v = (y_pixel + 0.5) / face_texture_height * 2.0 - 1.0

    if face == "up":
        point = vmath.Vector3(u, 1.0, -v)
    elif face == "down":
        point = vmath.Vector3(u, -1.0, v)
    elif face == "left":
        point = vmath.Vector3(-1.0, v, -u)
    elif face == "right":
        point = vmath.Vector3(1.0, v, u)
    elif face == "back":
        point = vmath.Vector3(-u, v, 1.0)
    elif face == "front":
        point = vmath.Vector3(u, v, -1.0)

    point_on_sphere = point.normalize()
    latitude = math.asin(point_on_sphere.y)
    latitude_degrees = math.degrees(latitude)

    return latitude_degrees


def compute_lut(x, y, width, height, p):
    return x, y, pixel_to_latitude(p, x, y, width, height)

rad2deg = 360.0 / (math.pi * 2)
# planetData = "assets/Agaris/Data/Agaris.sbc"
# planetName = "Planet Agaris"
# baseAssetPath = "assets/Agaris/Data/PlanetDataFiles/Planet Agaris/"
planetData = "assets/Triton.sbc"
planetName = "Triton"
baseAssetPath = "assets/Triton/"

cubemap = ["front", "back", "down", "up", "left", "right"]

if __name__ == "__main__":
    freeze_support()
    # Load material files and color averages

    with open("matfiles.json") as f:
        matfiles = json.loads(f.read())

    with open("matcoloravg.json") as f:
        matcoloravg = json.loads(f.read())

    # Build Material ID Map from planet SBC data
    materialIdMap = {}
    dom1 = parse(planetData)
    planetDefinitionsXml = dom1.getElementsByTagName("Definition")

    if len(planetDefinitionsXml) == 0:
        planetDefinitionsXml = dom1.getElementsByTagName("PlanetGeneratorDefinitions")

    planetDefinitions = {}
    for planetDefinition in planetDefinitionsXml:
        pd = PlanetDefinition.from_xml_element(planetDefinition)
        print(pd.Name)
        if pd.Name == planetName:
            pd.cache(matfiles, matcoloravg)
            planetDefinitions[pd.Name] = pd

    with open("matcolormap.json", "w") as f:
        f.write(MyEncoder().encode(planetDefinitions))

    currentPlanet = planetDefinitions[planetName]

    for p in cubemap:
        hmPath = os.path.join(baseAssetPath, f"{p}.png")
        imPath = os.path.join(baseAssetPath, f"{p}_mat.png")
        latLutPath = f"latlut_{p}.png"

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
            for y0 in range(0, height):
                for x0 in range(0, width):
                    x1 = x0+1
                    y1 = y0+1

                    x = x1 - x0
                    y = y1 - y0

                    if x1 >= width:
                        x1 -= width
                    if y1 >= height:
                        y1 -= height

                    z0, _, _ = hm.getpixel((x0, y0))
                    z1, _, _ = hm.getpixel((x1, y1))
                    z = z1 - z0
                    a = int(
                        abs(math.asin(z / math.sqrt(x*x + y*y + z*z)) * rad2deg))
                    sm.putpixel((x0, y0), (a, a, a))

            sm.save(slopePath)
        else:
            print(f"Cached slope at {slopePath}")
            sm = Image.open(slopePath).convert('RGB')

        tex = Image.new(mode="RGB", size=im.size)
        r, _,  _ = im.split()  # Red Channel contains biome

        for y in range(0, height):
            if y % 100 == 0:
                print(f"Line {y} from {height}")
            for x in range(0, width):
                v, _, _ = im.getpixel((x, y))
                h, _, _ = hm.getpixel((x, y))
                s, _, _ = sm.getpixel((x, y))
                h /= 255.0
                lat,_,_ = latlutimg.getpixel((x,y))
                color = currentPlanet.get_color(v, h, lat, s)
                tex.putpixel((x, y), color)

        #idsNotFound = ",".join(idsNotFound)
        # print(f"IDs not found: {idsNotFound}")
        #tex = tex.filter(ImageFilter.GaussianBlur(radius=2))
        #tex = ImageEnhance.Sharpness(tex).enhance(8)
        tex.save(f"{p}.jpg")
        #break
