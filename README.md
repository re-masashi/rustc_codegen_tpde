The `codegen` crate contains the code to convert from MIR into LLVM IR,
and then from LLVM IR into machine code. In general it contains code
that runs towards the end of the compilation process.

For more information about how codegen works, see the [rustc dev guide].

[*This is the last commit that compiles. Some changes are WIP.*](https://github.com/re-masashi/rustc_codegen_tpde/tree/6cc178728bf32b25015a11c3c2543fd795302840)

[rustc dev guide]: https://rustc-dev-guide.rust-lang.org/backend/codegen.html

Used @TechnoPorg's [rustc_codegen_tpde](https://codeberg.org/TechnoPorg/rustc_codegen_tpde) as a starting point.
