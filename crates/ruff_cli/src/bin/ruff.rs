use clap::{Parser};
use std::process::ExitCode;
use std::time::Instant;

use colored::Colorize;

use ruff_cli::args::{Args};
use ruff_cli::{run, ExitStatus};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "openbsd"),
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64"
    )
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn main() -> ExitCode {
    print!("Look at that!  I found the root!\n");
    let args: Vec<_> = wild::args().collect();
    let args = Args::parse_from(args);
    let before = Instant::now();
    run(args);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let args: Vec<_> = wild::args().collect();
    let args = Args::parse_from(args);
    match run(args) {
        Ok(code) => code.into(),
        Err(err) => {
            #[allow(clippy::print_stderr)]
            {
                eprintln!("{}{} {err:?}", "error".red().bold(), ":".bold());
            }
            ExitStatus::Error.into()
        }
    }
}
