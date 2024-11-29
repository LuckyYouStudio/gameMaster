mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    // 创建一个新的区块链，设置挖矿难度为 2（即前两个字符为 '00'）
    let mut blockchain = Blockchain::new(2);

    // 添加一些区块
    blockchain.add_block("Block 1 Data".to_string());
    blockchain.add_block("Block 2 Data".to_string());

    // 输出区块链的内容
    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    // 验证区块链的有效性
    println!("Blockchain valid: {}", blockchain.is_valid());
}
