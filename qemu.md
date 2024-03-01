# qemu简明教程  

本教程只会介绍一些简单的参数设置  
由于过于繁琐 整理水平有限 望海涵  

## 设备类型  
在使用qemu来开启内核的时候  
首先要明确指令架构、不同的指令集  
对于不同指令集对应的qemu是不相同的  

我们可以先下载qemu并且看已经安装了的qemu类型  
```shell
apt install qemu
ls /bin | grep qemu # 查看已经安装了的qemu
```
对于其他特定指令集的qemu  
可以自行下载源码编译下载  

qemu运行需要一个具体的设备  
使用-machine/-M来指定设备  

## 内存大小
使用-m来指定内存大小  
```shell
qemu-system-riscv64 -m 1G ... # 这里省略别的参数 此处就表示虚拟1G的内存  
qemu-system-riscv64 -m help # 这里就会显示一些提示  
# 以下是提示内容  
qemu-system-riscv64: -m help: Parameter 'size' expects a non-negative number below 2^64
Optional suffix k, M, G, T, P or E means kilo-, mega-, giga-, tera-, peta-
and exabytes, respectively.
```

## 映像文件
使用-device来添加映像文件  

## 调试
使用下面参数可以便于使用gdb进行调试  
-s 是快捷方式 -gdb tcp::1234
-S 表示启动的时候冻结CPU  
