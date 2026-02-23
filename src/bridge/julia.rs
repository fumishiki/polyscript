/// Julia ブリッジ — sp_bridge! の生成コードと同等。libjulia-sys への移行時はここを差し替える。
pub fn run(s: &str, a: &[String]) -> anyhow::Result<()> { super::sp("julia", &[], s, a) }
