对应rCore_tutorial_doc的ch8-pa4

# 建立开发环境
第一次编译需要先建立开发环境
```
make env
```

执行如下命令通过对最小独立内核的编译和用户程序
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
0000000......0000000000
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 1 in idle_main!
begin of thread 1
11111111......11111
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 2 in idle_main!
begin of thread 2
222222......2222
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 3 in idle_main!
begin of thread 3
333333......3333
<<<< switch_back to idle in idle_main!
......
```
