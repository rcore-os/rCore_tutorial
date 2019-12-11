DOCKER_NAME ?= rcoretutorial
.PHONY: docker build_docker

docker:
	docker run -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}

build_docker: qemu-4.1.1.tar.xz riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz
	docker build -t ${DOCKER_NAME} .
	rm qemu-4.1.1.tar.xz
	rm riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz

qemu-4.1.1.tar.xz:
	wget https://download.qemu.org/qemu-4.1.1.tar.xz

riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz:
	wget https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz
