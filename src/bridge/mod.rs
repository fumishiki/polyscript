pub mod python;
pub mod cpp;
pub mod ktn;

use anyhow::{ensure, Result};
use std::process::Command;

/// 汎用 subprocess ランナー。`cmd [pre...] script [args...]` を実行する。
pub(crate) fn sp(cmd: &str, pre: &[&str], script: &str, args: &[String]) -> Result<()> {
    let s = Command::new(cmd).args(pre).arg(script).args(args).status()?;
    ensure!(s.success(), "{cmd} exited with {s}");
    Ok(())
}

/// コンパイル→実行の 2 ステップランナー（Fortran など）。
pub(super) fn cr(compiler: &str, cflags: &[&str], script: &str, args: &[String]) -> Result<()> {
    let out = format!("/tmp/polyscript_out_{}", std::process::id());
    ensure!(
        Command::new(compiler).args(cflags).arg(script).arg("-o").arg(&out).status()?.success(),
        "{compiler}: compilation failed"
    );
    ensure!(Command::new(&out).args(args).status()?.success(), "binary exited non-zero");
    Ok(())
}

/// subprocess ブリッジモジュールを宣言的に生成するマクロ。
macro_rules! sp_bridge {
    ($mod:ident, $cmd:literal $(, $pre:literal)*) => {
        pub mod $mod {
            pub fn run(s: &str, a: &[String]) -> anyhow::Result<()> {
                super::sp($cmd, &[$($pre),*], s, a)
            }
        }
    };
}

/// コンパイル型ブリッジモジュールを生成するマクロ。
macro_rules! cr_bridge {
    ($mod:ident, $compiler:literal $(, $flag:literal)*) => {
        pub mod $mod {
            pub fn run(s: &str, a: &[String]) -> anyhow::Result<()> {
                super::cr($compiler, &[$($flag),*], s, a)
            }
        }
    };
}

// ── subprocess 型ブリッジ ──────────────────────────────────────────────────
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

// ── compile & run 型ブリッジ ─────────────────────────────────────────────
cr_bridge!(fort, "gfortran");
