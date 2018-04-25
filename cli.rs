#![allow(deprecated)]

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate regex;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use failure::Error;
use regex::Regex;
use structopt::StructOpt;

/// Run fuzzer collection
#[derive(StructOpt, Debug)]
enum Cli {
    /// Run all fuzz targets
    #[structopt(name = "continuous")]
    Continuous {
        /// Only run target containing this string
        #[structopt(short = "q", long = "filter")]
        filter: Option<String>,
        /// Set timeout per target
        #[structopt(short = "t", long = "timeout", default_value = "10")]
        timeout: i32,
        // Run until the end of time (or Ctrl+C)
        #[structopt(short = "i", long = "infinite")]
        infinite: bool,
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
        Continuous {
            filter,
            timeout,
            infinite,
            fuzzer,
        } => {
            let run = |target: &str| -> Result<(), Error> {
                use Fuzzer::*;
                match fuzzer {
                    Afl => run_afl(&target, Some(timeout))?,
                    Honggfuzz => run_honggfuzz(&target, Some(timeout))?,
                    Libfuzzer => run_libfuzzer(&target, Some(timeout))?,
                }
                Ok(())
            };

            let targets = get_targets()?;
            let targets = targets.iter()
                .filter(|x| filter.as_ref().map(|f| x.contains(f)).unwrap_or(true));

            if infinite {
                for target in targets.cycle() { run(target)?; }
            } else {
                for target in targets { run(target)?; }
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

fn create_corpus_dir(base: &Path, target: &str) -> Result<PathBuf, Error> {
    let corpus_dir = base.join(&format!("corpus-{}", target));
    fs::create_dir_all(&corpus_dir)?;
    Ok(corpus_dir)
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
    let fuzzer = Fuzzer::Honggfuzz;
    write_fuzzer_target(fuzzer, target)?;
    let dir = fuzzer.dir()?;

    let seed_dir = create_seed_dir(&target)?;

    let args = format!(
        "-f {} \
         --covdir_all hfuzz_workspace/{}/input \
         {} \
         {}",
        seed_dir.to_string_lossy(),
        target,
        if let Some(t) = timeout {
            format!("--run_time {}", t)
        } else {
            "".into()
        },
        env::var("HFUZZ_RUN_ARGS").unwrap_or_default()
    );

    let fuzzer_bin = Command::new("cargo")
        .args(&["hfuzz", "run", &target])
        .env("HFUZZ_RUN_ARGS", &args)
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer_bin.success(), "{} quit with code {}", fuzzer, fuzzer_bin);
    Ok(())
}

fn run_afl(target: &str, _timeout: Option<i32>) -> Result<(), Error> {
    let fuzzer = Fuzzer::Afl;
    write_fuzzer_target(fuzzer, target)?;
    let dir = fuzzer.dir()?;

    let seed_dir = create_seed_dir(&target)?;
    let corpus_dir = create_corpus_dir(&dir, target)?;

    let fuzzer_bin = Command::new("cargo")
        .args(&["afl", "fuzz"])
        .arg("-i")
        .arg(&seed_dir)
        .arg("-o")
        .arg(&corpus_dir)
        .args(&["--", &format!("target/release/{}", target)])
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer_bin.success(), "{} quit with code {}", fuzzer, fuzzer_bin);
    Ok(())
}

fn run_libfuzzer(target: &str, timeout: Option<i32>) -> Result<(), Error> {
    let fuzzer = Fuzzer::Libfuzzer;
    write_fuzzer_target(fuzzer, target)?;
    let dir = fuzzer.dir()?;

    let seed_dir = create_seed_dir(&target)?;
    let corpus_dir = create_corpus_dir(&dir, target)?;

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

    let libfuzzer_args = if let Some(timeout) = timeout {
        format!("{} -max_total_time={}", env::var("LIBFUZZER_ARGS").unwrap_or_default(), timeout)
    } else {
        env::var("LIBFUZZER_ARGS").unwrap_or_default()
    };

    let mut asan_options = env::var("ASAN_OPTIONS").unwrap_or_default();
    asan_options.push_str(" detect_odr_violation=0 ");

    let fuzzer_bin = Command::new("cargo")
        .args(&["run", "--target", &target_platform, "--bin", &target, "--"])
        .arg(&corpus_dir)
        .arg(&seed_dir)
        .env("RUSTFLAGS", &rust_flags)
        .env("ASAN_OPTIONS", &asan_options)
        .env("LIBFUZZER_ARGS", &libfuzzer_args)
        .current_dir(&dir)
        .spawn()?
        .wait()?;

    ensure!(fuzzer_bin.success(), "{} quit with code {}", fuzzer, fuzzer_bin);
    Ok(())
}

fn write_fuzzer_target(fuzzer: Fuzzer, target: &str) -> Result<(), Error> {
    use std::io::Write;

    let template = fs::read_to_string(&fuzzer.dir()?.join("template.rs"))?;
    let path = fuzzer.dir()?.join("src").join("bin").join(&format!("{}.rs", target));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    let source = template.replace("###TARGET###", &target);
    file.write_all(source.as_bytes())?;
    Ok(())
}

arg_enum!{
    #[derive(StructOpt, Debug, Clone, Copy, PartialEq, Eq)]
    enum Fuzzer {
        Afl,
        Honggfuzz,
        Libfuzzer
    }
}

impl Fuzzer {
    fn dir(&self) -> Result<PathBuf, Error> {
        use Fuzzer::*;
        let p = match self {
            Afl => env::current_dir()?.join("fuzzer-afl"),
            Honggfuzz => env::current_dir()?.join("fuzzer-honggfuzz"),
            Libfuzzer => env::current_dir()?.join("fuzzer-libfuzzer"),
        };

        Ok(p)
    }
}
