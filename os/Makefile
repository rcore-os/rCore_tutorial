target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run env

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview
	rustup target add $(target)

export USER_IMG = ../usr/build/riscv64.img

kernel:
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm:
	$(objdump) -d $(kernel) | less

build: $(bin)

clean:
	cargo clean

qemu: build
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-kernel $(bin)

run: build qemu
