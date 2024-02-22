# linux 指令  
长话短说  

## 文件路径

- cd == change directory  

> 假设目前所在目录是 /c/Users 
> character 是 Users 的一个文件夹  
> \$ cd character 可以进入该文件夹 还有  
> cd ./character  
> 使用..回到父目录  
> cd ..  则进入/c/Users

- ls == list the directory  

- pwd == print working dir

- mv filename1 filename2
> 此处的filename可以理解为file的相对路径结果
> 上述写法如果只是name，则是改名效果
> 如果是另一个文件夹中，则是移动效果

- cp filename1 filename2

- echo
> 使用echo可以创建文件  
> echo hello > hello\world.txt

- 善用流概念
- <
- \>
- |
> a < b 表示将b作为a的输入
> a > b 表示将a作为b的输入
> a | b 表示a作为b的输入 

linux中常用的五十个指令简介
> ls - The most frequently used command in Linux to list directories
> pwd - Print working directory command in Linux
> cd - Linux command to navigate through directories
> mkdir - Command used to create directories in Linux
> mv - Move or rename files in Linux
> cp - Similar usage as mv but for copying files in Linux
> rm - Delete files or directories
> touch - Create blank/empty files
> ln - Create symbolic links (shortcuts) to other files
> cat - Display file contents on the terminal
> clear - Clear the terminal display
> echo - Print any text that follows the command
> less - Linux command to display paged outputs in the terminal
> man - Access manual pages for all Linux commands
> uname - Linux command to get basic information about the OS
> whoami - Get the active username
> tar - Command to extract and compress files in Linux
> grep - Search for a string within an output
> head - Return the specified number of lines from the top
> tail - Return the specified number of lines from the bottom
> diff - Find the difference between two files
> cmp - Allows you to check if two files are identical
> comm - Combines the functionality of diff and cmp
> sort - Linux command to sort the content of a file while outputting
> export - Export environment variables in Linux
> zip - Zip files in Linux
> unzip - Unzip files in Linux
> ssh - Secure Shell command in Linux
> service - Linux command to start and stop services
> ps - Display active processes
> kill and killall - Kill active processes by process ID or name
> df - Display disk filesystem information
> mount - Mount file systems in Linux
> chmod - Command to change file permissions
> chown - Command for granting ownership of files or folders
> ifconfig - Display network interfaces and IP addresses
> traceroute - Trace all the network hops to reach the destination
> wget - Direct download files from the internet
> ufw - Firewall command
> iptables - Base firewall for all other firewall utilities to interface with
> apt, pacman, yum, rpm - Package managers depending on the distro
> sudo - Command to escalate privileges in Linux
> cal - View a command-line calendar
> alias - Create custom shortcuts for your regularly used commands
> dd - Majorly used for creating bootable USB sticks
> whereis - Locate the binary, source, and manual pages for a command
> whatis - Find what a command is used for
> top - View active processes live with their system usage
> useradd and usermod - Add new user or change existing users data
> passwd - Create or update passwords for existing users