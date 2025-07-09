use borsh::error;
use futures_util::{SinkExt, StreamExt, lock, sink::Fanout};
use log::{debug, error, info};
use solana_sdk::{account, bs58};
use std::{collections::HashMap, iter::zip, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, PingRequest, SubscribeRequest, SubscribeRequestAccountsDataSlice,
    SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions, SubscribeRequestPing,
    SubscribeUpdateTransaction, subscribe_update::UpdateOneof,
};

use crate::handle::EventHandler;

const CONNECT_TIMEOUT: u64 = 10;
const KEEP_ALIVE_TIMEOUT: u64 = 60;

#[derive(Clone)]
pub struct YellowstoneGrpc {
    endpoint: String,
    #[allow(dead_code)]
    x_token: Option<String>,
    pub event_handler: Arc<Mutex<EventHandler>>,
    // pub client:
}

impl YellowstoneGrpc {
    pub fn new(endpoint: String, x_token: Option<String>) -> Self {
        Self {
            endpoint,
            x_token,
            event_handler: Arc::new(Mutex::new(EventHandler::new())),
            // client:
        }
    }

    // fn get_client() {
    //     GeyserGrpcClient::build_from_shared(self.endpoint.clone()).unwrap()
    //         .tls_config(ClientTlsConfig::new().with_native_roots()).unwrap()
    //         .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT))
    //         .keep_alive_while_idle(true)
    //         .timeout(Duration::from_secs(KEEP_ALIVE_TIMEOUT))
    //         .connect()
    //         .await.unwrap();
    // }

