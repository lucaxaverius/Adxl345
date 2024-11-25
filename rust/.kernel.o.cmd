savedcmd_rust/kernel.o := OBJTREE=/home/rustxave/Scrivania/OS-Linux/linux rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --target=./scripts/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cdebuginfo=2 @./include/generated/rustc_cfg --extern alloc --extern build_error --extern macros --extern bindings --emit=dep-info=rust/.kernel.o.d --emit=obj=rust/kernel.o --emit=metadata=rust/libkernel.rmeta --crate-type rlib -L./rust --crate-name kernel rust/kernel/lib.rs 

source_rust/kernel.o := rust/kernel/lib.rs

deps_rust/kernel.o := \
    $(wildcard include/config/RUST) \
    $(wildcard include/config/ARM_AMBA) \
    $(wildcard include/config/COMMON_CLK) \
    $(wildcard include/config/NET) \
    $(wildcard include/config/SYSCTL) \
    $(wildcard include/config/HAS_IOMEM) \
    $(wildcard include/config/KUNIT) \
    $(wildcard include/config/SYSFS) \
  rust/kernel/allocator.rs \
  rust/kernel/build_assert.rs \
  rust/kernel/error.rs \
    $(wildcard include/config/ARM) \
  rust/kernel/prelude.rs \
  rust/kernel/print.rs \
    $(wildcard include/config/PRINTK) \
  rust/kernel/static_assert.rs \
  rust/kernel/std_vendor.rs \
  rust/kernel/str.rs \
  rust/kernel/sync.rs \
  rust/kernel/sync/arc.rs \
  rust/kernel/sync/condvar.rs \
  rust/kernel/sync/guard.rs \
  rust/kernel/sync/locked_by.rs \
  rust/kernel/sync/mutex.rs \
  rust/kernel/sync/nowait.rs \
  rust/kernel/sync/rcu.rs \
  rust/kernel/sync/revocable.rs \
  rust/kernel/sync/rwsem.rs \
  rust/kernel/sync/seqlock.rs \
  rust/kernel/sync/smutex.rs \
  rust/kernel/sync/spinlock.rs \
  rust/kernel/types.rs \
  rust/kernel/chrdev.rs \
  rust/kernel/clk.rs \
  rust/kernel/cred.rs \
  rust/kernel/delay.rs \
  rust/kernel/device.rs \
  rust/kernel/driver.rs \
  rust/kernel/file.rs \
  rust/kernel/fs.rs \
    $(wildcard include/config/QUOTA) \
  rust/kernel/fs/param.rs \
  rust/kernel/gpio.rs \
    $(wildcard include/config/GPIOLIB_IRQCHIP) \
  rust/kernel/hwrng.rs \
  rust/kernel/irq.rs \
    $(wildcard include/config/IRQ_DOMAIN) \
  rust/kernel/kasync.rs \
  rust/kernel/kasync/executor.rs \
  rust/kernel/kasync/executor/workqueue.rs \
  rust/kernel/kasync/net.rs \
  rust/kernel/miscdev.rs \
  rust/kernel/mm.rs \
  rust/kernel/net.rs \
    $(wildcard include/config/NETFILTER) \
  rust/kernel/net/filter.rs \
  rust/kernel/pages.rs \
  rust/kernel/power.rs \
  rust/kernel/revocable.rs \
  rust/kernel/security.rs \
  rust/kernel/task.rs \
  rust/kernel/workqueue.rs \
  rust/kernel/i2c.rs \
  rust/kernel/i2c/msg.rs \
  rust/kernel/i2c/adapter.rs \
  rust/kernel/i2c/board_info.rs \
  rust/kernel/i2c/device_id.rs \
  rust/kernel/i2c/client.rs \
  rust/kernel/i2c/driver.rs \
  rust/kernel/i2c/i2c_macros.rs \
  rust/kernel/i2c/utils.rs \
  rust/kernel/linked_list.rs \
  rust/kernel/raw_list.rs \
  rust/kernel/rbtree.rs \
  rust/kernel/unsafe_list.rs \
  rust/kernel/module_param.rs \
  rust/kernel/random.rs \
  rust/kernel/sysctl.rs \
  rust/kernel/io_buffer.rs \
  rust/kernel/io_mem.rs \
    $(wildcard include/config/64BIT) \
  rust/kernel/iov_iter.rs \
  rust/kernel/of.rs \
  rust/kernel/platform.rs \
  rust/kernel/user_ptr.rs \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcore.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libcompiler_builtins.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/liballoc.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libmacros.so \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libbindings.rmeta \
  /home/rustxave/Scrivania/OS-Linux/linux/rust/libbuild_error.rmeta \

rust/kernel.o: $(deps_rust/kernel.o)

$(deps_rust/kernel.o):
