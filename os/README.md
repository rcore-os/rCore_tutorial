对应rCore_tutorial_doc的cha4-pa1

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
++++ setup interrupt! ++++
++++ setup timer!     ++++
a breakpoint set @0x8020002c
panicked at 'end of rust_main', src/init.rs:11:5
* 100 ticks *
* 100 ticks *

```