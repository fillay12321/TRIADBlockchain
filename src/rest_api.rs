use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::sync::{Arc, Mutex};
use crate::block::Block;
use crate::mempool::Mempool;


#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AppState {
    pub blockchain: Vec<Block>,
    #[serde(skip)]
    pub mempool: Arc<Mutex<Mempool>>,
}

pub async fn get_blocks(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().unwrap();
    HttpResponse::Ok().json(&state.blockchain)
}

pub async fn get_status(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().unwrap();
    let status = json!({
        "block_count": state.blockchain.len(),
        "last_block_hash": state.blockchain.last().and_then(|b| b.hash.clone()).unwrap_or_default(),
    });
    HttpResponse::Ok().json(status)
}

pub async fn add_transaction(data: web::Data<Arc<Mutex<AppState>>>, new_tx: web::Json<crate::transaction::Transaction>) -> impl Responder {
    let state = data.lock().unwrap();
    state.mempool.lock().unwrap().add_transaction(new_tx.into_inner());
    HttpResponse::Ok().json(json!({"status": "transaction added"}))
}

pub fn start_rest_server(app_state: Arc<Mutex<AppState>>) {
    let sys = actix_web::rt::System::new();
    sys.block_on(async move {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Arc::clone(&app_state)))
                .route("/blocks", web::get().to(get_blocks))
                .route("/status", web::get().to(get_status))
                .route("/transaction", web::post().to(add_transaction))
        })
        .bind("127.0.0.1:8080")
        .expect("Failed to bind REST server")
        .run()
        .await
        .expect("REST server error");
    });
}
