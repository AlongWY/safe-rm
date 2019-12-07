#[macro_use]
extern crate clap;
extern crate version_check;

use clap::Shell;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

include!("src/app.rs");

fn main() {
    match version_check::is_min_version("1.39.0") {
        Some(true) => {}
        // rustc version too small or can't figure it out
        _ => {
            writeln!(&mut io::stderr(), "'safe-rm' requires rustc >= 1.39.0").unwrap();
            exit(1);
        }
    }

    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or(std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut app = build();
    app.gen_completions("safe-rm", Shell::Bash, &outdir);
    app.gen_completions("safe-rm", Shell::Fish, &outdir);
    app.gen_completions("safe-rm", Shell::Zsh, &outdir);
    app.gen_completions("safe-rm", Shell::PowerShell, &outdir);

    println!("Generated completion!!!");
}