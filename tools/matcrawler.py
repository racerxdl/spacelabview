
import os, json
from xml.dom.minidom import parse, parseString

folder = "assets"

voxelMaterialToFile = {}

def parseXML(fullpath):
    try:
        dom1 = parse(fullpath)
        mats = dom1.getElementsByTagName("VoxelMaterial")
        for m in mats:
            idtype = m.getElementsByTagName("Id")
            if len(idtype) == 0:
                continue
            matid = idtype[0].getElementsByTagName("SubtypeId")[0].firstChild.data
            texfile = m.getElementsByTagName("ColorMetalXZnY")[0].firstChild.data
            texfile = os.path.basename(texfile.replace("\\","/").replace(".dds", ".png"))
            voxelMaterialToFile[matid] = texfile
    except Exception:
        pass

# traverse root directory, and list directories as dirs and files as files
for root, dirs, files in os.walk("assets"):
    path = root.split(os.sep)
    #print((len(path) - 1) * '---', os.path.basename(root))
    for file in files:
        if ".xml" in file or ".sbc" in file:
            fullpath = os.path.join(root, file)
            parseXML(fullpath)

with open("matfiles.json", "w") as f:
    f.write(json.dumps(voxelMaterialToFile, indent=3))
