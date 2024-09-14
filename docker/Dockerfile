# 使用 Bun 的官方镜像
FROM oven/bun:latest as build-stage

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
# 设置作者信息
LABEL MAINTAINER="SliverHorn@sliver_horn@qq.com"

# 移除默认的 Nginx 配置
RUN rm /etc/nginx/conf.d/nginx.conf

# 复制构建好的文件
COPY --from=build-stage /app/dist /usr/share/nginx/html

# 复制自定义的 Nginx 配置
COPY nginx.conf /etc/nginx/conf.d/

RUN chmod 775 -R /usr/share/nginx/html
EXPOSE 80/tcp