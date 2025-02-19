use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::mempool::Mempool;
use log::{info, error};
use std::io::{Read, Write};
use serde_json;

fn handle_client(mut stream: TcpStream, mempool: Arc<Mutex<Mempool>>) {
    let mut buffer = Vec::new();
    if let Err(e) = stream.read_to_end(&mut buffer) {
        error!("Error reading stream: {}", e);
        return;
    }
    match serde_json::from_slice::<crate::transaction::Transaction>(&buffer) {
        Ok(tx) => {
            info!("Received transaction: {:?}", tx);
            mempool.lock().unwrap().add_transaction(tx);
            let _ = stream.write_all(b"Transaction added\n");
        },
        Err(e) => {
            error!("Failed to parse transaction: {}", e);
            let _ = stream.write_all(b"Failed to parse transaction\n");
        }
    }
}

pub fn start_p2p_server(mempool: Arc<Mutex<Mempool>>) {
    let listener = TcpListener::bind("0.0.0.0:7000").expect("Failed to bind");
    info!("P2P server started on port 7000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = Arc::clone(&mempool);
                thread::spawn(move || {
                    handle_client(stream, pool);
                });
            },
            Err(e) => error!("Connection error: {}", e),
        }
    }
}
