# Imposta il percorso dei sorgenti del kernel della BeagleBone Black
KDIR ?= ~/Scrivania/OS-Linux/linux

# Definisci il toolchain per ARM
CROSS_COMPILE ?= arm-linux-gnueabihf-

# Abilita LLVM per Rust
LLVM ?= 1

# Compila il modulo
all:
	$(MAKE) -C $(KDIR) M=$(PWD) ARCH=arm CROSS_COMPILE=$(CROSS_COMPILE) LLVM=$(LLVM) modules

x86:
	$(MAKE) LLVM=1 -C $(KDIR) M=$(PWD) modules


# Rimuove i file generati
clean:
	$(MAKE) -C $(KDIR) M=$(PWD)  clean
