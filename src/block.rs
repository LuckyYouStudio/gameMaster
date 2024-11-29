use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    // 创建一个新区块
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();  // 初始化时计算哈希
        block
    }

    // 计算区块的哈希值
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())  // 返回哈希值的十六进制表示
    }

    // 挖矿：通过调整 nonce 值使得哈希值符合特定要求（例如前两个字符为 '00'）
    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);  // 挖矿难度
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}
