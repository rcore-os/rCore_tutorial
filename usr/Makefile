target := riscv64imac-unknown-none-elf
mode := debug
rust_src_dir := rust/src/bin
rust_target_dir := rust/target/$(target)/$(mode)
rust_srcs := $(wildcard $(rust_src_dir)/*.rs)
rust_targets := $(patsubst $(rust_src_dir)/%.rs, $(rust_target_dir)/%, $(rust_srcs))
out_dir := build/riscv64
sfsimg := build/riscv64.img
.PHONY: rcore-fs-fuse rust user_img clean


rcore-fs-fuse:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs --rev d8d61190
endif

rust:
	@cd rust && cargo build
	@echo targets includes $(rust_targets)
	@rm -rf $(out_dir)/rust && mkdir -p $(out_dir)/rust
	@rm -f $(sfsimg)
	@cp $(rust_targets) $(out_dir)/rust

$(sfsimg): rcore-fs-fuse rust
	@dd if=/dev/zero of=$(out_dir)/temp bs=1k count=2
	@rcore-fs-fuse --fs sfs $@ $(out_dir) zip 

user_img: $(sfsimg)

clean:
	@rm -rf build/
