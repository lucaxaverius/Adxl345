savedcmd_rust/libmacros.so := rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --emit=dep-info=rust/.libmacros.so.d --emit=link=rust/libmacros.so --extern proc_macro --crate-type proc-macro --crate-name macros rust/macros/lib.rs

source_rust/libmacros.so := rust/macros/lib.rs

deps_rust/libmacros.so := \
  rust/macros/concat_idents.rs \
  rust/macros/helpers.rs \
  rust/macros/module.rs \
    $(wildcard include/config/HAVE_ARCH_PREL32_RELOCATIONS) \
  rust/macros/vtable.rs \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0d91c78a7710ed2e.so \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0d91c78a7710ed2e.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fd3918c72578db43.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2a597573799b576f.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-3fff6412017c0b89.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-65fb576691133eee.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-54c30397d4b33d3b.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-27b55c02caca49ea.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-579b26075cbe9eca.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-fcbb01769e88af40.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f69e84994e245fea.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-e9de5d8ee4e7a3dd.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-9023252e4f119830.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-fbde6acb28b510ca.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-84091f15e468b1ee.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-69d2770595dc6161.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-b156a020cc470e38.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-7dd2d47ddb5fff81.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-18d5ce8e8a320b85.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-9bfeb974ba4dc4e7.rlib \
  /home/rustxave/.rustup/toolchains/1.71.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-7ede36ba902b7649.rlib \

rust/libmacros.so: $(deps_rust/libmacros.so)

$(deps_rust/libmacros.so):