    pub async fn subscribe(&self, program_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT))
            .keep_alive_while_idle(true)
            .timeout(Duration::from_secs(KEEP_ALIVE_TIMEOUT))
            .connect()
            .await?;

        let client = Arc::new(Mutex::new(client));
        let addrs = vec![program_id];
        let subscribe_request = SubscribeRequest {
            transactions: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: addrs,
                    account_exclude: vec![],
                    account_required: vec![],
                },
            )]),
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        let subscribe_request = SubscribeRequest {
            transactions: HashMap::from([(
                "".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: vec![
                        "BVdVonejnHwKAVFKx1YpQaBc8t225hFuzjns5ZMEq3Pp".to_string(),
                    ],
                    account_exclude: vec![],
                    account_required: vec![],
                },
            )]),
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        let (mut subscribe_tx, mut stream) = client
            .lock()
            .await
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => match msg.update_oneof {
                    Some(UpdateOneof::Transaction(sut)) => {
                        if let Some(meta) = sut.transaction.clone().and_then(|t| t.meta) {
                            let logs = meta.log_messages;
                            if !logs.is_empty() {
                                let slot = sut.slot;
                                let signature = sut
                                    .transaction
                                    .and_then(|t| t.transaction)
                                    .and_then(|t| t.signatures.first().cloned())
                                    .map(|sig| bs58::encode(sig).into_string())
                                    .unwrap_or_else(|| "unknown".to_string());

                                let mut event_handler = self.event_handler.lock().await;
                                event_handler.handle_logs(&logs, slot, signature).await?;
                            }
                        }
                    }
                    Some(UpdateOneof::Ping(_)) => {
                        let _ = subscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await;
                        debug!("Ping sent");
                    }
                    _ => {}
                },
                Err(e) => {
                    error!("Error: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn subscribe_account(
        &self,
        program_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT))
            .keep_alive_while_idle(true)
            .timeout(Duration::from_secs(KEEP_ALIVE_TIMEOUT))
            .connect()
            .await?;

        let client = Arc::new(Mutex::new(client));

        let subscribe_request = SubscribeRequest {
            transactions: HashMap::from([(
                "".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: vec![
                        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string(),
                    ],
                    account_exclude: vec![],
                    account_required: vec![],
                },
            )]),
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        // let version = client.lock().await.get_version().await.unwrap();
        // println!("{:?}", version);

        let (mut subscribe_tx, mut stream) = client
            .lock()
            .await
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        while let Some(message) = stream.next().await {
            // info!("{:?}", message);
            match message {
                Ok(msg) => match msg.update_oneof {
                    // Some(UpdateOneof::Account(account)) => {
                    //     info!("{:?}", account);
                    // }
                    Some(UpdateOneof::Transaction(ts)) => {
                        // info!("{:?}", ts);
                        if let Some(sut) = ts.transaction {
                            if let Some(mate) = sut.meta {
                                // info!("{:?}", mate);
                                // info!("pre_balances: {:?} \n {:?}", mate.pre_balances, mate.post_balances);
                                // info!("[prost_balances: {:?}, post_balances{:?}", mate.post_balances, mate.post_token_balances);
                                info!("len: {}", mate.post_token_balances.len());
                                zip(mate.pre_token_balances, mate.post_token_balances)
                                    .into_iter()
                                    .for_each(|(a, b)| {
                                        info!(
                                            "{}, {}, {}, {}",
                                            a.mint,
                                            a.ui_token_amount.unwrap().ui_amount_string,
                                            b.mint,
                                            b.ui_token_amount.unwrap().ui_amount_string
                                        );
                                    });
                            }
                        }
                    }
                    _ => {} // Some(UpdateOneof::Ping(ping)) => {
                            //     info!("{:?}", ping);
                            // }
                            // x => {
                            //     info!("{:?}", x)
                            // }
                },
                Err(e) => {
                    error!("Error: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn subscribe_price(
        &self,
        // program_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT))
            .keep_alive_while_idle(true)
            .timeout(Duration::from_secs(KEEP_ALIVE_TIMEOUT))
            .connect()
            .await?;

        let client = Arc::new(Mutex::new(client));
        let subscribe_request = request_sub_wallet();

        let (mut subscribe_tx, mut stream) = client
            .lock()
            .await
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        while let Some(message) = stream.next().await {
            if message.is_err() {
                error!("Error!");
            }
            let message = message.unwrap();
            info!("---------------------");
            // info!("{}", message.)
            info!("{:?}", message.filters);
            info!("时间戳： {:?}", message.created_at);
            match message.update_oneof {
                Some(UpdateOneof::Account(account)) => info!("account: {:?}", account),
                Some(UpdateOneof::Block(block)) => info!("block: {:?}", block),
                Some(UpdateOneof::Entry(entry)) => info!("entry: {:?}", entry),
                Some(UpdateOneof::Slot(slot)) => info!("slot: {:?}", slot),
                Some(UpdateOneof::Transaction(tx)) => info!("subscribeupdateTransaction: {:?}", tx), // 信息最多

                Some(UpdateOneof::Ping(_)) => {
                    let _ = subscribe_tx.send(SubscribeRequest {
                        ping: Some(SubscribeRequestPing { id: 1 }),
                        ..Default::default()
                    }).await;
                }
                _ => {},
            }
        
            info!("---------------------\n");

            // match message {
            //     Ok(msg) => match msg.update_oneof {
            //         Some(UpdateOneof::Account(account)) => {
            //             info!("{:?}", account);
            //         }
            //         a => {info!("{:?}", a)}
            //     },
            //     Err(e) => {
            //         error!("Error: {:?}", e);
            //         break;
            //     }
            // }
        }

        Ok(())
    }
}

fn request_sub_wallet() -> SubscribeRequest {
    SubscribeRequest {
        // accounts: HashMap::from([(
        //     "price".to_string(),
        //     SubscribeRequestFilterAccounts {

        //         // account: vec!["8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj".to_string()],
        //         // owner: vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string()],
        //         ..Default::default()
        //     },
        // )]),
        transactions: HashMap::from([(
            "wellet".to_string(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                signature: None,
                account_include: vec!["5EeGiP8pt14DXY52atNTPnfwP7kxdREHQdgo1NRS4tbq".to_string()],
                ..Default::default()
            },
        )]),
        accounts_data_slice: vec![SubscribeRequestAccountsDataSlice {
            offset: 253,
            length: 16,
        }],
        commitment: Some(CommitmentLevel::Processed.into()),
        ..Default::default()
    }
}

// fn request_price() -> SubscribeRequest {
//     SubscribeRequest {
//         accounts: HashMap::from([(
//             "price".to_string(),
//             SubscribeRequestFilterAccounts {
//                 account: vec!["8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj".to_string()],
//                 owner: vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string()],
//                 ..Default::default()
//             },
//         )]),
//         accounts_data_slice: vec![SubscribeRequestAccountsDataSlice {
//             offset: 253,
//             length: 16,
//         }],
//         commitment: Some(CommitmentLevel::Processed.into()),
//         ..Default::default()
//     }
// }
