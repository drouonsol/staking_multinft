use std::array;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};

use crate::constants::NFT_MAX;






#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
    fn id() -> Pubkey {
        MetadataTokenId
    }


}
#[account]
pub struct UserStakeInfo {
    pub stake_start_time: i64,
    pub tokens_owed: i64,
    pub staked_amount: i8,
    pub user_pubkey: Pubkey,
    pub is_initialized: bool, 
  
}

#[account(zero_copy)]
pub struct StakedTokenINfo {
    pub staked_nfts: i8,
    pub staked_list: [Pubkey; 6],
}  


#[account]
pub struct GlobalStake {
    pub global_nft_count: i64
}



// Account Functions 

impl UserStakeInfo {
    pub fn new_stake(&mut self,item: Pubkey) {
        self.staked_amount += 1;
        msg!("Total NFTs Staked: {}", self.staked_amount)
    }

    pub fn calc_rate(&mut self, amountstaked : i8,laststaked: i64,tokensowed: i64) -> i64
     {
        let dailyrwrd = 10;
        let clock = Clock::get().unwrap();
        let staked_seconds = clock.unix_timestamp - laststaked;
        let stakedrate: i64 = (staked_seconds) * (amountstaked as i64) / 60 * 60 * 24 * i64::pow(10, 9) * (dailyrwrd as i64) + tokensowed; 
         msg!("Tokens Owed To User : {}", stakedrate);
        return stakedrate;
    }


    pub fn remove_stake(&mut self, item: Pubkey) {
        self.staked_amount -= 1;
        
    }
    // pub fn new_unstake(&mut self, owner: Pubkey, nft_mint: Pubkey, now: i64) {
    //     require!((self.user_pubkey == owner), StakeError::InvalidOwner);
        
    // }
} 

impl StakedTokenINfo {
    pub fn new_stake(&mut self,item: Pubkey) {
        msg!("Adding New Staked NFT");
        self.staked_list[self.staked_nfts as usize] = item;
        self.staked_nfts += 1;
        
    }

    pub fn remove_nft(&mut self, item: Pubkey, systemprogram: Pubkey) {
        self.staked_nfts -= 1;
        if !(self.staked_list.contains(&item)) {
            msg!("Error.")
        } else {
            self.staked_nfts -= 1;
            self.staked_list[self.staked_nfts as usize] = systemprogram;
        }

    }
} 