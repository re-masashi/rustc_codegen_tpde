use std::env;
use std::path::PathBuf;

use pathsearch::find_executable_in_path;

use crate::path::{Dirs, RelPath};
use crate::rustc_info::get_file_name;
use crate::shared_utils::{rustflags_from_config_files, rustflags_from_env, rustflags_to_cmd_env};
use crate::utils::{CargoProject, Compiler, LogGroup};

static CG_TPDE: CargoProject = CargoProject::new(&RelPath::source("."), "cg_tpde");

pub(crate) fn build_backend(
    dirs: &Dirs,
    bootstrap_host_compiler: &Compiler,
    debug: bool,
) -> PathBuf {
    let _group = LogGroup::guard("Build backend");

    let mut cmd = CG_TPDE.build(bootstrap_host_compiler, dirs, "rustc_codegen_tpde");

    let mut rustflags = rustflags_from_env("RUSTFLAGS");
    rustflags
        .extend(rustflags_from_config_files(&bootstrap_host_compiler.triple, &dirs.source_dir));
    rustflags.push("-Zallow-features=assert_matches,extern_types,file_buffered,if_let_guard,impl_trait_in_assoc_type,iter_intersperse,macro_derive,rustc_private,trim_prefix_suffix,try_blocks".to_owned());
    rustflags_to_cmd_env(&mut cmd, "RUSTFLAGS", &rustflags);

    if env::var("CG_TPDE_EXPENSIVE_CHECKS").is_ok() {
        cmd.env("CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS", "true");
        cmd.env("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "true");
    }

    cmd.env("LLVM_CONFIG", find_executable_in_path("llvm-config").expect("llvm-config not found"));
    cmd.env("LLVM_LINK_SHARED", "true");

    if !debug {
        cmd.arg("--release");
    }

    eprintln!("[BUILD] rustc_codegen_tpde");
    crate::utils::spawn_and_wait(cmd);

    CG_TPDE
        .target_dir(dirs)
        .join(&bootstrap_host_compiler.triple)
        .join(if debug { "debug" } else { "release" })
        .join(get_file_name(&bootstrap_host_compiler.rustc, "rustc_codegen_tpde", "dylib"))
}
