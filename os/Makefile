target := riscv64-os
mode := debug
kernel := target/$(target)/$(mode)/os
kernel_asm := target/$(target)/$(mode)/os.asm
bin := target/$(target)/$(mode)/kernel.bin
xbuild_args := --target $(target).json

.PHONY: kernel objdump build qemu run clean

export USER_IMG = usr/build/riscv64.img

kernel:
	@cargo xbuild --target $(target).json
objdump: kernel
	@riscv64-unknown-elf-objdump -d $(kernel) > $(kernel_asm)
	@vim $(kernel_asm)

$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $@
build: $(bin)
qemu: 
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-kernel opensbi/opensbi_rv64.elf \
		-device loader,file=$(bin),addr=0x80200000
run: build qemu
clean:
	@rm -r target/
