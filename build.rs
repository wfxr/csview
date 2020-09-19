use std::fs;
use std::path::Path;
use std::str::FromStr;
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| std::env::var_os("OUT_DIR"))
        .expect("OUT_DIR not found");
    let outdir_path = Path::new(&outdir);
    let mut app = Opt::clap();

    for shell in &Shell::variants() {
        let dir = outdir_path.join(shell);
        fs::create_dir_all(&dir)?;
        app.gen_completions(env!("CARGO_PKG_NAME"), Shell::from_str(shell)?, &dir);
    }
    Ok(())
}
