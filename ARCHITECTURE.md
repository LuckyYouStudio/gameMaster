# GameMaster 架构设计文档

## 概述

GameMaster 是一个基于 Rust 开发的区块链节点系统，专门为去中心化聊天应用 ChatMaster 提供底层支持。

## 核心设计原则

1. **去中心化**: 无单点故障，任何节点都可以验证数据
2. **高性能**: 使用 Rust 实现，追求极致性能
3. **可扩展**: 模块化设计，易于添加新功能
4. **安全性**: 密码学保证数据完整性和身份验证

## 系统组件

### 1. 区块链层 (Blockchain Layer)

#### Block 结构
```rust
pub struct Block {
    pub index: u64,           // 区块索引
    pub timestamp: String,    // 时间戳
    pub data: String,         // 交易数据 (JSON)
    pub previous_hash: String, // 前块哈希
    pub hash: String,         // 当前块哈希
    pub nonce: u64,          // PoW 随机数
}
```

#### 共识机制
- **算法**: Proof of Work (PoW)
- **难度**: 可调节（当前设置为 2）
- **验证**: SHA-256 哈希前缀匹配

### 2. 数据层 (Data Layer)

#### 用户数据结构
```rust
UserProfile {
    address: String,         // 唯一标识
    username: String,        // 显示名
    public_key: String,      // 公钥
    last_seen: DateTime,     // 最后活跃
    reputation: u64,         // 信誉值
    token_balance: u64,      // 代币余额
}
```

#### 交易类型
- `UserRegister`: 用户注册
- `StatusUpdate`: 状态更新
- `ConnectionEstablished`: 连接建立
- `RewardIssued`: 奖励发放

### 3. 网络层 (Network Layer)

#### HTTP API Server
- **框架**: Warp (异步 Web 框架)
- **端口**: 3002
- **协议**: RESTful JSON API
- **跨域**: CORS 全开放（开发环境）

#### API 端点设计
```
/health                  - 健康检查
/users/register          - 用户注册
/users/status           - 状态更新
/users/online           - 在线列表
/users/{address}        - 用户详情
/connections            - 连接记录
/blockchain/stats       - 链统计
```

### 4. 存储层 (Storage Layer)

#### 内存存储
```rust
pub struct ChatBlockchain {
    pub chain: Vec<Block>,                    // 区块链
    pub users: HashMap<String, UserProfile>,  // 用户映射
    pub online_users: HashMap<String, OnlineStatus>, // 在线状态
    pub connections: Vec<ConnectionRecord>,   // 连接历史
    pub total_rewards: u64,                   // 奖励池
}
```

#### 持久化策略（待实现）
- LevelDB / RocksDB 存储区块
- 定期快照和检查点
- 增量同步机制

## 数据流

### 用户注册流程
```
客户端 -> 信令服务器 -> 区块链API
                          ↓
                      验证唯一性
                          ↓
                      创建用户数据
                          ↓
                      打包交易
                          ↓
                      挖矿 (PoW)
                          ↓
                      添加到链
                          ↓
                      更新内存状态
                          ↓
                      返回成功
```

### P2P 连接流程
```
用户A 请求连接 用户B
        ↓
  记录连接尝试到链
        ↓
   交换信令信息
        ↓
  WebRTC 握手成功
        ↓
  记录成功连接到链
        ↓
   双方获得奖励
```

## 安全模型

### 身份验证
- 基于公私钥对
- 地址由公钥派生
- 签名验证（待实现）

### 数据完整性
- 区块哈希链
- Merkle Tree（规划中）
- 定期验证完整链

### 网络安全
- Rate Limiting（待实现）
- DDoS 防护（待实现）
- 恶意节点检测（待实现）

## 性能优化

### 当前优化
- 异步 I/O (Tokio)
- 零拷贝序列化
- 内存池复用

### 计划优化
- 分片 (Sharding)
- 并行验证
- 缓存层
- 索引优化

## 扩展性设计

### 水平扩展
- 多节点部署
- 负载均衡
- 数据分片

### 垂直扩展
- 插件系统
- 智能合约支持
- 跨链协议

## 监控与运维

### 指标收集
- 区块生成速度
- 交易吞吐量
- API 响应时间
- 内存/CPU 使用率

### 日志系统
- 结构化日志
- 日志级别控制
- 集中日志收集

## 未来演进

### Phase 1: 基础功能（当前）
- ✅ 基本区块链
- ✅ 用户管理
- ✅ HTTP API
- ✅ 激励机制

### Phase 2: 增强功能
- [ ] WebSocket 实时推送
- [ ] 节点发现协议
- [ ] 数据持久化
- [ ] 签名验证

### Phase 3: 高级特性
- [ ] 智能合约
- [ ] 跨链桥接
- [ ] IPFS 集成
- [ ] Layer 2 扩展

### Phase 4: 生产就绪
- [ ] 主网部署
- [ ] 经济模型优化
- [ ] 治理机制
- [ ] 审计与合规

## 技术债务

1. 缺少持久化存储
2. 没有节点间同步
3. 缺少签名验证
4. 需要更好的错误处理
5. 缺少单元测试

## 依赖关系

```toml
[dependencies]
serde = "1.0"          # 序列化
tokio = "1.0"          # 异步运行时
warp = "0.3"           # Web 框架
sha2 = "0.10"          # 哈希算法
chrono = "0.4"         # 时间处理
uuid = "1.0"           # UUID 生成
```

## 部署架构

### 开发环境
```
单节点本地运行
Port: 3002
挖矿难度: 2
```

### 测试环境
```
3-5 个节点
Docker Compose 部署
模拟网络延迟
```

### 生产环境
```
最少 21 个节点
Kubernetes 部署
地理分布式
自动扩缩容
```

---

*本文档会随着项目发展持续更新*