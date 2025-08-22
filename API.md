# GameMaster API 文档

Base URL: `http://localhost:3002`

## 认证

当前版本暂无认证机制，后续版本将添加基于签名的认证。

## 通用响应格式

所有 API 响应遵循统一格式：

```json
{
    "success": boolean,
    "data": any | null,
    "message": string
}
```

## API 端点

### 1. 健康检查

检查 API 服务器运行状态。

**请求**
```http
GET /health
```

**响应示例**
```json
{
    "success": true,
    "data": "ChatBlockchain API is running",
    "message": "OK"
}
```

---

### 2. 用户注册

在区块链上注册新用户。

**请求**
```http
POST /users/register
Content-Type: application/json

{
    "address": "0x1234567890abcdef",
    "username": "Alice",
    "public_key": "04a1b2c3d4..."
}
```

**参数说明**
| 参数 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| address | string | 是 | 用户的区块链地址（唯一标识） |
| username | string | 是 | 用户显示名称 |
| public_key | string | 是 | 用于加密通信的公钥 |

**响应示例**

成功：
```json
{
    "success": true,
    "data": "0x1234567890abcdef",
    "message": "User registered successfully"
}
```

失败（用户已存在）：
```json
{
    "success": false,
    "data": null,
    "message": "User already exists"
}
```

---

### 3. 更新用户状态

更新用户的在线状态。

**请求**
```http
POST /users/status
Content-Type: application/json

{
    "address": "0x1234567890abcdef",
    "username": "Alice",
    "status": "online",
    "node_id": "socket_xyz123"
}
```

**参数说明**
| 参数 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| address | string | 是 | 用户的区块链地址 |
| username | string | 是 | 用户名称 |
| status | string | 是 | 状态：online/offline/busy |
| node_id | string | 是 | 连接的节点ID |

**响应示例**
```json
{
    "success": true,
    "data": "Status updated",
    "message": "User status updated successfully"
}
```

---

### 4. 记录P2P连接

记录用户间的P2P连接信息。

**请求**
```http
POST /connections
Content-Type: application/json

{
    "from_address": "0x1234567890abcdef",
    "to_address": "0xfedcba0987654321",
    "connection_type": "p2p_success"
}
```

**参数说明**
| 参数 | 类型 | 必填 | 说明 |
|-----|------|-----|------|
| from_address | string | 是 | 发起连接的用户地址 |
| to_address | string | 是 | 接收连接的用户地址 |
| connection_type | string | 是 | 连接类型：p2p_attempt/p2p_success/relay |

**响应示例**
```json
{
    "success": true,
    "data": "Connection recorded",
    "message": "Connection recorded successfully"
}
```

---

### 5. 获取在线用户列表

获取当前所有在线用户。

**请求**
```http
GET /users/online
```

**响应示例**
```json
{
    "success": true,
    "data": [
        {
            "address": "0x1234567890abcdef",
            "username": "Alice",
            "status": "online",
            "node_id": "socket_xyz123",
            "timestamp": "2024-08-21T10:30:00Z"
        },
        {
            "address": "0xfedcba0987654321",
            "username": "Bob",
            "status": "online",
            "node_id": "socket_abc456",
            "timestamp": "2024-08-21T10:31:00Z"
        }
    ],
    "message": "Online users retrieved"
}
```

---

### 6. 获取用户详细信息

获取指定用户的详细信息。

**请求**
```http
GET /users/{address}
```

**路径参数**
| 参数 | 说明 |
|-----|------|
| address | 用户的区块链地址 |

**响应示例**

用户存在：
```json
{
    "success": true,
    "data": {
        "address": "0x1234567890abcdef",
        "username": "Alice",
        "public_key": "04a1b2c3d4...",
        "last_seen": "2024-08-21T10:30:00Z",
        "reputation": 100,
        "token_balance": 250
    },
    "message": "User found"
}
```

用户不存在：
```json
{
    "success": false,
    "data": null,
    "message": "User not found"
}
```

---

### 7. 获取区块链统计信息

获取区块链的整体统计数据。

**请求**
```http
GET /blockchain/stats
```

**响应示例**
```json
{
    "success": true,
    "data": {
        "block_count": 42,
        "user_count": 15,
        "online_count": 8,
        "connection_count": 120,
        "total_rewards": 950000,
        "is_valid": true
    },
    "message": "Blockchain stats retrieved"
}
```

**字段说明**
| 字段 | 说明 |
|-----|------|
| block_count | 区块总数 |
| user_count | 注册用户总数 |
| online_count | 当前在线用户数 |
| connection_count | 历史连接总数 |
| total_rewards | 剩余奖励池 |
| is_valid | 区块链完整性状态 |

---

## 错误码

| HTTP 状态码 | 说明 |
|------------|------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

## 使用示例

### cURL 示例

注册用户：
```bash
curl -X POST http://localhost:3002/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x1234567890abcdef",
    "username": "Alice",
    "public_key": "04a1b2c3d4..."
  }'
```

获取在线用户：
```bash
curl http://localhost:3002/users/online
```

### JavaScript 示例

```javascript
// 注册用户
const registerUser = async () => {
  const response = await fetch('http://localhost:3002/users/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      address: '0x1234567890abcdef',
      username: 'Alice',
      public_key: '04a1b2c3d4...'
    })
  });
  
  const result = await response.json();
  console.log(result);
};

// 获取在线用户
const getOnlineUsers = async () => {
  const response = await fetch('http://localhost:3002/users/online');
  const result = await response.json();
  console.log(result.data);
};
```

### Python 示例

```python
import requests

# 注册用户
def register_user():
    url = "http://localhost:3002/users/register"
    data = {
        "address": "0x1234567890abcdef",
        "username": "Alice",
        "public_key": "04a1b2c3d4..."
    }
    response = requests.post(url, json=data)
    return response.json()

# 获取在线用户
def get_online_users():
    url = "http://localhost:3002/users/online"
    response = requests.get(url)
    return response.json()
```

## WebSocket API (计划中)

未来版本将支持 WebSocket 连接，实现实时推送功能：

```javascript
const ws = new WebSocket('ws://localhost:3002/ws');

ws.on('connect', () => {
  // 订阅事件
  ws.send(JSON.stringify({
    type: 'subscribe',
    events: ['user_online', 'new_connection', 'new_block']
  }));
});

ws.on('message', (data) => {
  const event = JSON.parse(data);
  switch(event.type) {
    case 'user_online':
      console.log('User came online:', event.data);
      break;
    case 'new_block':
      console.log('New block mined:', event.data);
      break;
  }
});
```

## 限制与配额

当前版本无限制，生产环境将实施以下限制：

- 请求频率：100 请求/分钟/IP
- 批量查询：最多 100 条记录
- 响应大小：最大 10MB
- 连接超时：30 秒

## 版本历史

### v0.1.0 (当前)
- 基础 HTTP API
- 用户管理
- 连接记录
- 区块链统计

### v0.2.0 (计划)
- WebSocket 支持
- 签名认证
- 批量操作
- 分页查询

### v0.3.0 (规划)
- GraphQL API
- 订阅推送
- 高级查询
- API 密钥管理

---

*更多信息请查看 [GitHub](https://github.com/LuckyYouStudio/chatMaster)*