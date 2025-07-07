use std::error::Error;

use base64::{Engine, engine::general_purpose};

pub mod pumpamm;
pub mod pumpfun_model;

const PROGRAM_DATA: &str = "Program data: ";

pub trait EventTrait: Sized + std::fmt::Debug {
    fn discriminator() -> [u8; 8];

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;

    fn valid_discrminator(head: &[u8]) -> bool;

    fn parse_logs<T: EventTrait + Clone>(logs: &[String]) -> Option<T> {
        // println!("{:#?}", logs);
        // std::process::exit(1);
        logs.iter().rev().find_map(|log| {
            let payload = log.strip_prefix(PROGRAM_DATA)?;
            let bytes = general_purpose::STANDARD
                .decode(payload)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
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
