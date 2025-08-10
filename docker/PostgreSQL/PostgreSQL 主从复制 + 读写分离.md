当然可以！下面是一个完整的 `docker-compose.yaml` 示例，用于部署 **PostgreSQL 主从复制 + 读写分离** 的环境，使用 **流复制（Streaming Replication）** 实现主从同步，并通过 **HAProxy** 实现读写请求的自动路由（写 → 主库，读 → 从库）。

> ✅ 本示例包含：
- 1 个 PostgreSQL 主节点（Master）
- 1 个 PostgreSQL 从节点（Replica / Slave）
- 1 个 HAProxy 负载均衡器，实现读写分离
- 使用 Docker 网络隔离通信
- 数据持久化（使用本地卷）

---

### ✅ `docker-compose.yaml` 完整示例

```yaml
# docker-compose.yaml
# PostgreSQL 主从复制 + HAProxy 读写分离
# 架构：
# - master: 主库，处理写操作
# - replica: 从库，通过流复制同步数据，处理读操作
# - haproxy: 路由请求：写 → master，读 → replica

version: '3.8'

# 定义服务
services:

  # ==================== 主数据库（Master）====================
  master:
    image: postgres:15  # 使用 PostgreSQL 15 镜像
    container_name: postgres-master
    environment:
      POSTGRES_DB: myapp          # 默认数据库名
      POSTGRES_USER: admin        # 管理员用户名
      POSTGRES_PASSWORD: pass123  # 管理员密码
      POSTGRES_INITDB_ARGS: "--auth-host=scram-sha-256"  # 更安全的认证方式
    volumes:
      - ./data/master:/var/lib/postgresql/data  # 持久化数据
      - ./init/master.sql:/docker-entrypoint-initdb.d/master.sql  # 初始化脚本（可选）
    ports:
      - "5432:5432"  # 主库端口映射（仅用于调试，生产建议通过 HAProxy 访问）
    networks:
      - pg-network
    restart: unless-stopped
    command: |
      postgres -c "wal_level=replica" \
               -c "max_wal_senders=3" \
               -c "max_replication_slots=3" \
               -c "hot_standby=on" \
               -c "logging_collector=on" \
               -c "log_destination=stderr" \
               -c "log_line_prefix='%t [%p]: [%l-1] %c %x %d %u %a %h '" \
               -c "log_statement=ddl"  # 启用 WAL 复制相关参数

  # ==================== 从数据库（Replica）====================
  replica:
    image: postgres:15
    container_name: postgres-replica
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: pass123
    volumes:
      - ./data/replica:/var/lib/postgresql/data  # 持久化从库数据
    depends_on:
      - master  # 等待主库启动
    networks:
      - pg-network
    restart: unless-stopped
    # 使用初始化脚本配置从库复制
    entrypoint: >
      sh -c '
        # 等待主库就绪
        until pg_isready -h master -p 5432; do
          echo "等待主库启动..."
          sleep 2
        done

        # 如果从库数据目录为空，开始从主库同步
        if [ ! -f /var/lib/postgresql/data/standby.signal ]; then
          echo "从主库同步数据..."
          # 使用 pg_basebackup 进行基础备份并配置为从库
          pg_basebackup \
            -h master \
            -D /var/lib/postgresql/data \
            -U replicator \
            -v \
            -P \
            -R  # 自动生成 standby.signal 和 postgresql.auto.conf
          # -R 会自动配置 primary_conninfo 和设置为热备模式
        fi

        # 启动 PostgreSQL
        exec postgres
      '

  # ==================== HAProxy（读写分离）====================
  haproxy:
    image: haproxy:2.8
    container_name: haproxy-pg
    ports:
      - "5000:5000"   # 外部写入口（写 → master）
      - "5001:5001"   # 外部读入口（读 → replica）
    volumes:
      - ./haproxy/haproxy.cfg:/usr/local/etc/haproxy/haproxy.cfg  # 挂载配置文件
    depends_on:
      - master
      - replica
    networks:
      - pg-network
    restart: unless-stopped

# ==================== 网络与卷 ====================
networks:
  pg-network:
    driver: bridge  # 创建一个自定义桥接网络，便于容器间通信

volumes:
  master-data:   # 可选：使用命名卷（本例使用 bind mount）
  replica-data:
```

---

### 📁 需要创建的目录和文件

#### 1. `./haproxy/haproxy.cfg`
```cfg
# haproxy.cfg - HAProxy 配置文件，实现 PostgreSQL 读写分离

global
    log stdout format raw local0 info
    maxconn 4096
    stats socket /tmp/haproxy.sock mode 600 level admin

defaults
    log     global
    mode    tcp
    option  tcplog
    timeout connect 5s
    timeout client  300s
    timeout server  300s

# =============== 写请求：全部路由到 master ===============
frontend ft-postgres-write
    bind *:5000
    default_backend master

backend master
    server master master:5432 check

# =============== 读请求：路由到 replica（可扩展多个） ===============
frontend ft-postgres-read
    bind *:5001
    default_backend replicas

backend replicas
    balance roundrobin
    server replica replica:5432 check
```

#### 2. `./init/master.sql`（可选）
```sql
-- 创建复制用户（用于从库连接主库）
CREATE USER replicator WITH REPLICATION ENCRYPTED PASSWORD 'replpass123';

-- 可选：创建测试表
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

### ✅ 启动服务

```bash
# 创建所需目录
mkdir -p ./data/master ./data/replica ./haproxy ./init

# 保存上面的 haproxy.cfg 和 master.sql 文件

# 启动所有服务
docker-compose up -d
```

---

### 🔍 测试验证

1. **写操作测试（走主库）**
   ```bash
   psql -h localhost -p 5000 -U admin -d myapp -c "INSERT INTO users(name) VALUES ('Alice');"
   ```

2. **读操作测试（走从库）**
   ```bash
   psql -h localhost -p 5001 -U admin -d myapp -c "SELECT * FROM users;"
   ```

3. **查看复制状态（进入主库）**
   ```bash
   docker exec -it postgres-master psql -U admin -c "SELECT * FROM pg_stat_replication;"
   ```

---

### ⚠️ 注意事项

- **复制用户权限**：主库必须创建 `replication` 用户（如 `replicator`），并授权。
- **网络延迟**：从库有复制延迟，强一致性读仍需走主库。
- **高可用**：未包含自动故障转移（可结合 **Patroni** + etcd 实现）。
- **安全性**：生产环境建议使用 SSL、更复杂的认证和防火墙规则。

---

### ✅ 扩展建议

- 增加多个从库：复制多个 `replica` 服务，HAProxy 自动负载均衡。
- 使用 **Patroni** 实现高可用自动主从切换。
- 结合 **PgBouncer** 在 HAProxy 后端做连接池。

---

### 🎯 总结

这个 `docker-compose.yaml` 提供了一个**开箱即用的 PostgreSQL 主从 + 读写分离**环境，适合开发、测试或中小生产场景。通过 HAProxy 实现透明的读写分离，减轻主库压力，提升系统可扩展性。

> 🌐 官方参考：
> - https://www.postgresql.org/docs/current/runtime-config-replication.html
> - https://www.haproxy.org/

如需支持**自动故障转移**或**分片**，可进一步引入 **Citus** 或 **Patroni**。