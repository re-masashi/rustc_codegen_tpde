#![warn(rust_2018_idioms)]
#![warn(unused_lifetimes)]
#![warn(unreachable_pub)]

use std::path::PathBuf;
use std::{env, process};

use clap::{Parser, Subcommand, ValueEnum};

use self::utils::Compiler;

mod abi_cafe;
mod bench;
mod build_backend;
mod build_sysroot;
mod config;
mod path;
mod prepare;
mod rustc_info;
mod shared_utils;
mod tests;
mod utils;

/// The build system of rustc_codegen_tpde.
#[derive(Parser)]
#[command(about, long_about = None)]
struct BuildSystem {
    #[command(subcommand)]
    command: Command,
    /// Specify the directory in which the download, build and dist directories are stored.
    #[arg(default_value = ".", long)]
    out_dir: String,
    /// Specify the directory in which the download directory is stored. Overrides --out-dir.
    #[arg(long)]
    download_dir: Option<String>,
    /// Require Cargo.lock and cache are up to date.
    #[arg(default_value_t = false, long)]
    frozen: bool,
}

#[derive(PartialEq, Debug, Subcommand)]
enum Command {
    /// Download required files for testing.
    Prepare,
    /// Build codegen backend dylib.
    Build {
        /// Which sysroot libraries to use.
        #[arg(value_enum, long = "sysroot", default_value_t = SysrootKind::Tpde)]
        sysroot_kind: SysrootKind,
    },
    /// Run tests.
    Test {
        /// Which sysroot libraries to use.
        #[arg(value_enum, long = "sysroot", default_value_t = SysrootKind::Tpde)]
        sysroot_kind: SysrootKind,
        /// Skip testing the TESTNAME test. The test name format is the same as config.txt.
        skip_test: Vec<String>,
    },
    /// Test ABI compatibility of rustc_codegen_tpde.
    AbiCafe {
        /// Which sysroot libraries to use.
        #[arg(value_enum, long = "sysroot", default_value_t = SysrootKind::Tpde)]
        sysroot_kind: SysrootKind,
    },
    /// Test compile and runtime performance of different codegen backends.
    Bench {
        /// Which sysroot libraries to use.
        #[arg(value_enum, long = "sysroot", default_value_t = SysrootKind::Tpde)]
        sysroot_kind: SysrootKind,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
enum SysrootKind {
    None,
    Tpde,
    Llvm,
}

#[derive(Clone, Debug)]
struct CodegenBackend(PathBuf);

fn main() {
    if env::var_os("RUST_BACKTRACE").is_none() {
        unsafe {
            env::set_var("RUST_BACKTRACE", "1");
        }
    }

    // Force incr comp even in release mode unless in CI or incremental builds are explicitly disabled
    if env::var_os("CARGO_BUILD_INCREMENTAL").is_none() {
        unsafe {
            env::set_var("CARGO_BUILD_INCREMENTAL", "true");
        }
    }

    let cli = BuildSystem::parse();
    let current_dir = std::env::current_dir().unwrap();
    let out_dir = current_dir.join(cli.out_dir);
    let download_dir = cli
        .download_dir
        .map(|dir| current_dir.join(dir))
        .unwrap_or_else(|| out_dir.join("download"));

    if cli.command == Command::Prepare {
        prepare::prepare(&path::Dirs {
            source_dir: current_dir.clone(),
            download_dir,
            build_dir: PathBuf::from("dummy_do_not_use"),
            dist_dir: PathBuf::from("dummy_do_not_use"),
            frozen: cli.frozen,
        });
        process::exit(0);
    }

    let rustup_toolchain_name = match (env::var("CARGO"), env::var("RUSTC"), env::var("RUSTDOC")) {
        (Ok(_), Ok(_), Ok(_)) => None,
        (_, Err(_), Err(_)) => Some(rustc_info::get_toolchain_name()),
        vars => {
            eprintln!(
                "If RUSTC or RUSTDOC is set, both need to be set and in addition CARGO needs to be set: {vars:?}"
            );
            process::exit(1);
        }
    };
    let bootstrap_host_compiler = {
        let cargo = rustc_info::get_cargo_path();
        let rustc = rustc_info::get_rustc_path();
        let rustdoc = rustc_info::get_rustdoc_path();
        let triple =
            std::env::var("HOST_TRIPLE").unwrap_or_else(|_| rustc_info::get_host_triple(&rustc));
        Compiler {
            cargo,
            rustc,
            rustdoc,
            rustflags: vec![],
            rustdocflags: vec![],
            triple,
            runner: vec![],
        }
    };
    let target_triple =
        std::env::var("TARGET_TRIPLE").unwrap_or_else(|_| bootstrap_host_compiler.triple.clone());

    let dirs = path::Dirs {
        source_dir: current_dir.clone(),
        download_dir,
        build_dir: out_dir.join("build"),
        dist_dir: out_dir.join("dist"),
        frozen: cli.frozen,
    };

    std::fs::create_dir_all(&dirs.build_dir).unwrap();

    {
        // Make sure we always explicitly specify the target dir
        let target = dirs.build_dir.join("target_dir_should_be_set_explicitly");
        unsafe {
            env::set_var("CARGO_TARGET_DIR", &target);
        }
        let _ = std::fs::remove_file(&target);
        std::fs::File::create(target).unwrap();
    }

    unsafe {
        env::set_var("RUSTC", "rustc_should_be_set_explicitly");
        env::set_var("RUSTDOC", "rustdoc_should_be_set_explicitly");
    }

    let cg_tpde_dylib = CodegenBackend(build_backend::build_backend(
        &dirs,
        &bootstrap_host_compiler,
        matches!(cli.command, Command::Test { sysroot_kind: _, skip_test: _ }),
    ));
    match cli.command {
        Command::Prepare => {
            // Handled above
        }
        Command::Test { sysroot_kind, skip_test } => {
            tests::run_tests(
                &dirs,
                sysroot_kind,
                &skip_test.iter().map(|test| &**test).collect::<Vec<_>>(),
                &cg_tpde_dylib,
                &bootstrap_host_compiler,
                rustup_toolchain_name.as_deref(),
                target_triple.clone(),
            );
        }
        Command::AbiCafe { sysroot_kind } => {
            if bootstrap_host_compiler.triple != target_triple {
                eprintln!("Abi-cafe doesn't support cross-compilation");
                process::exit(1);
            }
            abi_cafe::run(
                sysroot_kind,
                &dirs,
                &cg_tpde_dylib,
                rustup_toolchain_name.as_deref(),
                &bootstrap_host_compiler,
            );
        }
        Command::Build { sysroot_kind } => {
            build_sysroot::build_sysroot(
                &dirs,
                sysroot_kind,
                &cg_tpde_dylib,
                &bootstrap_host_compiler,
                rustup_toolchain_name.as_deref(),
                target_triple,
            );
        }
        Command::Bench { sysroot_kind } => {
            let compiler = build_sysroot::build_sysroot(
                &dirs,
                sysroot_kind,
                &cg_tpde_dylib,
                &bootstrap_host_compiler,
                rustup_toolchain_name.as_deref(),
                target_triple,
            );
            bench::benchmark(&dirs, &compiler);
        }
    }
}
