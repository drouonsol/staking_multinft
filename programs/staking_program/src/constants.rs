
// SUPPLY AND MAX 

use std::str::FromStr;

use anchor_lang::{prelude::Pubkey, solana_program::native_token::LAMPORTS_PER_SOL};

pub const NFT_MAX: usize = 48;  
pub const TOTAL_SUPPLY: usize = 10000;

//  Token Related Constants
pub const TOKEN_DECIMALS: u32 = 2;
pub const DAILY_REWARDS: i64 = 100;
pub const STAKE_FEE: u64 = (LAMPORTS_PER_SOL / 250);

// Time Constants 
pub const DAY_IN_SEC: i64 = 86000; // Seconds In A day 



//Pubkey Constants 

