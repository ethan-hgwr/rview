# Fork Summary

This document summarizes the changes in this fork to help the original
project's maintainer review them. The main README remains the `rview` user
guide; this file describes what was added and how to verify it.

## What Was Added

### Optional Desktop Viewer

The `--gui` option opens the model in a 1024 × 768 desktop window. It reuses
the project's existing renderer and model loaders, but draws colored pixels
with per-face lighting instead of relying on terminal characters.

```bash
cargo run --release -- --gui path/to/model.obj
```

The viewer supports the formats already supported by the project: OBJ, glTF,
GLB, and STL. Press `Esc` or `q` to close the window.

### Controls and Camera

- Drag with the left mouse button to orbit the camera.
- Use the arrow keys to rotate the camera.
- Use the mouse wheel to zoom in or out.
- The desktop viewer also supports existing animation modes, such as
  `turntable`.

The terminal mode remains available and is still the default. In terminal mode,
the arrow keys can also rotate the camera. The terminal projection aspect ratio
was adjusted to account for text cells being taller than they are wide.

### Terminal Appearance

The long letter-based shading ramp was replaced with four block-density
characters (`░`, `▒`, `▓`, `█`) and a blank background. This produces more
consistent shading and makes the rendered model less like unrelated random
characters.

## Main Files Changed

- `src/gui.rs`: desktop window, colored rendering, and viewer controls.
- `src/pipeline.rs` and `src/raster.rs`: shared rasterization for terminal
  output and colored pixels.
- `src/palette.rs`: brightness ramps for terminal and desktop output.
- `src/main.rs` and `src/cli/mod.rs`: `--gui` option, output selection, and
  keyboard controls.
- `Cargo.toml` and `Cargo.lock`: `minifb` dependency for the desktop window.
- `README.md`: updated usage instructions and controls.

## Verification

The changes were checked with:

```bash
cargo check
cargo test
cargo run -- --help
```

The existing tests and the shading-ramp tests passed. The command-line help
also displays the `--gui` option.

## Scope and Limitations

The viewer is a simple mesh preview with color and lighting calculated per
face. It is not a PBR viewer and does not add support for materials, textures,
or model editing. The window starts at a fixed resolution of 1024 × 768. The
terminal mode and its existing file formats remain available.
