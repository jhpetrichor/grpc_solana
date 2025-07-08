use std::str::FromStr;

use sol_trade_sdk::{common::pumpfun::logs_events::PumpfunEvent, grpc::YellowstoneGrpc};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[tokio::test]
async fn test_mint_transction() {
    // Create gRPC client with Yellowstone
    let grpc_url = "https://solana-yellowstone-grpc.publicnode.com:443";
    let x_token = None; // Optional auth token
    let client = YellowstoneGrpc::new(grpc_url.to_string(), x_token).unwrap();

    // Define callback function
    let callback = |event: PumpfunEvent| match event {
        PumpfunEvent::NewDevTrade(trade_info) => {
            println!("Received new dev trade event: {:?}", trade_info);
        }
        PumpfunEvent::NewToken(token_info) => {
            println!("Received new token event: {:?}", token_info);
        }
        PumpfunEvent::NewUserTrade(trade_info) => {
            println!("Received new trade event: {:?}", trade_info);
        }
        PumpfunEvent::NewBotTrade(trade_info) => {
            println!("Received new bot trade event: {:?}", trade_info);
        }
        PumpfunEvent::Error(err) => {
            println!("Received error: {}", err);
        }
    };
    let wallet = Pubkey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA").unwrap();
    unsafe { client.subscribe_pumpfun(callback, Some(wallet)).await.unwrap() };
}

#[test]
fn test_say_hello() {
    println!("{}", "hello");
}