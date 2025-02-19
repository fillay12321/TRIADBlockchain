use chrono::Utc;
use ed25519_dalek::{Keypair};
use rand::rngs::OsRng;
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, warn, error};
use crate::external_adapter::BlockchainAdapter;
use crate::rest_api::AppState;

mod transaction;
mod block;
mod wallet;
mod token_economy;
mod consensus;
mod consensus_plugin;
mod atomic_swap;
mod external_adapter;
mod smart_contract;
mod mempool;
mod p2p_server;
mod rest_api;

use transaction::{Transaction, TxType, TxOutput};
use block::Block;
use wallet::{TokenWallet, generate_address};
use token_economy::TokenEconomy;
use external_adapter::{DefaultBlockchainAdapter};
use consensus_plugin::PluginManager;



fn main() {
    env_logger::init();
    let mut rng = OsRng;
    let app_state = Arc::new(Mutex::new(AppState {
        blockchain: Vec::new(),
        mempool: Arc::new(Mutex::new(mempool::Mempool::default())),
    }));
    
        let app_state = Arc::new(Mutex::new(rest_api::AppState {
            blockchain: Vec::new(),
            mempool: Arc::new(Mutex::new(mempool::Mempool::default())),
        }));
        
        

    
    // Запуск P2P-сервера
    {
        let mempool_clone = app_state.lock().unwrap().mempool.clone();
        thread::spawn(move || {
            p2p_server::start_p2p_server(mempool_clone);
        });
    }

    let miner_keypair = Keypair::generate(&mut rng);
    let miner_address = generate_address(&miner_keypair.public);
    let mut miner_wallet = TokenWallet::new(miner_address);

    let user_keypair = Keypair::generate(&mut rng);
    let user_address = generate_address(&user_keypair.public);
    let mut user_wallet = TokenWallet::new(user_address);

    let mut economy = TokenEconomy::new(1_000_000, 0.05, 0.01);
    info!("Blockchain simulation started; total supply: {}", economy.total_supply);

    let mut last_block_time = Utc::now().timestamp();

    for i in 1..=20 {
        let previous_hash = {
            let state = app_state.lock().unwrap();
            if state.blockchain.is_empty() {
                "0".into()
            } else {
                state.blockchain.last().unwrap().hash.clone().unwrap_or_default()
            }
        };

        let timestamp = 1675303065 + (i * 1000);

        let mempool_arc = {
            let state = app_state.lock().unwrap();
            state.mempool.clone()
        };
        let txs = mempool_arc.lock().unwrap().take_all();
        let mut transactions = txs;

        if transactions.is_empty() {
            let tx_output = TxOutput {
                asset: "SOL".into(),
                recipient: miner_wallet.address.clone(),
                amount: 10,
            };
            let mut tx = Transaction::new_cross_chain(
                user_wallet.address.clone(),
                vec![tx_output],
                1,
                "Solana".into(),
                "Ethereum".into(),
            );
            tx.payload = Some("0xContractAddress:transfer:100".into());
            tx.tx_type = TxType::ContractCall;
            tx.sign(&user_keypair);
            transactions.push(tx);
        }

        let adapter = DefaultBlockchainAdapter;
        for tx in &transactions {
            if let TxType::ContractCall = tx.tx_type {
                let _ = adapter.call_smart_contract("0xContractAddress", "transfer", "100");
            }
        }

        let mut block = Block::new(i, previous_hash, timestamp, transactions, "Hybrid".into());
        block.sign(&miner_keypair);
        let difficulty = 4 + (i / 2);
        block.mine(difficulty, last_block_time);

        // Использование плагин-системы для проверки блока
        let mut plugin_manager = PluginManager::new();
        // Регистрируем плагины для всех механизмов консенсуса
        plugin_manager.register_plugin(Box::new(consensus::PoW));
        plugin_manager.register_plugin(Box::new(consensus::PoS));
        plugin_manager.register_plugin(Box::new(consensus::DPoS));
        plugin_manager.register_plugin(Box::new(consensus::Tendermint));
        plugin_manager.register_plugin(Box::new(consensus::PoSpace));

        if plugin_manager.validate_block(&block) {
            info!("Block {} validated by consensus plugins", i);
            miner_wallet.add_tokens("TRD", block.miner_reward);
            economy.total_supply += block.miner_reward;
            if let Err(e) = user_wallet.subtract_tokens("TRD", block.transaction_fee) {
                warn!("Fee deduction failed: {}", e);
            } else {
                economy.burn_tokens(block.transaction_fee);
            }
            {
                let mut state = app_state.lock().unwrap();
                if Block::is_unique_hash(&state.blockchain, block.hash.as_ref().unwrap_or(&"".into())) {
                    state.blockchain.push(block);
                    info!("Block {} added to blockchain", i);
                } else {
                    warn!("Block {} has duplicate hash", i);
                }
            }
        } else {
            warn!("Block {} failed consensus plugin validation", i);
        }

        last_block_time = Utc::now().timestamp();
        if i % 5 == 0 {
            economy.apply_inflation();
        }
    }

    let save_path = "blockchain.json";
    {
        let state = app_state.lock().unwrap();
        if let Err(e) = Block::save_to_file(&state.blockchain, save_path) {
            error!("Failed to save blockchain: {}", e);
        } else {
            info!("Blockchain saved to {}", save_path);
        }
    }

    info!("Miner's balance: {}", miner_wallet.get_balance("TRD"));
    info!("User's balance: {}", user_wallet.get_balance("TRD"));
    info!("Final total supply: {}", economy.total_supply);

    {
        let app_state_clone = Arc::clone(&app_state);
        thread::spawn(move || {
            rest_api::start_rest_server(app_state_clone);
        });
    }

    loop {
        thread::park();
    }


}