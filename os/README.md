对应rCore_tutorial_doc的cha6-pa4

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
switch satp from 0x8000000000080215 to 0x8000000000080a2b
++++ setup memory!    ++++
I'm leaving soon, but I still want to say: Hello world!
switched back from temp_thread!
```