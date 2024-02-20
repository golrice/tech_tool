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

## git命令
- git init 新建仓库

- git config --global user.name \<name>
- git config --global user.email \<email>

- git add 添加到缓存区  
> git add \<filename>
> git add ./\<file_form>
> git add \<dirname>  git add . 添加当前文件夹所有文件

- git commit 缓存文件添加到仓库  
> git commit -m \<description>
> git commit 进入vim模式后填写description  
> git commit -am \<description>
> git commit --amend -m \<description>

- git status 查看仓库状态 
- git log 查看提交日志  
> git log
> git log --oneline 简洁模式
> git log --graph --oneline
> git log --pretty="%h %cd %an %ce -> %s"

- git reset 回退版本  
> git reset --soft 工作区恢复 缓存区恢复
> git reset --hard 工作区删除 缓存区删除
> git reset --mixed 工作区恢复 缓存区删除

- ls 显示工作区文件  
- git ls-files 显示缓冲区文件
- git ls-tree -r HEAD 显示当前版本库文件

- git diff 查看版本差异
> git diff 工作区 vs 缓存区
> git diff HEAD  工作区 缓存区 vs 最新本地仓库
> git diff --cached  暂存区 vs 本地仓库
> git diff HEAD~\<number> HEAD 两个版本之间的差异

- git rm 删除 同时在工作区和暂存区  
> git rm \<filename>
> git rm --cached \<filename> 只在缓存区删除
> git rm -r * 删除目录中所有内容

- .gitignore 忽略文件所在文件夹
> 使用vim打开该文件后 输入需要忽略的文件名称即可  
> 注意 只会监视到当时在工作区的文件  
> 可以使用正则表达式来表示多个文件  

- 生成ssh密钥
步骤如下  
> cd 回到根目录
> cd .ssh
> ssh -keygen -r rsa -b 4096
> 填写文件名称和密码  
> 密钥 id_res
> 公钥 id_res.pub

从github上下载一个库
> - git clone \<repo-address> 克隆仓库到当前文件夹
> - git push \<remote> <branch> 推送到远程仓库  
> - git pull \<remote> 拉取远程仓库

将本地仓库内容作为新仓库送到GitHub上  
> - git remote add \<another name of the repo> \<address>
> - git push -u \<alias> \<remote branch>:\<local branch>

查看关联的远程仓库的别名和地址  
> - git remote -v

将远程仓库信息同步到本地仓库  
> - git pull \<remote repo name> \<remote branch>:\<local branch>


分支管理  

- git branch 展示当前存在的分支  
- git branch \<branch name> 创建新的分支  
- git switch \<branch name> 切换到该分支  
- git merge \<branch name> 将目标分支合并到当前分支
- git branch -d \<branch name> 删除目标分支

冲突解决  
> 在提交合并的时候如果修改了同一处代码  
> 在合并的时候就会显示存在冲突  
> 可以使用几个代码查看状态
> 
> - git status
> - git diff  
>
> 通过vim修改冲突后提交即可 

回退版本
- git branch -b \<branch name> \<id> 回到某个分支时刻  
- git rebase \<new base> 将当前分支合并到新base上