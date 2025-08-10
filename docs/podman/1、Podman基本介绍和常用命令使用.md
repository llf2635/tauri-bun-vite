# Podman 简介及在 Fedora 中的使用指南

## 什么是 Podman？

Podman（Pod Manager）是一个开源的容器引擎，用于开发、管理和运行OCI（Open Container Initiative）容器。它是 Red Hat 开发的 Daemonless 容器工具，可以作为 Docker 的替代品。

### Podman 的主要作用：

- 运行和管理容器
- 构建容器镜像
- 推送和拉取容器镜像
- 管理容器网络和存储
- 支持 root 和非 root 用户运行容器

## 在 Fedora 中使用 Podman

Fedora 默认已安装 Podman，如果没有可以通过以下命令安装：

```bash
sudo dnf install -y podman podman-compose
echo Fedora 内置 podman 版本号为：$(podman --version)
echo 安装的 podman-compose 版本号为：$(podman-compose --version)
```

## Podman 常用命令及说明

以下是 Podman 常用命令的表格整理：

| 命令                  | 说明                     | 示例                                         |
| --------------------- | ------------------------ | -------------------------------------------- |
| `podman run`          | 运行容器                 | `podman run -it fedora bash`                 |
| `podman ps`           | 列出运行中的容器         | `podman ps -a` (显示所有容器)                |
| `podman images`       | 列出本地镜像             | `podman images`                              |
| `podman pull`         | 拉取镜像                 | `podman pull fedora:latest`                  |
| `podman build`        | 构建镜像                 | `podman build -t myimage .`                  |
| `podman stop`         | 停止容器                 | `podman stop container_name`                 |
| `podman start`        | 启动已停止的容器         | `podman start container_name`                |
| `podman restart`      | 重启容器                 | `podman restart container_name`              |
| `podman rm`           | 删除容器                 | `podman rm container_name`                   |
| `podman rmi`          | 删除镜像                 | `podman rmi image_name`                      |
| `podman exec`         | 在运行中的容器中执行命令 | `podman exec -it container_name bash`        |
| `podman logs`         | 查看容器日志             | `podman logs container_name`                 |
| `podman inspect`      | 查看容器/镜像详细信息    | `podman inspect container_name`              |
| `podman commit`       | 从容器创建新镜像         | `podman commit container_name new_image`     |
| `podman network`      | 管理网络                 | `podman network create mynet`                |
| `podman volume`       | 管理卷                   | `podman volume create myvol`                 |
| `podman info`         | 显示系统信息             | `podman info`                                |
| `podman version`      | 显示版本信息             | `podman version`                             |
| `podman search`       | 搜索容器镜像             | `podman search nginx`                        |
| `podman login`        | 登录到容器注册表         | `podman login docker.io`                     |
| `podman push`         | 推送镜像到注册表         | `podman push myimage docker.io/user/myimage` |
| `podman system prune` | 清理未使用的数据         | `podman system prune`                        |

## 使用示例

1. **运行一个 Fedora 容器**：

   ```bash
   # 1. 首先拉取 PostgreSQL 官方镜像（这里使用 15 版本）
   podman pull postgres:latest
   
   # 2. 创建用于持久化数据的卷
   podman volume create pgdata
   
   # 3. 设置数据库密码环境变量（实际使用中建议使用更安全的方式）
   export POSTGRES_PASSWORD="mysecretpassword"
   
   # 4. 运行 PostgreSQL 容器，-p 主机端口:容器端口
   podman run -d \
     --name postgres-server \
     -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
     -e POSTGRES_USER=myuser \
     -e POSTGRES_DB=mydatabase \
     -v pgdata:/var/lib/postgresql/data \
     -p 5432:5432 \
     postgres:latest
   
   # 5. 检查容器运行状态
   podman ps -f name=postgres-server
   
   # 6. 查看日志确认数据库已就绪
   podman logs postgres-server
   
   # 7. 进入容器执行 psql 命令行工具
   podman exec -it postgres-server psql -U myuser -d mydatabase
   
   # 退出 psql 命令行时使用 \q
   
   # 停止容器
   podman stop postgres-server
   
   # 启动已停止的容器
   podman start postgres-server
   
   # 备份数据卷
   podman run --rm -v pgdata:/volume -v $(pwd):/backup alpine \
     tar -czf /backup/pgdata_backup.tar.gz -C /volume ./
   
   # 删除容器（数据卷会保留）
   podman rm postgres-server
   ```

