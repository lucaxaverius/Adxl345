savedcmd_rust/built-in.a := rm -f rust/built-in.a;  printf "rust/%s " core.o compiler_builtins.o helpers.o alloc.o bindings.o kernel.o exports.o | xargs llvm-ar cDPrST rust/built-in.a
