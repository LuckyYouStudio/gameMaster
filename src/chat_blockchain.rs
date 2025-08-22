use crate::block::Block;
use crate::chat_data::*;
use serde_json;
use std::collections::HashMap;
use chrono::Utc;

#[derive(Debug)]
pub struct ChatBlockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub users: HashMap<String, UserProfile>,      // 地址 -> 用户信息
    pub online_users: HashMap<String, OnlineStatus>, // 在线用户状态
    pub connections: Vec<ConnectionRecord>,       // 连接记录
    pub total_rewards: u64,                      // 总奖励池
}

impl ChatBlockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis_data = serde_json::to_string(&ChatTransaction::UserRegister(UserProfile {
            address: "genesis".to_string(),
            username: "Genesis User".to_string(),
            public_key: "genesis_key".to_string(),
            last_seen: Utc::now(),
            reputation: 0,
            token_balance: 1000000, // 初始代币供应
        })).unwrap();
        
        let genesis_block = Block::new(0, genesis_data, "0".to_string());
        chain.push(genesis_block);
        
        ChatBlockchain {
            chain,
            difficulty,
            users: HashMap::new(),
            online_users: HashMap::new(),
            connections: Vec::new(),
            total_rewards: 1000000,
        }
    }

    // 注册新用户
    pub fn register_user(&mut self, address: String, username: String, public_key: String) -> bool {
        if self.users.contains_key(&address) {
            return false; // 用户已存在
        }

        let user_profile = UserProfile {
            address: address.clone(),
            username,
            public_key,
            last_seen: Utc::now(),
            reputation: 0,
            token_balance: 100, // 新用户奖励
        };

        let transaction = ChatTransaction::UserRegister(user_profile.clone());
        let data = serde_json::to_string(&transaction).unwrap();
        self.add_block(data);
        
        self.users.insert(address.clone(), user_profile);
        self.issue_reward(address, "register".to_string(), 100);
        
        true
    }

    // 更新用户在线状态
    pub fn update_user_status(&mut self, address: String, username: String, status: String, node_id: String) {
        let online_status = OnlineStatus {
            address: address.clone(),
            username,
            status: status.clone(),
            node_id,
            timestamp: Utc::now(),
        };

        let transaction = ChatTransaction::StatusUpdate(online_status.clone());
        let data = serde_json::to_string(&transaction).unwrap();
        self.add_block(data);

        if status == "online" {
            self.online_users.insert(address.clone(), online_status);
            self.issue_reward(address, "online".to_string(), 10);
        } else {
            self.online_users.remove(&address);
        }
    }

    // 记录连接建立
    pub fn record_connection(&mut self, from_address: String, to_address: String, connection_type: String) {
        let connection = ConnectionRecord {
            from_address: from_address.clone(),
            to_address: to_address.clone(),
            connection_type,
            timestamp: Utc::now(),
            duration: None,
            message_count: 0,
        };

        let transaction = ChatTransaction::ConnectionEstablished(connection.clone());
        let data = serde_json::to_string(&transaction).unwrap();
        self.add_block(data);

        self.connections.push(connection);
        
        // 连接成功奖励
        self.issue_reward(from_address, "connect".to_string(), 20);
        self.issue_reward(to_address, "connect".to_string(), 20);
    }

    // 发放奖励
    pub fn issue_reward(&mut self, user_address: String, action: String, amount: u64) {
        if self.total_rewards < amount {
            return; // 奖励池不足
        }

        let reward = ChatReward {
            user_address: user_address.clone(),
            action,
            reward_amount: amount,
            timestamp: Utc::now(),
        };

        let transaction = ChatTransaction::RewardIssued(reward);
        let data = serde_json::to_string(&transaction).unwrap();
        self.add_block(data);

        // 更新用户余额
        if let Some(user) = self.users.get_mut(&user_address) {
            user.token_balance += amount;
        }
        self.total_rewards -= amount;
    }

    // 获取在线用户列表
    pub fn get_online_users(&self) -> Vec<&OnlineStatus> {
        self.online_users.values().collect()
    }

    // 获取用户信息
    pub fn get_user(&self, address: &str) -> Option<&UserProfile> {
        self.users.get(address)
    }

    // 获取用户连接历史
    pub fn get_user_connections(&self, address: &str) -> Vec<&ConnectionRecord> {
        self.connections.iter()
            .filter(|conn| conn.from_address == address || conn.to_address == address)
            .collect()
    }

    // 添加区块到链
    fn add_block(&mut self, data: String) {
        let latest_block = &self.chain[self.chain.len() - 1];
        let new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
        );
        let mut block_to_add = new_block.clone();
        block_to_add.mine_block(self.difficulty);
        self.chain.push(block_to_add);
    }

    // 验证区块链
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    // 同步区块链状态（重新构建内存状态）
    pub fn rebuild_state(&mut self) {
        self.users.clear();
        self.online_users.clear();
        self.connections.clear();
        
        for block in &self.chain {
            if let Ok(transaction) = serde_json::from_str::<ChatTransaction>(&block.data) {
                match transaction {
                    ChatTransaction::UserRegister(user) => {
                        self.users.insert(user.address.clone(), user);
                    },
                    ChatTransaction::StatusUpdate(status) => {
                        if status.status == "online" {
                            self.online_users.insert(status.address.clone(), status);
                        } else {
                            self.online_users.remove(&status.address);
                        }
                    },
                    ChatTransaction::ConnectionEstablished(conn) => {
                        self.connections.push(conn);
                    },
                    ChatTransaction::RewardIssued(reward) => {
                        if let Some(user) = self.users.get_mut(&reward.user_address) {
                            user.token_balance += reward.reward_amount;
                        }
                    },
                }
            }
        }
    }
}