mod block;
mod blockchain;
mod chat_data;
mod chat_blockchain;
mod chat_api;

use std::sync::Arc;
use tokio::sync::Mutex;
use chat_blockchain::ChatBlockchain;
use chat_api::start_api_server;

#[tokio::main]
async fn main() {
    println!("🚀 Starting ChatMaster Blockchain Node...");
    
    // 创建聊天区块链实例
    let chat_blockchain = ChatBlockchain::new(2); // 挖矿难度为2
    let blockchain_arc = Arc::new(Mutex::new(chat_blockchain));
    
    println!("✅ ChatBlockchain initialized with mining difficulty 2");
    
    // 启动API服务器
    start_api_server(blockchain_arc).await;
}
