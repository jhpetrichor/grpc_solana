use base64::Engine;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

use crate::model::EventTrait;

use std::error::Error;

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
}

impl EventTrait for BuyEvent {
    fn discriminator() -> [u8; 8] {
        [103, 244, 82, 31, 44, 245, 119, 119]
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

impl EventTrait for CreatePoolEvent {
    fn discriminator() -> [u8; 8] {
        [177, 49, 12, 210, 160, 118, 167, 116]
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

impl EventTrait for SellEvent {
    fn discriminator() -> [u8; 8] {
        [62, 47, 55, 10, 165, 3, 220, 42]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}
