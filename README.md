# GameMaster - 去中心化聊天区块链节点

GameMaster 是一个专为 ChatMaster P2P 聊天系统设计的区块链节点，使用 Rust 语言开发，提供去中心化的用户身份管理、连接记录和激励机制。

## 🎯 项目定位

GameMaster 作为 ChatMaster 聊天系统的区块链基础设施，负责：
- 用户身份的去中心化管理
- P2P 连接记录的不可篡改存储
- 聊天行为的代币激励分发
- 跨节点的用户发现和状态同步

## 🏗️ 系统架构

```
┌─────────────────────────────────────────────┐
│           ChatMaster 客户端                  │
│         (React + WebRTC + Web3)             │
└────────────────┬────────────────────────────┘
                 │
┌────────────────▼────────────────────────────┐
│        混合信令服务器 (Port 3001)            │
│     (Socket.IO + Blockchain Client)         │
└────────────────┬────────────────────────────┘
                 │
┌────────────────▼────────────────────────────┐
│      GameMaster 区块链节点 (Port 3002)      │
│         (Rust + Warp + PoW)                 │
└─────────────────────────────────────────────┘
```

## 📦 核心模块

### 1. **区块链核心** (`src/block.rs`, `src/blockchain.rs`)
- 基础区块结构和哈希计算
- PoW (工作量证明) 共识机制
- 区块链验证和状态管理

### 2. **聊天数据模型** (`src/chat_data.rs`)
```rust
UserProfile {
    address: String,        // 钱包地址
    username: String,       // 显示名称
    public_key: String,     // 加密公钥
    reputation: u64,        // 信誉分
    token_balance: u64,     // 代币余额
}

ConnectionRecord {
    from_address: String,
    to_address: String,
    connection_type: String, // "p2p" or "relay"
    message_count: u64,
}
```

### 3. **聊天区块链** (`src/chat_blockchain.rs`)
- 用户注册和身份管理
- 在线状态追踪
- P2P连接记录
- 代币奖励机制

### 4. **HTTP API** (`src/chat_api.rs`)
- RESTful API 接口
- WebSocket 支持（规划中）
- 跨域请求处理

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- Cargo
- 端口 3002 可用

### 安装依赖
```bash
cd gameMaster
cargo build
```

### 启动节点
```bash
cargo run
```

节点启动后会在端口 3002 提供 HTTP API 服务。

## 📡 API 接口

### 健康检查
```http
GET /health
```

### 用户注册
```http
POST /users/register
{
    "address": "0x1234...",
    "username": "Alice",
    "public_key": "pub_key_hex"
}
```

### 更新在线状态
```http
POST /users/status
{
    "address": "0x1234...",
    "username": "Alice",
    "status": "online",
    "node_id": "socket_id"
}
```

### 记录P2P连接
```http
POST /connections
{
    "from_address": "0x1234...",
    "to_address": "0x5678...",
    "connection_type": "p2p_success"
}
```

### 获取在线用户
```http
GET /users/online
```

### 获取用户信息
```http
GET /users/{address}
```

### 区块链统计
```http
GET /blockchain/stats
```

## 💰 代币经济

### 初始供应
- 创世区块：1,000,000 代币

### 奖励机制
| 行为 | 奖励数量 | 说明 |
|------|---------|------|
| 用户注册 | 100 | 新用户激励 |
| 上线 | 10 | 活跃度奖励 |
| P2P连接成功 | 20 | 双方各得20 |
| 发送消息 | 待定 | 计划实现 |
| 中继消息 | 待定 | 节点贡献奖励 |

### 未来规划
- 质押机制：锁定代币获得更高信誉
- 治理投票：代币持有者参与决策
- 手续费：复杂操作需要支付少量代币

## 🔐 安全特性

1. **身份安全**
   - 基于公私钥对的身份验证
   - 地址由公钥派生，不可伪造

2. **数据完整性**
   - 所有数据上链前进行哈希
   - 区块链不可篡改特性保证历史记录

3. **隐私保护**
   - 消息内容不上链，仅记录元数据
   - 支持端到端加密（客户端实现）

## 🛠️ 技术栈

- **语言**: Rust
- **Web框架**: Warp
- **异步运行时**: Tokio
- **序列化**: Serde
- **加密**: SHA-256
- **时间处理**: Chrono

## 📋 待办事项

- [ ] 实现节点间同步协议
- [ ] 添加 WebSocket 实时推送
- [ ] 实现更复杂的激励机制
- [ ] 支持智能合约
- [ ] 实现跨链桥接
- [ ] 添加 IPFS 集成存储大文件
- [ ] 实现 DID (去中心化身份) 标准

## 🤝 与 ChatMaster 集成

GameMaster 专门为 ChatMaster 聊天系统设计，提供：
1. 去中心化的用户身份管理
2. P2P 连接的信任基础
3. 经济激励促进网络活跃
4. 跨节点的用户发现

## 📈 性能指标

- 区块生成时间：约 3-5 秒（难度=2）
- TPS：100+ (优化后可达 1000+)
- API 响应时间：< 50ms
- 内存占用：< 100MB（10万用户）

## 🔧 配置选项

可通过环境变量配置：
```bash
MINING_DIFFICULTY=2     # 挖矿难度
API_PORT=3002           # API 端口
REWARD_POOL=1000000     # 初始奖励池
BLOCK_REWARD=50         # 区块奖励
```

## 📝 开源协议

MIT License

## 👥 贡献指南

欢迎提交 PR 和 Issue！

## 🌐 相关项目

- [ChatMaster](../chatMaster) - P2P 聊天客户端
- WebRTC - 点对点通信协议
- IPFS - 分布式存储（计划集成）

## 📞 联系方式

- GitHub: [LuckyYouStudio/chatMaster](https://github.com/LuckyYouStudio/chatMaster)

---

*GameMaster - 为去中心化聊天而生的区块链节点*