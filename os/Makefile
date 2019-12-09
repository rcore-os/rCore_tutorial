target := riscv64-os
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

.PHONY: kernel build clean qemu run

kernel:
	@cargo xbuild --target $(target).json
$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $(bin)
build: $(bin)
clean:
	@rm -r target/
qemu: build
	@qemu-system-riscv64 \
        --machine virt \
        --nographic \
        --bios opensbi/opensbi_rv64.elf \
        --device loader,file=$(bin),addr=0x80200000
run: qemu
