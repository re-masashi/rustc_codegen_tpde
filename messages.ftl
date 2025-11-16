codegen_tpde_autodiff_without_enable = using the autodiff feature requires -Z autodiff=Enable

codegen_tpde_copy_bitcode = failed to copy bitcode to object file: {$err}


codegen_tpde_fixed_x18_invalid_arch = the `-Zfixed-x18` flag is not supported on the `{$arch}` architecture

codegen_tpde_from_llvm_diag = {$message}

codegen_tpde_from_llvm_optimization_diag = {$filename}:{$line}:{$column} {$pass_name} ({$kind}): {$message}

codegen_tpde_load_bitcode = failed to load bitcode of module "{$name}"
codegen_tpde_load_bitcode_with_llvm_err = failed to load bitcode of module "{$name}": {$llvm_err}

codegen_tpde_lto_bitcode_from_rlib = failed to get bitcode from object file for LTO ({$err})

codegen_tpde_mismatch_data_layout =
    data-layout for target `{$rustc_target}`, `{$rustc_layout}`, differs from LLVM target's `{$llvm_target}` default layout, `{$llvm_layout}`

codegen_tpde_parse_bitcode = failed to parse bitcode for LTO module
codegen_tpde_parse_bitcode_with_llvm_err = failed to parse bitcode for LTO module: {$llvm_err}

codegen_tpde_parse_target_machine_config =
    failed to parse target machine config to target machine: {$error}

codegen_tpde_prepare_autodiff = failed to prepare autodiff: src: {$src}, target: {$target}, {$error}
codegen_tpde_prepare_autodiff_with_llvm_err = failed to prepare autodiff: {$llvm_err}, src: {$src}, target: {$target}, {$error}
codegen_tpde_prepare_thin_lto_context = failed to prepare thin LTO context
codegen_tpde_prepare_thin_lto_context_with_llvm_err = failed to prepare thin LTO context: {$llvm_err}

codegen_tpde_prepare_thin_lto_module = failed to prepare thin LTO module
codegen_tpde_prepare_thin_lto_module_with_llvm_err = failed to prepare thin LTO module: {$llvm_err}

codegen_tpde_run_passes = failed to run LLVM passes
codegen_tpde_run_passes_with_llvm_err = failed to run LLVM passes: {$llvm_err}

codegen_tpde_sanitizer_kcfi_arity_requires_llvm_21_0_0 = `-Zsanitizer-kcfi-arity` requires LLVM 21.0.0 or later.

codegen_tpde_sanitizer_memtag_requires_mte =
    `-Zsanitizer=memtag` requires `-Ctarget-feature=+mte`

codegen_tpde_serialize_module = failed to serialize module {$name}
codegen_tpde_serialize_module_with_llvm_err = failed to serialize module {$name}: {$llvm_err}

codegen_tpde_symbol_already_defined =
    symbol `{$symbol_name}` is already defined

codegen_tpde_target_machine = could not create LLVM TargetMachine for triple: {$triple}
codegen_tpde_target_machine_with_llvm_err = could not create LLVM TargetMachine for triple: {$triple}: {$llvm_err}

codegen_tpde_unknown_debuginfo_compression = unknown debuginfo compression algorithm {$algorithm} - will fall back to uncompressed debuginfo

codegen_tpde_write_bytecode = failed to write bytecode to {$path}: {$err}

codegen_tpde_write_ir = failed to write LLVM IR to {$path}
codegen_tpde_write_ir_with_llvm_err = failed to write LLVM IR to {$path}: {$llvm_err}

codegen_tpde_write_output = could not write output to {$path}
codegen_tpde_write_output_with_llvm_err = could not write output to {$path}: {$llvm_err}

codegen_tpde_write_thinlto_key = error while writing ThinLTO key data: {$err}
codegen_tpde_write_thinlto_key_with_llvm_err = error while writing ThinLTO key data: {$err}: {$llvm_err}
