# rview

<div align="center">
  <img alt="Utah teapot demo" src="gif/teapot_compressed.gif" height="50%" width="50%">
</div>

A software rasterizer written in Rust that renders `.obj` models as ASCII art directly in your terminal.

## Features

- 3D transformations: scale, rotate, translate
- Perspective projection
- Triangle rasterization
- Diffuse lighting based on face normals, tied to the camera position

## Usage

> [!IMPORTANT]
> Only works with triangulated models!

```bash
cargo run --release -- <path/to/model.obj>
```

| Control | Actions |
|---|---|
| Mouse drag | Rotate camera |
| Scroll up/down | Zoom in/out |
| `q` | Quit |

## Build With

- [`rust`](https://rust-lang.org/) - Language
- [`crossterm`](https://github.com/crossterm-rs/crossterm) - cross-platform terminal manipulation (input events, cursor, rendering)
- [`glam`](https://github.com/bitshifter/glam-rs) - SIMD-accelerated linear algebra (vectors, matrices)
- [`obj-rs`](https://github.com/simnalamburt/obj-rs) - `.obj` file parsing
- [`clap`](https://github.com/clap-rs/clap) - command-line argument parsing
- [`anyhow`](https://github.com/dtolnay/anyhow) - ergonomic error handling
- [`rayon`](https://github.com/rayon-rs/rayon) - data-parallel vertex processing

## Assets

- **Utah Teapot**: [Sketchfab](https://sketchfab.com/3d-models/utah-teapot-92f31e2028244c4b8ef6cbc07738aee5)
- **Suzanne**: Blender

## License

This project is licensed under the GNU GPLv3 License - see the [COPYING.md](COPYING.md) file for details
