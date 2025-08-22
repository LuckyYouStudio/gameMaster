use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub address: String,        // 钱包地址作为用户ID
    pub username: String,       // 显示名称
    pub public_key: String,     // 用于加密通信
    pub last_seen: DateTime<Utc>,
    pub reputation: u64,        // 信誉分数
    pub token_balance: u64,     // 代币余额
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRecord {
    pub from_address: String,
    pub to_address: String,
    pub connection_type: String, // "p2p" or "relay"
    pub timestamp: DateTime<Utc>,
    pub duration: Option<u64>,   // 连接持续时间(秒)
    pub message_count: u64,      // 消息数量
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReward {
    pub user_address: String,
    pub action: String,          // "online", "connect", "message"
    pub reward_amount: u64,      // 奖励代币数量
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineStatus {
    pub address: String,
    pub username: String,
    pub status: String,          // "online", "offline", "busy"
    pub node_id: String,         // 所在节点ID
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatTransaction {
    UserRegister(UserProfile),
    StatusUpdate(OnlineStatus),
    ConnectionEstablished(ConnectionRecord),
    RewardIssued(ChatReward),
}