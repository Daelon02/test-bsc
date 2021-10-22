use std::fs::{File, read};
use std::io::BufReader;
use ethabi::{ParamType, Contract};
use rust_lib::logger;
use log::{LevelFilter, info, error};
use web3;
use web3::types::{Block, BlockId};
use web3::Error as Web3Error;
use web3::futures::{future, StreamExt};

#[tokio::main]
async fn main() {
    logger::logger_init(LevelFilter::Info);
    let ws = web3::transports::WebSocket::new("wss://data-seed-prebsc-2-s3.binance.org:8545/").await.unwrap();
    let web3 = web3::Web3::new(ws.clone());
    let mut sub = web3.eth_subscribe().subscribe_new_heads().await.unwrap();

   info!("Got subscription id: {:?}", sub.id());

    let mut vec = vec![];

    (&mut sub)
        .take(5)
        .for_each(|x| {
            // info!("Got: {:?}", x);
            vec.push(x.unwrap());
            future::ready(())
        })
        .await;



    for block in vec {
        // info!("{:?}", web3.eth().block_with_txs(web3::types::BlockId::Hash(block.hash.unwrap())).await);
        let some = web3.eth().block_with_txs(web3::types::BlockId::Hash(block.hash.unwrap())).await.unwrap().unwrap();
        for transaction in some.transactions {
            info!("Transaction hash: {:?}", transaction.hash);
                // info!("{:?}", transaction.input.0);
            // info!("{:?}", web3.eth().transaction_receipt(transaction.hash).await);
            let tx = web3.eth().transaction_receipt(transaction.hash).await.unwrap().unwrap();
            //     match transaction.from {
            for log in tx.logs {
                info!("{:?}", ethabi::decode(&[ParamType::Address, ParamType::Address, ParamType::Uint(5)], &log.data.0));
            }
            //     Some(value) => info!("Sender: {:?}", value),
                //     None => error!("Cannot unwrap this option")
                // }

            // for event in contract {
            //
            // }
                // match transaction.to {
                //     Some(value) => info!("Recipient: {:?}", value),
                //     None => error!("Cannot unwrap this option")
                // }
                // info!("{:?}", (ethabi::decode(&[ParamType::Address, ParamType::Address, ParamType::Int(4) ], &*transaction.raw.unwrap().0)));
        }
    }
    sub.unsubscribe().await.unwrap();
}