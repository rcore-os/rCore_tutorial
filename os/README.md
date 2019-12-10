对应rCore_tutorial_doc的cha4-pa2

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
++++ setup memory!    ++++
heap_value assertion successfully!
heap_value is at 0x80a10000
heap_value is in section .bss!
vec assertion successfully!
vec is at 0x80211000
vec is in section .bss!
* 100 ticks *git 
* 100 ticks *
```