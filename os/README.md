对应rCore_tutorial_doc的cha5-pa2

# 建立开发环境
第一次编译需要先建立开发环境
```
make env
```

执行如下命令通过对最小独立内核的编译
```
make build
```

执行如下命令在rv模拟器上运行最小独立内核，
```
make run
```
显示如下字符串
```
OpenSBI v0.5 (Oct  9 2019 12:03:04)
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
Firmware Size          : 116 KB
Runtime SBI Version    : 0.2

PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
free physical memory paddr = [0x80a22140, 0x88000000)
free physical memory ppn = [0x80a23, 0x88000)
++++ setup interrupt! ++++
++++ setup timer!     ++++
++++ setup memory!    ++++
heap_value assertion successfully!
heap_value is at 0xffffffffc0a10000
heap_value is in section .bss!
vec assertion successfully!
vec is at 0xffffffffc0a11000
vec is in section .bss!
* 100 ticks *
* 100 ticks *
```