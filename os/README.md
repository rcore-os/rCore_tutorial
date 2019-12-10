对应rCore_tutorial_doc的cha7-pa4

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
switch satp from 0x800000000008021a to 0x8000000000080a30
++++ setup memory!    ++++
++++ setup process!   ++++
++++ setup timer!     ++++

>>>> will switch_to thread 0 in idle_main!
begin of thread 0
000000......0000000000
end  of thread 0
thread 0 exited, exit code = 0

<<<< switch_back to idle in idle_main!
......
```