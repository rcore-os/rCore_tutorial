对应rCore_tutorial_doc的ch3-pa2

# 建立开发环境
第一次编译需要先建立开发环境
```
make env
```

执行如下命令通过对最小独立内核的编译
```
make build
```

执行如下命令在rv模拟器上运行最小独立内核
```
make run 
```
可显示如下信息：
```
++++ setup interrupt! ++++
trap: cause: Exception(Breakpoint), epc: 0x0x80200022
panicked at 'trap handled!', src/interrupt.rs:20:5

```