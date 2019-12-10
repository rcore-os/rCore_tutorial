对应rCore_tutorial_doc的ch9-pa2

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
switch satp from 0x8000000000080256 to 0x8000000000080eb1
++++ setup memory!    ++++
++++ setup interrupt! ++++
available programs in rust/ are:
  .
  ..
  notebook
  hello_world
  model
++++ setup fs!        ++++
it really a executable!
++++ setup process!   ++++
++++ setup timer!     ++++
Welcome to notebook!

......
```
