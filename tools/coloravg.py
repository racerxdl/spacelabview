#!/usr/bin/env python3
import os
import cv2
import numpy
import json

def avg_color(filename):
    myimg = cv2.imread(filename)
    avg_color_per_row = numpy.average(myimg, axis=0)
    avg_color = numpy.average(avg_color_per_row, axis=0)
    b, g, r = avg_color
    return int(r),int(g),int(b)

colors = {}

for filename in os.listdir("./assets/DDS"):
    f = os.path.join("./assets/DDS", filename)
    # checking if it is a file
    if os.path.isfile(f) and ".png" in filename and "_cm" in filename:
        r,g,b = avg_color(f)
        print(f"{filename} => {r},{g},{b}")
        colors[filename] = [r,g,b]

# Save
with open("./luts/matcoloravg.json", "w") as f:
    f.write(json.dumps(colors, indent=3))
