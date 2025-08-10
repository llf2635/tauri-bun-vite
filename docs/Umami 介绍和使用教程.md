# 安装和使用教程

Umami 是一个开源的、以隐私为中心的网站分析工具，是 Google Analytics 的替代品。它提供网站流量、用户行为和性能的基本见解，同时优先考虑数据隐私。

与许多传统分析平台不同，Umami 不会收集或存储个人数据，避免了对 cookies 的需求，并且符合 GDPR 和 PECR 标准。

Umami 设计轻量且易于设置，可以进行自托管，使用户对其数据拥有完全控制权。

# 安装和使用教程

以下是使用 Docker 安装和配置 Umami（搭配 PostgreSQL 数据库）的详细步骤：

---

### **1. 准备工作**
确保已安装：
- Docker 和 Docker Compose
- PostgreSQL 数据库（可容器化或使用外部现有数据库）

---

### **2. 拉取 Umami 镜像**
运行官方提供的命令拉取镜像：
```bash
docker pull docker.umami.is/umami-software/umami:postgresql-latest
```

---

### **3. 创建配置文件**
在项目目录下创建 `docker-compose.yml` 文件，配置 Umami 和 PostgreSQL：

```yaml
version: '3'
services:
  umami:
    image: docker.umami.is/umami-software/umami:postgresql-latest
    container_name: umami
    ports:
      - "3000:3000"  # 将容器端口 3000 映射到主机
    environment:
      DATABASE_URL: postgresql://umami:umami@db:5432/umami  # 数据库连接字符串
      DATABASE_TYPE: postgresql
      REDIS_URL: redis://username:password@your-redis-server:port
      HASH_SALT: your_random_salt_string  # 替换为随机字符串（用于加密）
    depends_on:
      - postgresql-db
      - redis-db
    restart: always

  umami_db:
    image: postgres:latest
    container_name: umami_db
    environment:
      POSTGRES_DB: umami
      POSTGRES_USER: umami
      POSTGRES_PASSWORD: umami  # 生产环境建议使用强密码
    volumes:
      - umami_db_data:/var/lib/postgresql/data  # 持久化数据库数据
    restart: always

  redis-db:
    image: redis:latest
    container_name: umami_db
    environment:
       POSTGRES_DB: umami
       POSTGRES_USER: umami
       POSTGRES_PASSWORD: umami  # 生产环境建议使用强密码
    volumes:
       - umami_db_data:/var/lib/postgresql/data  # 持久化数据库数据
    restart: always

volumes:
  umami_db_data:
```

---

### **4. 启动容器**
在 `docker-compose.yml` 所在目录运行：
```bash
docker-compose up -d
```
- `-d` 表示后台运行。

---

### **5. 验证安装**
1. **检查容器状态**：
   ```bash
   docker ps -a  # 确认 umami 和 db 容器状态为 "Up"
   ```

2. **访问 Umami 控制台**：
    - 打开浏览器访问 `http://localhost:3000`。
    - 默认登录账号：`admin`，密码：`umami`（首次登录后请立即修改）。

---

### **6. 配置网站统计**
1. **登录后**，在控制台添加需要统计的网站。
2. **获取跟踪代码**：
    - 在网站中插入 Umami 提供的 JavaScript 代码：
      ```html
      <script async defer
        src="http://your-umami-domain/script.js"
        data-website-id="YOUR_WEBSITE_ID"></script>
      ```
    - 如果 Umami 运行在本地，替换 `your-umami-domain` 为服务器 IP 或域名（如 `http://localhost:3000`）。

---

### **7. 高级配置**
#### **A. 使用外部 PostgreSQL**
如果使用已有 PostgreSQL 服务，修改 `DATABASE_URL`：
```yaml
environment:
  DATABASE_URL: postgresql://username:password@host:5432/dbname
```

#### **B. 配置 HTTPS**
通过反向代理（如 Nginx）添加 SSL 证书：
```nginx
server {
    listen 443 ssl;
    server_name umami.yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://umami:3000;
        proxy_set_header Host $host;
    }
}
```

#### **C. 数据备份**
定期备份 PostgreSQL 数据卷：
```bash
docker exec umami_db pg_dump -U umami -d umami > umami_backup.sql
```

---

### **8. 常见问题**
#### **Q1: 无法连接数据库**
- 检查 `DATABASE_URL` 格式是否正确。
- 确保 PostgreSQL 容器已启动：
  ```bash
  docker logs umami_db  # 查看数据库日志
  ```

#### **Q2: 跟踪代码不生效**
- 确保网站 ID 正确，且 Umami 服务可被公开访问（非本地需配置域名或公网 IP）。

#### **Q3: 更新 Umami 版本**
```bash
docker-compose pull umami  # 拉取最新镜像
docker-compose up -d      # 重启容器
```

---

### **总结**
通过 Docker Compose 可快速部署 Umami + PostgreSQL，适合轻量级、隐私友好的访问统计需求。如需高可用性，建议将 PostgreSQL 迁移到云数据库（如 AWS RDS）。