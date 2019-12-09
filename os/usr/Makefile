target := riscv64
mode := debug
rust_src_dir := rust/src/bin
rust_target_dir := rust/target/$(target)-rust/$(mode)
rust_srcs := $(wildcard $(rust_src_dir)/*.rs)
rust_targets := $(patsubst $(rust_src_dir)/%.rs, $(rust_target_dir)/%, $(rust_srcs))
out_dir := build/$(target)
sfsimg := build/$(target).img
.PHONY: rcore-fs-fuse rust user_img clean


rcore-fs-fuse:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs --rev c611248
endif

rust:
	@cd rust && cargo xbuild --target $(target)-rust.json
	@echo targets includes $(rust_targets)
	@rm -rf $(out_dir)/rust && mkdir -p $(out_dir)/rust
	@rm -f $(sfsimg)
	@cp $(rust_targets) $(out_dir)/rust

$(sfsimg): rcore-fs-fuse rust
	@rcore-fs-fuse --fs sfs $@ $(out_dir) zip 

user_img: $(sfsimg)

clean:
	@rm -rf build/
