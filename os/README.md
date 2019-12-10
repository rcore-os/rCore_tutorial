对应rCore_tutorial_doc的ch9-pa1

# 建立开发环境
第一次编译需要先建立开发环境
```
make env
```
执行如下命令通过对用户执行程序的编译和文件系统的生成
```
cd ../usr
make user_img
cd ../os
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
switch satp from 0x8000000000080252 to 0x800000000008100d
++++ setup memory!    ++++
available programs in rust/ are:
  .
  ..
  user_shell
  notebook
  hello_world
  model
++++ setup fs!        ++++
it really a executable!
++++ setup process!   ++++
++++ setup timer!     ++++

>>>> will switch_to thread 0 in idle_main!
begin of thread 0
00000...00000
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 1 in idle_main!
begin of thread 1
111...1111111
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 2 in idle_main!
begin of thread 2
222...22222
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 3 in idle_main!
begin of thread 3
333...333333
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 4 in idle_main!
begin of thread 4
444444...44444444
<<<< switch_back to idle in idle_main!

>>>> will switch_to thread 5 in idle_main!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
Hello world! from user mode program!
thread 5 exited, exit code = 0

<<<< switch_back to idle in idle_main!

......
```
