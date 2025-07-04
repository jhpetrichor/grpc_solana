use std::{clone, collections::HashMap, sync::LazyLock};

use futures_util::StreamExt;
use log::{error, info};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions,
    subscribe_update::UpdateOneof,
};

use crate::grpc::YellowstoneGrpc;

mod grpc;

const PROGRAM_ID1: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
#[allow(dead_code)]
const PROGRAM_ID2: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

#[allow(dead_code)]
static SUBCRIBE_REQUEST: LazyLock<SubscribeRequest> = LazyLock::new(|| SubscribeRequest {
    transactions: HashMap::from([(
        "client".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            signature: None,
            account_include: vec![PROGRAM_ID1.to_string()],
            account_exclude: vec![],
            account_required: vec![],
        },
    )]),
    commitment: Some(CommitmentLevel::Processed.into()),
    ..Default::default()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("YELLOWSTONE_GRPC_URL").expect("YELLOWSTONE_GRPC_UTL must be set");
    let mut client = YellowstoneGrpc::new(url, None).build_client().await?;

    // let res = client.get_block_height(Some(CommitmentLevel::Processed)).await.unwrap();
    // println!("{:?}", res);

    // let res = client.get_latest_blockhash(Some(CommitmentLevel::Processed)).await?;
    // println!("latest_blockhash: {:?}", res);

    let mut respone = client
        .subscribe_with_request(Some(get_subscribe_request()))
        .await?;
    while let Some(it) = respone.1.next().await {
        match it {
            Ok(msg) => match msg.update_oneof {
                Some(UpdateOneof::Transaction(sut)) => {
                    if let Some(meta) = sut.transaction.clone().and_then(|t| t.meta) {
                        let logs = meta.log_messages;
                        info!("logs: {:#?}", logs);
                    }
                }

                Some(UpdateOneof::Ping(_ping)) => {
                    info!("get ping msg!, {:?}", _ping)
                }

                _ => {}
            },
            Err(e) => error!("error code: {}", e.code()),
        };
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}

fn get_subscribe_request() -> SubscribeRequest {
    SubscribeRequest {
        transactions: HashMap::from([(
            PROGRAM_ID1.to_string(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                signature: None,
                account_include: vec![PROGRAM_ID1.to_string()],
                account_exclude: vec![],
                account_required: vec![],
            },
        )]),
        commitment: Some(CommitmentLevel::Processed.into()),
        ..Default::default()
    }
}

#[test]
fn test() {
    println!("{}", 9.35 / 0.5763);
}
