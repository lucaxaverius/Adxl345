savedcmd_rust/build_error.o := OBJTREE=/home/rustxave/Scrivania/OS-Linux/linux rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --target=./scripts/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cdebuginfo=2 @./include/generated/rustc_cfg --emit=dep-info=rust/.build_error.o.d --emit=obj=rust/build_error.o --emit=metadata=rust/libbuild_error.rmeta --crate-type rlib -L./rust --crate-name build_error rust/build_error.rs 

source_rust/build_error.o := rust/build_error.rs

deps_rust/build_error.o := \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcore.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcompiler_builtins.rmeta \

rust/build_error.o: $(deps_rust/build_error.o)

$(deps_rust/build_error.o):
