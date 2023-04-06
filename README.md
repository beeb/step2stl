# STEP to STL converter

This simple graphical utility uses the [OpenCascade](https://dev.opencascade.org/) OCCT library to convert a STEP file
into a triangulated STL file.

There are 3 presets for quality, from the coarsest setting which is suitable for FDM printing
(values taken from [PrusaSlicer](https://github.com/prusa3d/PrusaSlicer)) to the finest which can be used with
artistic 3D software like Blender.

## Build instructions

To build this software on Windows, you need the header files and library files from the OpenCASCADE Technology library.
Additionally, you need the rust toolchain (e.g. installed with [`rustup`](https://rustup.rs/)) and
a JavaScript package manager, ideally [`pnpm`](https://pnpm.io/).

- Get OCCT version 7.7.0 from [the official release page](https://dev.opencascade.org/release)
- Install the header and binary files (e.g. in the default location at `C:\OpenCASCADE-7.7.0-vc14-64`)
- Copy the folder `C:\OpenCASCADE-7.7.0-vc14-64\opencascade-7.7.0\inc` into the `src-tauri` folder of this repo
- Copy the folder `C:\OpenCASCADE-7.7.0-vc14-64\opencascade-7.7.0\win64\vc14\lib` into the `src-tauri` folder of this
  repo
- Run `pnpm install`
- Run `pnpm tauri build`

The software is currently targetting Windows but should be easily compiled for other platforms by copying the relevant
library files into the `lib` folder. Pre-built binaries for OpenCASCADE can probably be installed with Homebrew
(package `opencascade`) or `nix-env` (package `nixpkgs.opencascade-occt`). Once the runtime dependencies binaries are
available on the `PATH`, the software should run fine.

## Runtime dependencies

In order to run the software, it needs access to the compiled DLLs from the OpenCascade library and its dependencies.
In the OpenCASCADE install folder, find the following files and copy them into the directory containing `step2stl.exe`:

<details>
<summary>List of DLLs that are needed</summary>

```
TKXSBase.dll
avcodec-57.dll
avformat-57.dll
avutil-55.dll
FreeImage.dll
freetype.dll
openvr_api.dll
swscale-4.dll
tbb12.dll
tbbmalloc.dll
TKBO.dll
TKBRep.dll
TKCAF.dll
TKCDF.dll
TKernel.dll
TKG2d.dll
TKG3d.dll
TKGeomAlgo.dll
TKGeomBase.dll
TKHLR.dll
TKLCAF.dll
TKMath.dll
TKMesh.dll
TKPrim.dll
TKService.dll
TKShHealing.dll
TKSTEP.dll
TKSTEP209.dll
TKSTEPAttr.dll
TKSTEPBase.dll
TKSTL.dll
TKTopAlgo.dll
TKV3d.dll
TKVCAF.dll
TKXCAF.dll
TKXDE.dll
```

</details>

## Development

- Follow the build instructions above
- Copy the DLLs from the runtime dependencies section above into a folder `ddls` at the root of this directory
- Add the `dlls` folder to your path, i.e. `$env:Path += ";C:\...\dlls"`
- Run `pnpm tauri dev`

## License

According to the [OCCT license exception](https://github.com/Open-Cascade-SAS/OCCT/blob/master/OCCT_LGPL_EXCEPTION.txt),
this work which only links to the library is distributed under a different license from the original code, namely
either the MIT license or Apache-2.0 license.
