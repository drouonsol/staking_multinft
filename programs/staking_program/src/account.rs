use std::array;
use std::cell::RefMut;
use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::{SECONDS_PER_DAY, self};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::stake;
use anchor_lang::solana_program::stake::state::Meta;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};


use mpl_token_metadata::state::{ PREFIX, EDITION, TokenMetadataAccount};
use mpl_token_metadata::state::{ Metadata};

use crate::constants::{NFT_MAX, TOKEN_DECIMALS, DAILY_REWARDS, DAY_IN_SEC};
use crate::errors;





#[derive(Clone)]
pub struct MetadataTest;

impl anchor_lang::Id for MetadataTest {
    fn id() -> Pubkey {
        MetadataTokenId
    }


}


#[zero_copy]
#[derive(Default, PartialEq)]
pub struct MintInfo {
    pub token_mint: Pubkey,
    pub preped: bool
}

#[derive(Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MintInfo2 {
    pub token_mint: Pubkey,
    pub preped: bool
}
#[account]
pub struct UserStakeInfo {
    pub stake_start_time: i64,
    pub tokens_owed: i64,
    pub staked_amount: i8,
    pub user_pubkey: Pubkey,
    pub prev_key: bool,
    pub prev_key_claimed: bool,
    pub is_initialized: bool, 
    pub prevunstake: MintInfo2,
    // pub prev_unstake: Pubkey,
}



#[account(zero_copy)]
#[repr(packed)]
pub struct WalletList {
    pub amountstaked: i64,
    pub account_grown: bool,
    pub special_boosters: i64,
    pub new_user: [i8; 7],
    pub mintlist: [Pubkey; 63], 
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



pub fn new_stake(mut account: RefMut<WalletList>,tokenmint: Pubkey,stakeinfo: &mut  anchor_lang::prelude::Account<UserStakeInfo>) {
    let system_progrm =
    Pubkey::from_str("11111111111111111111111111111111").unwrap();
    
    msg!("Adding Current NFT to list");
    let index = account.mintlist.iter().position(|&x| x == system_progrm).unwrap();
   
    account.mintlist[index as usize] = tokenmint;
    msg!("Index: {:?}", index);
    msg!("Done: Added: {:?}", account.mintlist[index as usize]);

    msg!("Amount Staked: {:?}", stakeinfo.staked_amount)
    
}

pub fn remove_stake(mut account: RefMut<WalletList>,tokenmint: Pubkey, system_prgm: Pubkey,stakeglobal: &mut anchor_lang::prelude::Account<UserStakeInfo>) -> usize {
    let index = account.mintlist.iter().position(|&x| x == tokenmint).unwrap();
    let stakecount = account.amountstaked;
    stakeglobal.prevunstake.preped = true;
    stakeglobal.prevunstake.token_mint = tokenmint;
    account.mintlist[index] = system_prgm;
 
    msg!("Index : {:?}", index);
    
    return index;
}

pub fn find_stake(mut account: RefMut<WalletList>,tokenmint: Pubkey) -> usize {
    let index = account.mintlist.iter().position(|&x| x == tokenmint).unwrap();

    return index;
}

pub fn check_if_stake(mut account: RefMut<WalletList>, tokenmint: Pubkey) -> bool {
    if account.mintlist.iter().any(|&x| x == tokenmint) {
        return true;
    } else {
        return false;
    }
}





pub fn calc_rate( amountstaked : i8,laststaked: i64,tokensowed: i64) -> i64 {
    let clock = Clock::get().unwrap();
    msg!("{:?}", clock.unix_timestamp);
    msg!("{:?}",amountstaked);
    msg!("{:?}",laststaked);
    msg!("{:?}",tokensowed);
   let dailyrwrd = 10;
   
   let staked_seconds = clock.unix_timestamp - laststaked;
   msg!("{:?}",staked_seconds);
   //

   let stakedrate: i64 = (staked_seconds) * 100 * 10 *  (amountstaked as i64) / DAY_IN_SEC + tokensowed; 
    msg!("Tokens Owed To User : {}", stakedrate);
   return stakedrate;
}



pub fn check_nft(user: &Signer, nft_mint: &Account<Mint>, token_account: &Account<TokenAccount>,metadata_program: &mut Program<MetadataTest>, nft_metadata_account: &AccountInfo) {




// msg!("{:?}",full_metadata_clone.as_ref().unwrap().data.creators.as_ref().unwrap()[0].address);
let expected_creator =
    Pubkey::from_str("BWxYFcNv1TacJTkVo39eimrJHWiBkNYn2KRebAbEr6ZV").unwrap();

//     if full_metadata_clone.as_ref().unwrap().data.creators.as_ref().unwrap()[0].address == expected_creator {

//     msg!("NFT MATCHES CONGRATS");   
//     return true ;  

//   } else {
//         return true;

//        msg!("Token Cannot Be Staked");
//   }

    // assert_eq!(
    //     full_metadata_clone.as_ref().unwrap().data.creators.as_ref().unwrap()[0].address,
    //     expected_creator
    // );
   

}


// Account Functions 

