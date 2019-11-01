target := riscv64-os
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

kernel:
	@cargo xbuild --target $(target).json
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
