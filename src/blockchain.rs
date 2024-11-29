use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    // 创建一个新的区块链，包含创世区块
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        chain.push(genesis_block);
        Blockchain {
            chain,
            difficulty,
        }
    }

    // 获取最新的区块
    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    // 向区块链添加新区块
    pub fn add_block(&mut self, data: String) {
        let latest_block = self.get_latest_block();
        let new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
        );
        let mut block_to_add = new_block.clone();
        block_to_add.mine_block(self.difficulty);  // 挖矿过程
        self.chain.push(block_to_add);
    }

    // 验证区块链是否有效
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            // 验证当前区块的哈希是否正确
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // 验证前一个区块的哈希是否匹配
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
