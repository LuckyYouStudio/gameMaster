use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use crate::chat_blockchain::ChatBlockchain;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub address: String,
    pub username: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusUpdateRequest {
    pub address: String,
    pub username: String,
    pub status: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionRequest {
    pub from_address: String,
    pub to_address: String,
    pub connection_type: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

pub async fn start_api_server(blockchain: Arc<Mutex<ChatBlockchain>>) {
    let blockchain_filter = warp::any().map(move || blockchain.clone());

    // GET /health - 健康检查
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&ApiResponse {
                success: true,
                data: Some("ChatBlockchain API is running"),
                message: "OK".to_string(),
            })
        });

    // POST /users/register - 注册用户
    let register = warp::path!("users" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(register_user);

    // POST /users/status - 更新用户状态
    let status_update = warp::path!("users" / "status")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(update_status);

    // POST /connections - 记录连接
    let connection = warp::path("connections")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter.clone())
        .and_then(record_connection);

    // GET /users/online - 获取在线用户
    let online_users = warp::path!("users" / "online")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_online_users);

    // GET /users/{address} - 获取用户信息
    let user_info = warp::path!("users" / String)
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_user_info);

    // GET /blockchain/stats - 获取区块链统计
    let stats = warp::path!("blockchain" / "stats")
        .and(warp::get())
        .and(blockchain_filter.clone())
        .and_then(get_blockchain_stats);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let routes = health
        .or(register)
        .or(status_update)
        .or(connection)
        .or(online_users)
        .or(user_info)
        .or(stats)
        .with(cors);

    println!("ChatBlockchain API server starting on port 3002...");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3002))
        .await;
}

async fn register_user(
    req: RegisterRequest,
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    let success = blockchain.register_user(req.address.clone(), req.username, req.public_key);
    
    Ok(warp::reply::json(&ApiResponse {
        success,
        data: if success { Some(req.address) } else { None },
        message: if success { "User registered successfully".to_string() } else { "User already exists".to_string() },
    }))
}

async fn update_status(
    req: StatusUpdateRequest,
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    blockchain.update_user_status(req.address, req.username, req.status, req.node_id);
    
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        data: Some("Status updated"),
        message: "User status updated successfully".to_string(),
    }))
}

async fn record_connection(
    req: ConnectionRequest,
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let mut blockchain = blockchain.lock().await;
    blockchain.record_connection(req.from_address, req.to_address, req.connection_type);
    
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        data: Some("Connection recorded"),
        message: "Connection recorded successfully".to_string(),
    }))
}

async fn get_online_users(
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let users = blockchain.get_online_users();
    
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        data: Some(users),
        message: "Online users retrieved".to_string(),
    }))
}

async fn get_user_info(
    address: String,
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    let user = blockchain.get_user(&address);
    
    Ok(warp::reply::json(&ApiResponse {
        success: user.is_some(),
        data: user,
        message: if user.is_some() { "User found".to_string() } else { "User not found".to_string() },
    }))
}

async fn get_blockchain_stats(
    blockchain: Arc<Mutex<ChatBlockchain>>,
) -> Result<impl Reply, warp::Rejection> {
    let blockchain = blockchain.lock().await;
    
    #[derive(Serialize)]
    struct Stats {
        block_count: usize,
        user_count: usize,
        online_count: usize,
        connection_count: usize,
        total_rewards: u64,
        is_valid: bool,
    }
    
    let stats = Stats {
        block_count: blockchain.chain.len(),
        user_count: blockchain.users.len(),
        online_count: blockchain.online_users.len(),
        connection_count: blockchain.connections.len(),
        total_rewards: blockchain.total_rewards,
        is_valid: blockchain.is_valid(),
    };
    
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        data: Some(stats),
        message: "Blockchain stats retrieved".to_string(),
    }))
}