#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate regex;

use std::env;
use std::fs;
use std::path::PathBuf;

use failure::Error;
use regex::Regex;
use structopt::StructOpt;

/// Run fuzzer collection
#[derive(StructOpt, Debug)]
enum Cli {
    /// Run fuzz targets until the end of time (or Ctrl+C)
    #[structopt(name = "continuous")]
    Continuous {
        #[structopt(short = "q", long = "filter")]
        filter: Option<String>,
        #[structopt(short = "t", long = "timeout", default_value = "10")]
        timeout: i32,
        /// Which fuzzer to run
        #[structopt(
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: Fuzzer,
    },
    /// Run one target with specific fuzzer
    #[structopt(name = "run")]
    Run {
        /// Which target to run
        target: String,
        /// Which fuzzer to run
        #[structopt(
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: Fuzzer,
    },
    /// List all available targets
    #[structopt(name = "list-targets")]
    ListTargets,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    use Cli::*;
    let cli = Cli::from_args();

    match cli {
        ListTargets => {
            for target in &get_targets()? {
                println!("{}", target);
            }
        }
        Run { target, fuzzer } => {
            use Fuzzer::*;
            match fuzzer {
                Afl => run_afl(&target, None)?,
                Honggfuzz => run_honggfuzz(&target, None)?,
                Libfuzzer => run_libfuzzer(&target, None)?,
            }
        }
        Continuous { filter, timeout, fuzzer } => {
            let targets = get_targets()?;
            loop {
                for target in &targets {
                    use Fuzzer::*;
                    match fuzzer {
                        Afl => run_afl(&target, Some(timeout))?,
                        Honggfuzz => run_honggfuzz(&target, Some(timeout))?,
                        Libfuzzer => run_libfuzzer(&target, Some(timeout))?,
                    }
                }
            }
        }
    }
    Ok(())
}

fn common_dir() -> Result<PathBuf, Error> {
    let p = env::var("CARGO_MANIFEST_DIR")
        .map(From::from)
        .or_else(|_| env::current_dir())?
        .join("common");

    Ok(p)
}

fn create_seed_dir(target: &str) -> Result<PathBuf, Error> {
    let seed_dir = common_dir()?.join("seeds").join(&target);
    fs::create_dir_all(&seed_dir)?;
    Ok(seed_dir)
}

fn get_targets() -> Result<Vec<String>, Error> {
    let targets_rs = fs::read_to_string(&common_dir()?.join("src/lib.rs"))?;
    let match_fuzz_fs = Regex::new(r"pub fn fuzz_(\w+)\(")?;
    let target_names = match_fuzz_fs
        .captures_iter(&targets_rs)
        .map(|x| x[1].to_string());
    Ok(target_names.collect())
}

fn run_honggfuzz(target: &str, timeout: Option<i32>) -> Result<(), Error> {
    use std::process::Command;
    let seed_dir = create_seed_dir(&target)?;

    let dir = env::current_dir()?.join("fuzzer-honggfuzz");
    let args = format!(
        "-f {} \
         --covdir_all hfuzz_workspace/{}/input \
         {} \
         {}",
        seed_dir.to_string_lossy(),
        target,
        if let Some(t) = timeout { format!("--run_time {}", t) } else { "".into() },
        env::var("HFUZZ_RUN_ARGS").unwrap_or_default()
    );

    let fuzzer = Command::new("cargo")
        .args(&["hfuzz", "run", &target])
        .env("HFUZZ_RUN_ARGS", &args)
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer.success(), "hongfuzz quit with code {}", fuzzer);
    Ok(())
}

fn run_afl(target: &str, _timeout: Option<i32>) -> Result<(), Error> {
    use std::process::Command;

    let dir = env::current_dir()?.join("fuzzer-afl");

    let seed_dir = create_seed_dir(&target)?;
    let corpus_dir = dir.join(&format!("corpus-{}", target));
    fs::create_dir_all(&corpus_dir)?;

    let fuzzer = Command::new("cargo")
        .args(&["afl", "fuzz"])
        .arg("-i")
        .arg(&seed_dir)
        .arg("-o")
        .arg(&corpus_dir)
        .args(&["--", &format!("target/release/{}", target)])
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer.success(), "AFL quit with code {}", fuzzer);
    Ok(())
}

fn run_libfuzzer(target: &str, _timeout: Option<i32>) -> Result<(), Error> {
    use std::process::Command;
    let seed_dir = create_seed_dir(&target)?;

    #[cfg(target_os = "macos")]
    let target_platform = "x86_64-apple-darwin";
    #[cfg(target_os = "linux")]
    let target_platform = "x86_64-unknown-linux-gnu";
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    bail!("libfuzzer-sys only supports Linux and macOS");

    let mut rust_flags = env::var("RUSTFLAGS").unwrap_or_default();
    rust_flags.push_str(
        " --cfg fuzzing \
         -C passes=sancov \
         -C llvm-args=-sanitizer-coverage-level=4 \
         -C llvm-args=-sanitizer-coverage-trace-pc-guard \
         -C llvm-args=-sanitizer-coverage-prune-blocks=0 \
         -C debug-assertions=on \
         -C debuginfo=0 \
         -C opt-level=3 ",
    );

    let mut asan_options = env::var("ASAN_OPTIONS").unwrap_or_default();
    asan_options.push_str(" detect_odr_violation=0 ");

    let dir = env::current_dir()?.join("fuzzer-libfuzzer");

    let corpus_dir = dir.join(&format!("corpus-{}", target));
    fs::create_dir_all(&corpus_dir)?;

    let fuzzer = Command::new("cargo")
        .args(&["run", "--target", &target_platform, "--bin", &target, "--"])
        .arg(&corpus_dir)
        .arg(&seed_dir)
        .env("RUSTFLAGS", &rust_flags)
        .env("ASAN_OPTIONS", &asan_options)
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer.success(), "libfuzzer quit with code {}", fuzzer);
    Ok(())
}

#[allow(deprecated)]
arg_enum!{
    #[derive(StructOpt, Debug)]
    enum Fuzzer {
        Afl,
        Honggfuzz,
        Libfuzzer
    }
}
