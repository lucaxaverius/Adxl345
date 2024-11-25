savedcmd_rust/alloc.o := OBJTREE=/home/rustxave/Scrivania/OS-Linux/linux rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --target=./scripts/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cdebuginfo=2 @./include/generated/rustc_cfg --cfg no_global_oom_handling --cfg no_rc --cfg no_sync --emit=dep-info=rust/.alloc.o.d --emit=obj=rust/alloc.o --emit=metadata=rust/liballoc.rmeta --crate-type rlib -L./rust --crate-name alloc rust/alloc/lib.rs 

source_rust/alloc.o := rust/alloc/lib.rs

deps_rust/alloc.o := \
  rust/alloc/macros.rs \
  rust/alloc/raw_vec.rs \
  rust/alloc/alloc.rs \
  rust/alloc/boxed.rs \
  rust/alloc/boxed/thin.rs \
  rust/alloc/borrow.rs \
  rust/alloc/collections/mod.rs \
  rust/alloc/fmt.rs \
  rust/alloc/slice.rs \
  rust/alloc/str.rs \
  rust/alloc/string.rs \
  rust/alloc/vec/mod.rs \
  rust/alloc/vec/drain_filter.rs \
  rust/alloc/vec/drain.rs \
  rust/alloc/vec/into_iter.rs \
  rust/alloc/vec/is_zero.rs \
  rust/alloc/vec/partial_eq.rs \
  rust/alloc/vec/set_len_on_drop.rs \
  rust/alloc/vec/spec_extend.rs \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcore.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcompiler_builtins.rmeta \

rust/alloc.o: $(deps_rust/alloc.o)

$(deps_rust/alloc.o):
