FROM buildpack-deps:xenial

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly-2019-12-08; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# 以上部分来自https://hub.docker.com/r/rustlang/rust/dockerfile

# install QEMU
ADD qemu-4.1.1.tar.xz .
RUN cd qemu-4.1.1 \
    && ./configure --target-list=riscv64-softmmu \
    && make -j \
	&& make install \
	&& cd .. \
	&& rm qemu-4.1.1 -r

# riscv gcc needed by rustc
ADD riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz .
ENV PATH=$PWD/riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14/bin:$PATH

# install others
RUN apt update \
    && apt install less device-tree-compiler -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# install Rust tools
RUN echo '[source.crates-io]' >> $CARGO_HOME/config \
    && echo 'replace-with = \047ustc\047' >> $CARGO_HOME/config \
    && echo '[source.ustc]' >> $CARGO_HOME/config \
    && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"' >> $CARGO_HOME/config \
	&& cargo install cargo-binutils cargo-xbuild \
    && rustup component add rust-src \
    && rustup component add llvm-tools-preview \
    && rustup target add riscv64imac-unknown-none-elf
