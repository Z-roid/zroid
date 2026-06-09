default:
    @just --list

# ── Build ──────────────────────────────────────────────────────────────────────

build:
    cargo build

release:
    cargo build --release

check:
    cargo check

clippy:
    cargo clippy -- -D warnings

# ── Run ───────────────────────────────────────────────────────────────────────

run:
    cargo run

run-release:
    cargo run --release

watch:
    cargo watch -x run

# ── Test ──────────────────────────────────────────────────────────────────────

test:
    cargo nextest run

test-verbose:
    cargo nextest run --no-capture

# ── Profiling ─────────────────────────────────────────────────────────────────

flame:
    cargo flamegraph --bin zord

bloat:
    cargo bloat --release --crates

bench:
    hyperfine --warmup 3 'cargo run --release'

# ── Audit ─────────────────────────────────────────────────────────────────────

audit:
    cargo audit

deny:
    cargo deny check

# ── Docs ──────────────────────────────────────────────────────────────────────

doc:
    cargo doc --no-deps --open

doxygen:
    doxygen Doxyfile

# ── C++ (zyre kernel) ─────────────────────────────────────────────────────────

# Build the C++ analysis kernel (requires MSVC or clang-cl)
cpp-build:
    cmake -B build/cpp -S cpp -DCMAKE_BUILD_TYPE=Release
    cmake --build build/cpp --config Release

# Check C++ with clang-tidy
cpp-lint:
    clang-tidy cpp/src/*.cpp -- -std=c++23 -Icpp/include

# ── Housekeeping ──────────────────────────────────────────────────────────────

clean:
    cargo clean
    @if exist build rmdir /s /q build

fmt:
    cargo fmt

loc:
    @powershell -Command "(Get-ChildItem src -Recurse -Filter *.rs | Get-Content | Measure-Object -Line).Lines"

loc-cpp:
    @powershell -Command "(Get-ChildItem cpp -Recurse -Include *.cpp,*.hpp | Get-Content | Measure-Object -Line).Lines"