2. **使用 `podman-compose` 改写的 PostgreSQL 示例**：

   ```bash
   # 1. 安装 podman-compose（如果尚未安装）
   sudo dnf install -y podman-compose
   
   podman --version
   podman-compose --version
   
   # Podman 本身是一个守护进程，但默认不自动启动。可以手动启用
   sudo systemctl enable --now podman.service
   sudo systemctl start podman
   sudo systemctl stop podman
   # 重启 podman 相关服务（如果使用了 API 服务）
   sudo systemctl restart podman  # 仅当启用了 podman 服务时需执行
   sudo systemctl status podman
   
   # 在 Fedora 上可能需要先启用 socket
   sudo systemctl --user enable --now podman.socket
   ```

3. **创建 `docker-compose.yaml` 文件**：

   ```yaml
   version: '3.8'
   
   services:
     postgres:
       image: postgres:latest
       container_name: postgres-server
       environment:
         POSTGRES_PASSWORD: mysecretpassword
         POSTGRES_USER: myuser
         POSTGRES_DB: mydatabase
       volumes:
         - pgdata:/var/lib/postgresql/data
       ports:
         - "5432:5432"
       restart: unless-stopped
   
   volumes:
     pgdata:
   ```

4. **podman-compose 操作命令集**：

   ```bash
   # 2. 启动服务（后台模式）
   podman-compose up -d
   
   # 3. 查看运行状态
   podman-compose ps
   
   # 4. 查看日志
   podman-compose logs
   
   # 5. 进入容器执行 psql
   podman-compose exec postgres psql -U myuser -d mydatabase
   
   # 6. 停止服务（保留数据卷）
   podman-compose down
   
   # 7. 停止并删除所有资源（包括数据卷）
   podman-compose down -v  
   ```

Podman 提供了与 Docker 类似的用户体验，但采用了更安全的无守护进程架构，是 Fedora 等 Linux 发行版中推荐的容器解决方案。

# Podman 常用命令及重要参数详解

以下是 Podman 常用命令及其重要参数的详细表格整理：

## 容器运行与管理

| 命令          | 重要参数    | 参数说明             | 示例                                           |
| ------------- | ----------- | -------------------- | ---------------------------------------------- |
| `podman run`  | `-d`        | 后台运行容器         | `podman run -d nginx`                          |
|               | `-it`       | 交互式终端           | `podman run -it fedora bash`                   |
|               | `--name`    | 指定容器名称         | `podman run --name myweb nginx`                |
|               | `-p`        | 端口映射 (主机:容器) | `podman run -p 8080:80 nginx`                  |
|               | `-v`        | 卷挂载 (主机:容器)   | `podman run -v /data:/var/lib/data mysql`      |
|               | `--rm`      | 退出后自动删除容器   | `podman run --rm alpine`                       |
|               | `-e`        | 设置环境变量         | `podman run -e MYSQL_ROOT_PASSWORD=pass mysql` |
|               | `--network` | 指定网络             | `podman run --network=host nginx`              |
| `podman exec` | `-it`       | 交互式终端           | `podman exec -it mycontainer bash`             |
|               | `-u`        | 指定用户             | `podman exec -u root mycontainer bash`         |

## 容器生命周期管理

