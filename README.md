# polyscript 🦀

> Rust CLI that dispatches to **16 languages** via FFI bridges and subprocess.  
> 187 lines of `src` across 5 files.

---

## Languages

| Subcommand | Runtime | Bridge | Startup | Best for |
|---|---|---|---|---|
| `py` | Python | PyO3 in-process | **zero** | ML, scripting |
| `cpp` | C++ | libloading | **zero** | SIMD, native libs |
| `jl` | Julia | subprocess | 1–4 s | ODE, SciML |
| `go` | Go | subprocess | ~30 ms | CLI, APIs |
| `js` | Node.js | subprocess | ~40 ms | JSON, web |
| `ts` | Deno | subprocess | ~60 ms | TypeScript |
| `lua` | Lua | subprocess | ~10 ms | embedded config |
| `r` | R | subprocess | ~300 ms | stats, plots |
| `mojo` | Mojo | subprocess | ~200 ms | GPU, Python+speed |
| `zig` | Zig | subprocess | ~50 ms | C replacement |
| `wasm` | wasmtime | subprocess | ~80 ms | sandboxed plugins |
| `hs` | GHC | subprocess | ~150 ms | DSL, correctness |
| `swift` | Swift | subprocess | ~200 ms | Apple SDK |
| `kt` | Kotlin | subprocess | ~1 s | JVM ecosystem |
| `nim` | Nim | subprocess | ~100 ms | C-speed scripting |
| `fort` | gfortran | compile+run | compile+~5 ms | HPC, legacy |

---

## Install

```bash
git clone https://github.com/fumishiki/polyscript
cd polyscript
cargo build --release
```

Requires: Python dev headers (PyO3). Each subcommand needs its runtime in `$PATH`.

---

## Usage

```bash
polyscript <COMMAND> <script> [args...]
polyscript cpp <lib.so> <func> [args...]
```

```bash
# Python (in-process, zero overhead)
polyscript py  scripts/python/example.py  hello

# Julia
polyscript jl  scripts/julia/example.jl   hello

# Go
polyscript go  scripts/go/example.go      hello

# C++ shared library
g++ -shared -fPIC -o libex.dylib scripts/cpp/example.cpp
polyscript cpp libex.dylib run   hello
```

---

## Architecture

```
polyscript
├── bridge::python   PyO3 in-process          (python.rs, 20 lines)
├── bridge::cpp      libloading dynamic load   (cpp.rs,    23 lines)
└── bridge::mod      sp() / cr() + macros      (mod.rs,    63 lines)
     ├─ sp(cmd, pre[], script, args[])  →  Command::new(cmd).args(pre).arg(script).args(args)
     ├─ cr(compiler, flags[], src, args) →  compile to /tmp/polyscript_out → run
     ├─ sp_bridge!(mod, cmd [, pre...])  →  generates pub mod with sp() call
     └─ cr_bridge!(mod, compiler)        →  generates pub mod with cr() call
```

13 language bridges declared in 15 lines:

```rust
sp_bridge!(julia, "julia");
sp_bridge!(go,    "go",       "run");
sp_bridge!(js,    "node");
sp_bridge!(ts,    "deno",     "run");
sp_bridge!(lua,   "lua");
sp_bridge!(r,     "Rscript");
sp_bridge!(mojo,  "mojo");
sp_bridge!(zig,   "zig",      "run");
sp_bridge!(wasm,  "wasmtime", "run");
sp_bridge!(hs,    "runghc");
sp_bridge!(swift, "swift");
sp_bridge!(kt,    "kotlinc",  "-script");
sp_bridge!(nim,   "nim",      "r");
cr_bridge!(fort,  "gfortran");
```

### C++ convention

```cpp
extern "C" int run(int argc, const char** argv) { return 0; }
```

---

## Data IPC

Data is passed as string arguments or **file paths**. Recommended pattern:

| Data size | Method |
|---|---|
| Flags / IDs | CLI `[args...]` |
| MB–GB | Parquet / Arrow IPC file path as `args[1]` |
| In-process | Python (PyO3) / C++ (libloading) — no disk I/O |

```bash
polyscript py  preprocess.py  /data/raw.parquet   /tmp/features.arrow
polyscript jl  simulate.jl    /tmp/features.arrow  /tmp/result.arrow
polyscript r   plot.r         /tmp/result.arrow    /tmp/figures/
```

---

## Known limitations

- **Julia cold-start**: 1–4 s per call (fix: `libjulia-sys` FFI — planned)
- **Kotlin cold-start**: ~1 s JVM boot (fix: GraalVM Native Image — planned)
- **Concurrent `fort`**: both calls overwrite `/tmp/polyscript_out` (fix: per-PID path — planned)
- **No structured IPC**: data contract is the caller's responsibility

---

## License

MIT OR Apache-2.0
