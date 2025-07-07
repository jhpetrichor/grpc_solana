use std::{collections::HashMap, sync::LazyLock};

use log::debug;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions};

use crate::grpc::YellowstoneGrpc;

mod grpc;
mod handle;
mod model;
mod account;

const PROGRAM_ID1: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const PROGRAM_ID2: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
const PROGRAM_ID3: &str = "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("YELLOWSTONE_GRPC_URL").expect("YELLOWSTONE_GRPC_UTL must be set");
    // let mut client = YellowstoneGrpc::new(url, None).build_client().await?;
    let client = YellowstoneGrpc::new(url, None);

    debug!("Starting subscription for Pump: {}", PROGRAM_ID1);
    debug!("Starting subscription for PumpAmm: {}", PROGRAM_ID2);

    // 创建一个包含两个程序 ID 的向量
    let program_ids = vec![PROGRAM_ID3];

    // 订阅所有程序
    for program_id in program_ids {
        let client = client.clone();
        tokio::spawn(async move {
            if let Err(e) = client.subscribe(program_id.to_string()).await {
                log::error!("Error subscribing to program {}: {:?}", program_id, e);
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
