测试环境(虚拟机) - Linux(CentOS7.9)

## 目录结构

```tex
httpserver
├── Cargo.lock
├── Cargo.toml
├── docs
│   ├── css
│   │   ├── comment_reply.css
│   │   └── comments.css
│   ├── index.html
│   └── js
│       └── comments.js
└── src
    ├── Config
    │   ├── _config.json
    │   └── mod.rs
    ├── DatabaseServer
    │   └── mod.rs
    ├── HttpServer
    │   ├── mod.rs
    │   ├── RequestType.rs
    │   └── ResponseSelector
    │       ├── mod.rs
    │       ├── Response2xx.rs
    │       ├── Response4xx.rs
    │       └── Response5xx.rs
    ├── lib.rs
    ├── main.rs
    └── TcpServer
        └── mod.rs
```

## Linux中安装MySQL数据库

以Linux系统下的`MySQL数据库(5.7版本)`为主。

### 卸载Linux下的MySQL

```bash
# 检查是否安装了mysql 
yum list installed | grep mysql

# 倘若有安装了mysql，就执行下面的卸载命令
yum -y remove mysql-libs

#****************************************#

# 查看mysql组件工具是否存在
rpm -qa | grep -i mysql

# 倘若有组件工具，执行下面的卸载命令。
# 鼠标选中组件名，按快捷键ctrl + shift + c 复制
# 按快捷键ctrl + shift + v 粘贴
yum remove 组件名
```

### 安装Linux下的MySQL的

+ [Linux下MySQL指定版本的安装-官方教程](https://dev.mysql.com/doc/refman/5.7/en/linux-installation-yum-repo.html)

+ [MySQL-yum源查看网址](https://dev.mysql.com/downloads/repo/yum/)
+ `Red Hat Enterprise Linux 7 / Oracle Linux 7 (Architecture Independent), RPM Package`
  + `RPM Package`表示该文件为RPM包
  + `Red Hat Enterprise Linux 7 / Oracle Linux 7`表示该 RPM 包适用于`Red Hat Enterprise Linux 7`和`Oracle Linux 7`这两个操作系统版本(如CentOS 7，Oracle Linux 7等)
  + `Architecture Independent`表示该软件包与硬件架构无关

```bash
# 先安装wget
yum -y install wget

# 下载MySQL的yum的Repository
wget https://repo.mysql.com/mysql80-community-release-el7-10.noarch.rpm

# 添加MySQL的yum源
rpm -ivh mysql80-community-release-el7-10.noarch.rpm

# 安装yum-config-manager
yum -y install yum-utils

# 查看当前Release版本的默认安装版本情况
yum repolist all | grep mysql

# 取消MySQL8.0版本的默认安装选中
yum-config-manager --disable mysql80-community

# 开启MySQL5.7版本的默认安装选中
yum-config-manager --enable mysql57-community

# 安装MySQL5.7版本
yum -y install mysql-community-server --nogpgcheck
```

### MySQL初始化配置

```bash
# 启动MySQL服务
service mysqld start

# 查看MySQL服务的状态
service mysqld status

# 刚安装完MySQL后，可通过临时密码来重设数据库密码
# 查看临时密码
grep 'temporary password' /var/log/mysqld.log

# 隐式登录MySQL
mysql -uroot -p
# 接着输入临时密码

# 重设密码
# 密码要求：
#  1.需包含一个大写字母，一个小写字母，一个数字，一个特殊字符(类似!)
#  2.总长度不少于8位
ALTER USER 'root'@'localhost' IDENTIFIED BY '你的新密码';
```

### **创建评论用的数据库**

```sql
/* 评论系统使用的是UTF-8编码，创建数据库时需指定 */ 
/* 创建评论系统数据库 */
CREATE DATABASE CommentSystem CHARACTER SET 'utf8';

/* 查看当前数据库有哪些 */
SHOW DATABASES;
```

## 以二进制的方式运行

运行命令 - `cargo run`

监听当前虚拟机地址的80端口，允许任意接入地址，即监听HTTP请求。

浏览器中输入`http://虚拟主机ip地址/index.html`

