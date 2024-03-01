# Linux命令简明教程  

本文旨在简述Linux中常用的命令  
方便开发人员在使用Linux的时候能够高效率处理Linux的文件  

## 文件系统组织  
/bin        二进制文件，系统常规命令
/boot       系统启动分区，系统启动时读取的文件
/dev        设备文件
/etc        大多数配置文件
/home       普通用户的家目录
/lib        32位函数库
/lib64      64位库
/media      手动临时挂载点
/mnt        手动临时挂载点
/opt        第三方软件安装位置
/proc       进程信息及硬件信息
/root       临时设备的默认挂载点
/sbin       系统管理命令
/srv        数据
/var        数据
/sys        内核相关信息
/tmp        临时文件
/usr        用户相关设定

## 帮助
面对不认识的命令 或者 想要深入理解的命令  
如果有时间细看 可以使用以下命令  
```shell
[instr] --help
man [instr]
```

一般而言 命令都有内置参数--help  
或者使用man命令来查看指令的用法  

## 切换目录  
使用cd指令切换到指定目录  
```shell
cd [dir]
cd -
```
cd - 表示切换到上一个所在的目录  

## 查看目录  
```shell
ls # 显示当前目录文件名 和 文件夹名
ls -a # 也可以使用 la , 在ls的基础上，还把隐藏的文件显示出来  
ls -l # 也可以使用 ll , 在la的基础上，还显示文件的权限  
```

## 创建目录
```shell
mkdir [dir] # 创建一个路径为dir的目录 如果只有名字，则是在当前目录创建
```

## 删除目录或者文件
```shell
rm [file] # 删除路径为file的文件
rm -r [dir] # 删除路径为dir的目录
rm -f [file] # 强制删除file
rm -rf [dir] # 强制删除目录  
```

## 移动或者改名
Linux中没有专门的改名指令  
但是我们可以利用移动指令来实现改名  
```shell
# 如果是相同目录下移动 就相当于改名
mv [old_file_path] [new_file_path] # 将文件从old_path移动到new_path
mv -r [old_dir_path] [new_dir_path] # 将目录移动
```

## 拷贝
```shell
cp [old_file] [new_file]
cp -r [old_dir] [new_dir]
```

## 搜索  
```shell
find [path] -name 'name' # 查找路径path下名字为'name'的文件
find [path] -type 'type' # 查找路径path下类型为'type'的文件
```

## 文件类型
```shell
file [file] # 查看文件类型 
```

## 文件权限
文件权限有三个  
1. 'r' 读取权限
2. 'w' 写权限
3. 'x' 执行权限

使用ll指令可以查看文件的权限  

修改权限使用chmod指令  
```shell
chmod +[mod] [file] # 给file添加mod权限
chmod -[mod] [file] # 给file删除mod权限 
```

## 打包压缩
首先需要区分zip、rar、tar、gz、bz2后缀的区别  

1. zip、rar 表示Windows系统的压缩文件
2. tar 表示Linux系统的打包文件
3. gz、bz2 表示Linux系统中两种压缩文件

### tar打包压缩
首先说明几个常用参数  
| 参数   | 作用                 |
| ------ | -------------------- |
| -c     | 打包                 |
| -x     | 解包                 |
| -f     | 指定打包后的名字     |
| -v     | 展示操作过程         |
| -z     | gzip压缩文件         |
| -j     | bzip2 压缩文件       |
| --list | 列出打包文件中的文件 |

```shell
tar -cf [bag.tar] [dir] # 将dir目录下所有文件 打包(c) 为 bag.tar(f)
tar -xf [bag.tar] # 将 bag.tar(f) 解包(x)
tar -czf [bag.tar.gz] [dir] # 打包的基础上还要压缩  
tar -xzf [bag.tar.gz] # 解压缩之后解包  
```

## 系统
```shell
uname -a # 显示系统内核、主机、版本等等  
```
