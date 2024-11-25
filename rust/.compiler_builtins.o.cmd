savedcmd_rust/compiler_builtins.o := OBJTREE=/home/rustxave/Scrivania/OS-Linux/linux rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --target=./scripts/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cdebuginfo=2 @./include/generated/rustc_cfg --emit=dep-info=rust/.compiler_builtins.o.d --emit=obj=rust/compiler_builtins.o --emit=metadata=rust/libcompiler_builtins.rmeta --crate-type rlib -L./rust --crate-name compiler_builtins rust/compiler_builtins.rs ;llvm-objcopy -w -W '__*' rust/compiler_builtins.o

source_rust/compiler_builtins.o := rust/compiler_builtins.rs

deps_rust/compiler_builtins.o := \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcore.rmeta \

rust/compiler_builtins.o: $(deps_rust/compiler_builtins.o)

$(deps_rust/compiler_builtins.o):
