
import json
from json import JSONEncoder

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

class OreMap:
    Value = int
    Type = str
    Start = int
    Depth = int
    TargetColor = (int,int,int)
    ColorInfluence = int

    @classmethod
    def from_xml_element(cls, element):
        ore = OreMap()
        ore.Value = int(element.attributes["Value"].nodeValue)
        ore.Type = element.attributes["Type"].nodeValue
        ore.Start = int(element.attributes["Start"].nodeValue)
        ore.Depth = int(element.attributes["Depth"].nodeValue)
        if "TargetColor" in element.attributes:
            TargetColor = element.attributes["TargetColor"].nodeValue
            # TargetColor in format #RRGGBB convert to tuple
            ore.TargetColor = (int(TargetColor[1:3], 16), int(TargetColor[3:5], 16), int(TargetColor[5:7], 16))
        else:
            ore.TargetColor = (0,0,0)

        if "ColorInfluence" in element.attributes:
            ore.ColorInfluence = int(element.attributes["ColorInfluence"].nodeValue)
        else:
            ore.ColorInfluence = 0

        return ore

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__,
                          sort_keys=True, indent=4)

    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return f"OreMap(Value={self.Value}, Type={self.Type}, Start={self.Start}, Depth={self.Depth}, TargetColor={self.TargetColor}, ColorInfluence={self.ColorInfluence})"

class PlanetDefinition:
    Name = str
    DefaultMaterial = MaterialLayer
    SimpleMaterials = {}
    ComplexMaterials = {}
    Ores = {}
    BaseFolder = ""

    @classmethod
    def from_xml_element(cls, element):
        pd = PlanetDefinition()
        pd.Name = element.getElementsByTagName("Id")[0].getElementsByTagName(
            "SubtypeId")[0].firstChild.nodeValue
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

        pd.Ores = {}

        ores = element.getElementsByTagName("OreMappings")
        if len(ores) > 0:
            oremaps = ores[0].getElementsByTagName("Ore")
            #print(f"{len(oremaps)} Ores for {pd.Name}")
            for ore in oremaps:
                om = OreMap.from_xml_element(ore)
                pd.Ores[om.Value] = om
        #else:
        #    print(f"No ores for {pd.Name}")

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
                    path = file["path"]
                    file = file["file"]
                    colorset = False
                    if path in matcoloravg:
                        if file in matcoloravg[path]:
                            layer.R, layer.G, layer.B = matcoloravg[path][file]
                            colorset = True
                    if not colorset:
                        if file in matcoloravg["default"]:
                            layer.R, layer.G, layer.B = matcoloravg["default"][file]
                        else:
                            print(f"404 color: {path} | {file}")

        # Cache Simple
        for matid in self.SimpleMaterials:
            layer = self.SimpleMaterials[matid]
            if not layer.Material in matfiles:
                print(f"404: {layer.Material}")
                continue
            file = matfiles[layer.Material]
            path = file["path"]
            file = file["file"]
            colorset = False
            if path in matcoloravg:
                if file in matcoloravg[path]:
                    layer.R, layer.G, layer.B = matcoloravg[path][file]
                    colorset = True
            if not colorset:
                if file in matcoloravg["default"]:
                    layer.R, layer.G, layer.B = matcoloravg["default"][file]
                else:
                    print(f"404 color: {path} | {file}")
        # Cache default
        if not self.DefaultMaterial.Material in matfiles:
            return
        file = matfiles[self.DefaultMaterial.Material]
        path = file["path"]
        file = file["file"]
        colorset = False
        if path in matcoloravg:
            if file in matcoloravg[path]:
                self.DefaultMaterial.R, self.DefaultMaterial.G, self.DefaultMaterial.B = matcoloravg[path][file]
                colorset = True
        if not colorset:
            if file in matcoloravg["default"]:
                self.DefaultMaterial.R, self.DefaultMaterial.G, self.DefaultMaterial.B = matcoloravg["default"][file]
            else:
                print(f"404 color: {path} | {file}")
