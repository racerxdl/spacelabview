
# SpaceLab Viewer (WIP)

This is WAY under testing and development. But it does generate some nice maps.

REQUIRES SpaceLab Plugin on Torch (To be released yet)

# Generating material textures from game assets

Check `tools/surfacegen.py`. In the start of the file you will find these lines:

```python
planetData = "assets/Triton.sbc"
planetName = "Triton"
baseAssetPath = "assets/Triton/"
```

Set `planetData` to the SBC file containing the planet description XML, `planetName` to the name of the planet (as per the XML file) and `baseAssetPath` to the folder where texture files like `up.png`, `down.png` are.


# Building UI

Requires nodejs and npm

```bash
cd webui
npm install
npm run build
```

The contents of `dist` folder is WebUI built.

# Building the Golang Backend

```bash
# Linux / MacOSX
go build -o agarismap
# Windows
go build -o agarismap.exe
```

# CHANGES TO GAME DATA

### Agaris

As is, no changes required...

### Vanilla Planets

Weirdly, I had to swap the front and left images, and rotate 180 degrees the up image.