# Zord IDE

> "The Z's ain't playing."

Zord is a hyper-performance, general-purpose IDE built for absolute speed. While modern editors have succumbed to the bloat of web technologies, Zord is engineered from the metal up using **Rust**, **C++23**, and **GPU-accelerated rendering**.

## ⚡ The Z Alliance
Zord is inspired by and built with the spirit of [Zed](https://zed.dev). We believe in a world where your IDE should never be the bottleneck. 

- **Zed** is the speed king.
- **Zord** is the classic-soul powerhouse.
- Together, the Z's are signing the death certificates for bloated editors.

## 🚀 Performance Benchmarks
- **Cold Boot:** < 300ms
- **Large File Handling:** 500MB+ (15M+ lines) opened instantly via Async I/O.
- **Rendering:** Solid 120+ FPS (Uncapped) using a virtualized GPU-painted editor engine.
- **Multi-View:** Support for 4+ concurrent splits with zero performance degradation.

## 🏗 Architecture
- **Core:** Rust with a `ropey` B-tree text engine.
- **Heavy Lifting:** High-performance C++23 modules via safe FFI.
- **UI:** Utilitarian, classic Lazarus/Code::Blocks aesthetic powered by GPU-accelerated `egui` (WGPU).
- **Extension Host:** Embedded `rquickjs` (JavaScript) for ultra-lightweight scripting.

---
*Built for the engineers who remember what 'fast' actually feels like.*
