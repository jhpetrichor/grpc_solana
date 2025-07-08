use std::error::Error;

use base64::Engine;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

use crate::model::EventTrait;

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
}

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator: Pubkey,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
}

impl EventTrait for CreateEvent {
    fn discriminator() -> [u8; 8] {
        [27, 114, 169, 77, 222, 235, 99, 118]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }

    fn parse_logs<T: EventTrait + Clone>(logs: &[String]) -> Option<T> {
        // println!("{:#?}", logs);
        // std::process::exit(1);
        logs.iter().rev().find_map(|log| {
            let payload = log.strip_prefix(super::PROGRAM_DATA)?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(payload)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                .ok()?;

            let (discr, rest) = bytes.split_at(8);
            if Self::valid_discrminator(discr) {
                T::from_bytes(rest).ok()
            } else {
                None
            }
        })
    }
}

impl EventTrait for CompleteEvent {
    fn discriminator() -> [u8; 8] {
        [95, 114, 97, 156, 212, 46, 152, 8]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for TradeEvent {
    fn discriminator() -> [u8; 8] {
        [189, 219, 127, 211, 78, 230, 97, 238]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}