| 命令             | 重要参数   | 参数说明                 | 示例                                             |
| ---------------- | ---------- | ------------------------ | ------------------------------------------------ |
| `podman ps`      | `-a`       | 显示所有容器(包括停止的) | `podman ps -a`                                   |
|                  | `-s`       | 显示容器大小             | `podman ps -s`                                   |
|                  | `--format` | 自定义输出格式           | `podman ps --format "table {{.ID}}\t{{.Names}}"` |
| `podman stop`    | `-t`       | 停止超时时间(秒)         | `podman stop -t 5 mycontainer`                   |
| `podman rm`      | `-f`       | 强制删除(运行中的容器)   | `podman rm -f mycontainer`                       |
|                  | `-v`       | 同时删除关联卷           | `podman rm -v mycontainer`                       |
| `podman restart` | `-t`       | 重启超时时间(秒)         | `podman restart -t 5 mycontainer`                |

## 镜像管理

| 命令            | 重要参数      | 参数说明                 | 示例                                       |
| --------------- | ------------- | ------------------------ | ------------------------------------------ |
| `podman images` | `-a`          | 显示所有镜像(包括中间层) | `podman images -a`                         |
|                 | `--digests`   | 显示摘要信息             | `podman images --digests`                  |
|                 | `--no-trunc`  | 不截断输出               | `podman images --no-trunc`                 |
| `podman rmi`    | `-f`          | 强制删除镜像             | `podman rmi -f fedora`                     |
| `podman pull`   | `--quiet`     | 静默模式(只显示镜像ID)   | `podman pull --quiet nginx`                |
|                 | `--platform`  | 指定平台架构             | `podman pull --platform linux/arm64 nginx` |
| `podman build`  | `-f`          | 指定Dockerfile路径       | `podman build -f Dockerfile.dev .`         |
|                 | `--no-cache`  | 不使用缓存               | `podman build --no-cache .`                |
|                 | `--build-arg` | 设置构建参数             | `podman build --build-arg VERSION=1.0 .`   |

## 网络与存储

| 命令                    | 重要参数   | 参数说明                   | 示例                                                |
| ----------------------- | ---------- | -------------------------- | --------------------------------------------------- |
| `podman network create` | `--subnet` | 指定子网                   | `podman network create --subnet 10.10.0.0/24 mynet` |
|                         | `--driver` | 指定驱动(bridge/macvlan等) | `podman network create --driver macvlan mynet`      |
| `podman volume create`  | `--opt`    | 设置卷选项                 | `podman volume create --opt type=tmpfs myvol`       |

## 系统管理

| 命令                  | 重要参数    | 参数说明             | 示例                            |
| --------------------- | ----------- | -------------------- | ------------------------------- |
| `podman system prune` | `-a`        | 删除所有未使用的镜像 | `podman system prune -a`        |
|                       | `--volumes` | 同时删除未使用的卷   | `podman system prune --volumes` |
|                       | `-f`        | 不提示确认           | `podman system prune -f`        |
| `podman info`         | `--debug`   | 显示调试信息         | `podman info --debug`           |

## 日志与检查

| 命令             | 重要参数   | 参数说明             | 示例                                                         |
| ---------------- | ---------- | -------------------- | ------------------------------------------------------------ |
| `podman logs`    | `--tail`   | 显示最后N行日志      | `podman logs --tail 50 mycontainer`                          |
|                  | `-f`       | 实时跟踪日志         | `podman logs -f mycontainer`                                 |
|                  | `--since`  | 显示某时间后的日志   | `podman logs --since 1h mycontainer`                         |
| `podman inspect` | `--format` | 使用Go模板格式化输出 | `podman inspect --format '{{.NetworkSettings.IPAddress}}' mycontainer` |

这些参数可以组合使用，例如：

```bash
podman run -d --name web -p 8080:80 -v /data:/usr/share/nginx/html nginx
```

以上参数能帮助您更灵活地使用Podman管理容器和镜像，满足不同的使用场景需求。