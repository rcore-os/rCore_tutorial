DOCKER_NAME ?= panqinglin/tutorial
.PHONY: docker build_docker
all:
	make -C usr user_img
	make -C os build
run:
	make -C usr user_img
	make -C os run
clean:
	make -C usr clean
	make -C os clean
env:
	make -C os env
docker:
	docker run --rm -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}

build_docker: qemu-4.1.1.tar.xz riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz
	docker build -t ${DOCKER_NAME} .
	rm qemu-4.1.1.tar.xz
	rm riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz

qemu-4.1.1.tar.xz:
	wget https://download.qemu.org/qemu-4.1.1.tar.xz

riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz:
	wget https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz

