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

use failure::{Error, ResultExt};
use regex::Regex;
use structopt::StructOpt;

/// Run fuzzer collection
#[derive(StructOpt, Debug)]
enum Cli {
    /// Run all fuzz targets
    #[structopt(name = "continuously")]
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
        // Run `cargo update` between cycles
        #[structopt(long = "cargo-update")]
        cargo_update: bool,
    },
    /// Run one target with specific fuzzer
    #[structopt(name = "target")]
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
        for cause in e.causes().skip(1) {
            eprintln!("caused by: {}", cause);
        }
        ::std::process::exit(1);
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
            let targets = get_targets()?;
            if targets.iter().find(|x| *x == &target).is_none() {
                bail!(
                    "Don't know target `{}`. {}",
                    target,
                    if let Some(alt) = did_you_mean(&target, &targets) {
                        format!("Did you mean `{}`?", alt)
                    } else {
                        "".into()
                    }
                );
            }

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
            cargo_update,
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
            let targets = targets
                .iter()
                .filter(|x| filter.as_ref().map(|f| x.contains(f)).unwrap_or(true));

            'cycle: loop {
                'targets_pass: for target in targets.clone() {
                    if let Err(e) = run(target) {
                        match e.downcast::<FuzzerQuit>() {
                            Ok(_) => {
                                println!("Fuzzer failed so we'll continue with the next one");
                                continue 'targets_pass;
                            }
                            Err(other_error) => Err(other_error)?,
                        }
                    }
                }

                if !infinite {
                    break 'cycle;
                }

                if cargo_update {
                    run_cargo_update()?;
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
    fs::create_dir_all(&seed_dir).context(format!("unable to create seed dir for {}", target))?;
    Ok(seed_dir)
}

fn create_corpus_dir(base: &Path, target: &str) -> Result<PathBuf, Error> {
    let corpus_dir = base.join(&format!("corpus-{}", target));
    fs::create_dir_all(&corpus_dir).context(format!(
        "unable to create corpus dir for {}{}",
        base.display(),
        target
    ))?;
    Ok(corpus_dir)
}

fn get_targets() -> Result<Vec<String>, Error> {
    let source = common_dir()?.join("src/lib.rs");
    let targets_rs = fs::read_to_string(&source).context(format!("unable to read {:?}", source))?;
    let match_fuzz_fs = Regex::new(r"pub fn fuzz_(\w+)\(")?;
    let target_names = match_fuzz_fs
        .captures_iter(&targets_rs)
        .map(|x| x[1].to_string());
    Ok(target_names.collect())
}

fn run_cargo_update() -> Result<(), Error> {
    let run = Command::new("cargo")
        .arg("update")
        .spawn()
        .context("error starting `cargo update`")?
        .wait()
        .context("error running `cargo update`")?;

    ensure!(
        run.success(),
        "error running `cargo update`: Exited with {:?}",
        run.code()
    );
    Ok(())
}

#[derive(Fail, Debug)]
#[fail(display = "Fuzzer quit")]
pub struct FuzzerQuit;

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
        .spawn()
        .context(format!("error starting {:?} to runn {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
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
        .spawn()
        .context(format!("error starting {:?} to runn {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
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

    let mut asan_options = env::var("ASAN_OPTIONS").unwrap_or_default();
    asan_options.push_str(" detect_odr_violation=0 ");

    let max_time = if let Some(timeout) = timeout {
        format!("-max_total_time={}", timeout)
    } else {
        "".into()
    };

    let fuzzer_bin = Command::new("cargo")
        .args(&[
            "run",
            "--target",
            &target_platform,
            "--bin",
            &target,
            "--",
            &max_time,
        ])
        .arg(&corpus_dir)
        .arg(&seed_dir)
        .env("RUSTFLAGS", &rust_flags)
        .env("ASAN_OPTIONS", &asan_options)
        .current_dir(&dir)
        .spawn()
        .context(format!("error starting {:?} to runn {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
    Ok(())
}

fn write_fuzzer_target(fuzzer: Fuzzer, target: &str) -> Result<(), Error> {
    use std::io::Write;

    let template_path = fuzzer.dir()?.join("template.rs");
    let template = fs::read_to_string(&template_path).context(format!(
        "error reading template file {}",
        template_path.display()
    ))?;

    let target_dir = fuzzer.dir()?.join("src").join("bin");
    fs::create_dir_all(&target_dir).context(format!(
        "error creating fuzz target dir {}",
        target_dir.display()
    ))?;
    let path = target_dir.join(&format!("{}.rs", target));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .context(format!(
            "error writing fuzz target binary {}",
            path.display()
        ))?;

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
        let cwd = env::current_dir().context("error getting current directory")?;

        use Fuzzer::*;
        let p = match self {
            Afl => cwd.join("fuzzer-afl"),
            Honggfuzz => cwd.join("fuzzer-honggfuzz"),
            Libfuzzer => cwd.join("fuzzer-libfuzzer"),
        };

        Ok(p)
    }
}

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
///
/// Originally from [clap] which is Copyright (c) 2015-2016 Kevin B. Knapp
///
/// [clap]: https://github.com/kbknapp/clap-rs/blob/dc7ae65fb784dc355d56f09554f1216b22755c3e/src/suggestions.rs
pub fn did_you_mean<'a, T: ?Sized, I>(v: &str, possible_values: I) -> Option<&'a str>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    extern crate strsim;

    let mut candidate: Option<(f64, &str)> = None;
    for pv in possible_values {
        let confidence = strsim::jaro_winkler(v, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    match candidate {
        None => None,
        Some((_, candidate)) => Some(candidate),
    }
}
