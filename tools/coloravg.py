#!/usr/bin/env python3
import os
import cv2
import numpy
import json

criteria = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 200, .1)
flags = cv2.KMEANS_RANDOM_CENTERS
n_colors = 5

def avg_color(filename):
    print(f"Reading {filename}")
    myimg = cv2.imread(filename)

    if myimg.shape[0] < 64:
        avg_color_per_row = numpy.average(myimg, axis=0)
        avg_color = numpy.average(avg_color_per_row, axis=0)
        b, g, r = avg_color
    else:
        myimg = cv2.resize(myimg, dsize=(int(myimg.shape[0]/2), int(myimg.shape[1]/2)), interpolation=cv2.INTER_NEAREST)
        if myimg.shape[2] == 4:
            myimg = myimg[:, :, :-1]
        pixels = numpy.float32(myimg.reshape(-1, 3))
        _, labels, palette = cv2.kmeans(pixels, n_colors, None, criteria, 10, flags)
        _, counts = numpy.unique(labels, return_counts=True)
        dominant = palette[numpy.argmax(counts)]
        b, g, r = dominant
    return int(r),int(g),int(b)

colors = {}

for root, dirs, files in os.walk("./assets/DDS"):
    path = root.split(os.sep)
    #print((len(path) - 1) * '---', os.path.basename(root))
    for file in files:
        fullpath = os.path.join(root, file)
        vr = root.replace("./assets/DDS/", "").replace("./assets/DDS", "")
        if vr == "":
            vr = "default"
        #print(file, fullpath, os.path.isfile(fullpath))
        if os.path.isfile(fullpath) and ".png" in file:
            r,g,b = avg_color(fullpath)
            print(f"{vr}|{file} => {r},{g},{b}")
            if not vr in colors:
                colors[vr] = {}
            colors[vr][file] = [r,g,b]

# for filename in os.listdir("./assets/DDS"):
#     f = os.path.join("./assets/DDS", filename)
#     # checking if it is a file
#     if os.path.isfile(f) and ".png" in filename and "_cm" in filename:
#         r,g,b = avg_color(f)
#         print(f"{filename} => {r},{g},{b}")
#         colors[filename] = [r,g,b]

# Save
with open("./luts/matcoloravg.json", "w") as f:
    f.write(json.dumps(colors, indent=3))
