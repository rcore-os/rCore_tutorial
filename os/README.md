对应rCore_tutorial_doc的cha2-pa7

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
_start vaddr = 0x80200000
bootstacktop vaddr = 0x80208000
hello world!
panicked at 'you want to do nothing!', src/init.rs:15:5
```