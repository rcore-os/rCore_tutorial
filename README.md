# rCore_tutorial

[![Actions Status](https://github.com/rcore-os/rCore_tutorial/workflows/CI/badge.svg)](https://github.com/rcore-os/rCore_tutorial/actions)

## WARNING

This project is *no longer maintained*, please try [Tutorial v3.5](https://github.com/rcore-os/rCore-Tutorial-v3).

## Documentations

Please read
- https://rcore-os.github.io/rCore_tutorial_doc/
    - [Docs Repository](https://github.com/rcore-os/rCore_tutorial_doc)
- https://github.com/rcore-os/rCore/wiki/os-tutorial-os2atc

## Prerequisite

You need: `rustup` installed, ensure `~/.cargo/bin` is added to PATH and run:

```shell
make env
```

## Quick Try

```shell
$ make run
## If everything is OK, then you will see below info：
......
OpenSBI v0.4 (Jul  2 2019 11:53:53)
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 112 KB
Runtime SBI Version    : 0.1

PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
switch satp from 0x8000000000080256 to 0x800000000008119f
++++ setup memory!    ++++
++++ setup interrupt! ++++
available programs in rust/ are:
  .
  ..
  user_shell
  notebook
  greenthread
  hello_world
  model
++++ setup fs!        ++++
++++ setup process!   ++++
++++ setup timer!     ++++
Rust user shell
>>
```
