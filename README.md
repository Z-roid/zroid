# Zroid

A GPU-native IDE for systems programmers. Handles files other editors refuse to open, renders at uncapped framerate, and ships every feature built-in.

## Performance targets

| Metric | Target |
|---|---|
| Cold boot | < 300ms |
| Large file support | 500MB+ / 15M+ lines |
| Rendering | 120+ FPS uncapped |
| Concurrent splits | 4+ with no degradation |

## Stack

| Layer | Technology |
|---|---|
| Language | Rust (2024 edition) |
| Renderer | egui + WGPU (D3D12/Vulkan/Metal) |
| Text engine | ropey (B-tree rope) |
| Parsing | tree-sitter |
| Async | tokio |
| Large files | memmap2 |
| Scripting | rquickjs (embedded QuickJS) |

## Features

- Multi-buffer split view with drag-handle resizing
- Async file loading via memory-mapped I/O
- Per-buffer undo/redo history
- Basic syntax highlighting (Rust, C#, C++, JS, TS)
- File explorer with .gitignore support
- Resizable panels (explorer, output, AI assistant)
- GPU-accelerated rendering at uncapped FPS

## Building

```sh
cargo build --release
cargo run
```

Requires Rust 1.85+ (2024 edition).

On Windows, D3D12 is used by default. If `DxcCreateInstance` errors appear, set:

```sh
set WGPU_DX12_COMPILER=fxc
```

## Organization

Zroid is the flagship project of [Z-roid](https://github.com/Z-roid), a collection of GPU-native developer tools built in Rust.

| Repository | Description |
|---|---|
| [zroid](https://github.com/Z-roid/zroid) | The IDE |
| [zyre](https://github.com/Z-roid/zyre) | Reverse engineering engine |
| [zir](https://github.com/Z-roid/zir) | SSA intermediate representation |
| [zbin](https://github.com/Z-roid/zbin) | Binary rope  O(log n) access into large binaries |

## License

MIT
