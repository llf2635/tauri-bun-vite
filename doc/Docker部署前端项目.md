在 Vue 3 + TypeScript + Vite 项目中构建 Docker 镜像并运行，需要经过几个步骤。以下是详细的步骤和示例代码：

### 1. 准备 Dockerfile

首先，你需要创建一个 `Dockerfile` 文件，该文件描述了如何构建 Docker 镜像。假设你的项目已经准备好，并且你有一个 `vite.config.ts` 文件来配置 Vite 构建。

创建一个名为 `Dockerfile` 的文件，并放置在项目的根目录下：

```Dockerfile
# 使用 Bun 的官方镜像
FROM bun:latest as build-stage

# 设置工作目录
WORKDIR /app

# 复制 package.json 和 bun.lockfile
COPY package.json bun.lockfile ./

# 安装依赖
RUN bun install

# 复制项目源码
COPY . .

# 构建应用
RUN bun run build

# 使用 Nginx 镜像来运行静态文件
FROM nginx:alpine

# 移除默认的 Nginx 配置
RUN rm /etc/nginx/conf.d/default.conf

# 复制构建好的文件
COPY --from=build-stage /app/dist /usr/share/nginx/html

# 复制自定义的 Nginx 配置
COPY nginx.conf /etc/nginx/conf.d/
```

### 2. 创建 Nginx 配置文件

为了正确地配置 Nginx，以便它可以正确地处理 Vue Router 的路由，你需要创建一个 `nginx.conf` 文件。这个文件同样需要放在项目的根目录下：

```nginx
server {
    listen 80;
    server_name localhost;

    location / {
        root /usr/share/nginx/html;
        index index.html;
        try_files $uri $uri/ /index.html;
    }

    error_log  /var/log/nginx/error.log;
    access_log /var/log/nginx/access.log;
}
```

### 3. 构建 Docker 镜像

在项目根目录下执行以下命令来构建 Docker 镜像：

```bash
docker build -t bun-vue-ts .
```

这里的 `your-image-name` 是你给镜像命名的方式，你可以替换成任何你想要的名字。

### 4. 运行 Docker 容器

构建完成后，你可以运行一个新的 Docker 容器：

```bash
docker run -d -p 80:80 --name bun-vue-ts bun-vue-ts
```

这里的 `your-container-name` 是你给容器命名的方式，同样可以替换成任何你想要的名字。

### 5. 访问应用

现在你的应用应该在本地的 `http://localhost` 上运行了。

### 说明

- **构建阶段** (`build-stage`)：
    - 使用 `node:16-alpine` 镜像作为构建阶段的基础镜像。
    - 安装项目依赖，并构建项目。
    - 将构建好的文件复制到 Nginx 镜像中。

- **运行阶段** (`nginx:alpine`)：
    - 使用 `nginx:alpine` 镜像作为运行阶段的基础镜像。
    - 配置 Nginx 来处理静态文件，并支持 Vue Router 的路由。

### 注意事项

- **Node.js 版本**：确保你选择的 Node.js 版本与你的项目兼容。
- **Nginx 配置**：Nginx 的配置文件 `nginx.conf` 确保了对 Vue Router 的支持，特别是对于单页面应用（SPA）的路由重定向。
- **端口映射**：确保你正确地映射了端口，以便可以从主机访问容器中的应用。

通过上述步骤，你可以成功地将 Vue 3 + TypeScript + Vite 项目构建成 Docker 镜像，并运行在 Docker 容器中。