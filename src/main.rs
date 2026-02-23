use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod bridge;

#[derive(Parser)]
#[command(name = "polyscript", about = "Polyglot script runner — 15 言語を FFI / subprocess で呼び出す")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

/// subprocess 系サブコマンドが共通で受け取る引数。
#[derive(Args)]
struct S {
    /// スクリプトファイルのパス
    script: String,
    /// スクリプトへ渡す引数
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Python スクリプト（PyO3 インプロセス）
    Py(S),
    /// Julia スクリプト（subprocess）
    Jl(S),
    /// Go スクリプト（go run）
    Go(S),
    /// JavaScript（node）
    Js(S),
    /// TypeScript（deno run）
    Ts(S),
    /// Lua スクリプト
    Lua(S),
    /// R スクリプト（Rscript）
    R(S),
    /// Mojo スクリプト
    Mojo(S),
    /// Zig スクリプト（zig run）
    Zig(S),
    /// WebAssembly モジュール（wasmtime run）
    Wasm(S),
    /// Haskell スクリプト（runghc）
    Hs(S),
    /// Swift スクリプト
    Swift(S),
    /// Kotlin スクリプト（kotlinc -script）
    Kt(S),
    /// Nim スクリプト（nim r）
    Nim(S),
    /// Fortran ソース（gfortran compile + run）
    Fort(S),
    /// C++ 共有ライブラリ関数（libloading）
    Cpp { lib: String, func: String, args: Vec<String> },
}

fn main() -> Result<()> {
    use bridge::*;
    use Cmd::*;
    match Cli::parse().cmd {
        Py(a)    => python::run(&a.script, &a.args),
        Jl(a)    => julia::run(&a.script, &a.args),
        Go(a)    => go::run(&a.script, &a.args),
        Js(a)    => js::run(&a.script, &a.args),
        Ts(a)    => ts::run(&a.script, &a.args),
        Lua(a)   => lua::run(&a.script, &a.args),
        R(a)     => r::run(&a.script, &a.args),
        Mojo(a)  => mojo::run(&a.script, &a.args),
        Zig(a)   => zig::run(&a.script, &a.args),
        Wasm(a)  => wasm::run(&a.script, &a.args),
        Hs(a)    => hs::run(&a.script, &a.args),
        Swift(a) => swift::run(&a.script, &a.args),
        Kt(a)    => kt::run(&a.script, &a.args),
        Nim(a)   => nim::run(&a.script, &a.args),
        Fort(a)  => fort::run(&a.script, &a.args),
        Cpp { lib, func, args } => cpp::run(&lib, &func, &args),
    }
}
