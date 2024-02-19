# git的简单教学

## 简单入手
分为两种情况  

一、 刚创建文件夹，啥都没有  
二、 已经创建了一个仓库，就准备提交了  

### 一、 无仓库
具体流程
```powershell
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin <ssh地址>
git push -u origin main
```

简单来说  
首先使用**git init**来创建一个仓库  
然后使用**git add**来添加需要的项目  
然后使用**git branch -M**来改掉分支名称  
改掉分支名称是因为现在默认的分支名称是master  
然后使用**git remote add**命令来添加远程仓库的地址  
同时可以使用**git remote -v**来查看自己添加的远程仓库  
最后使用**git push -u origin main**来将main分支的内容提交  

值得注意的是  
以后只需要使用**git push**和**git pull**  
就可以完成提交和拉取这个操作了  

### 二、有仓库
那和上面的流程是类似的  
只不过需要从 配置仓库地址这一步开始就行  

