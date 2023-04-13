use std::array;
use std::cell::RefMut;
use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::SECONDS_PER_DAY;
use anchor_lang::solana_program::program::invoke_signed;
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

use crate::constants::{NFT_MAX, TOKEN_DECIMALS, DAILY_REWARDS, DAY_IN_SEC};
use crate::errors;





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
    pub prev_key: bool,
    pub prev_key_claimed: bool,
    pub is_initialized: bool, 
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

pub fn new_stake(mut account: RefMut<WalletList>,tokenmint: Pubkey, amountstaked: i8,) {
    let system_progrm =
    Pubkey::from_str("11111111111111111111111111111111").unwrap();
    
    msg!("Adding Current NFT to list");
    let index = account.mintlist.iter().position(|&x| x == system_progrm).unwrap();

    account.mintlist[index as usize] = tokenmint;
    msg!("Index: {:?}", index);
    msg!("Done: Added: {:?}", account.mintlist[index as usize]);
    

    msg!("Amount Staked: {:?}", amountstaked)
    
}

pub fn remove_stake(mut account: RefMut<WalletList>,tokenmint: Pubkey, system_prgm: Pubkey) -> Pubkey {
    let index = account.mintlist.iter().position(|&x| x == tokenmint).unwrap();
    let stakecount = account.amountstaked;
    account.mintlist[index] = system_prgm;
    msg!("Index : {:?}", index);
    return tokenmint;
}

pub fn check_if_stake(mut account: RefMut<WalletList>, tokenmint: Pubkey) -> bool {
    if account.mintlist.iter().any(|&x| x == tokenmint) {
        return true;
    } else {
        return false;
    }
}





pub fn calc_rate( amountstaked : i8,laststaked: i64,tokensowed: i64) -> i64 {
    msg!("{:?}",amountstaked);
    msg!("{:?}",laststaked);
    msg!("{:?}",tokensowed);
   let dailyrwrd = 10;
   let clock = Clock::get().unwrap();
   let staked_seconds = clock.unix_timestamp - laststaked;
   msg!("{:?}",staked_seconds);
   //

   let stakedrate: i64 = (staked_seconds) * 100 * 10 *  (amountstaked as i64) / DAY_IN_SEC + tokensowed; 
    msg!("Tokens Owed To User : {}", stakedrate);
   return stakedrate;
}



pub fn check_nft(user: &Signer, nft_mint: &Account<Mint>, token_account: &Account<TokenAccount>,metadata_program: &mut Program<Metadata>, nft_metadata_account: &AccountInfo) -> bool {
    assert_eq!(token_account.owner, user.key())  ;
    assert_eq!(token_account.mint, nft_mint.key());
    assert_eq!(token_account.amount, 1);

    let master_edition_seed = &[
        PREFIX.as_bytes(),
        metadata_program.key.as_ref(),
        token_account.mint.as_ref(),
        EDITION.as_bytes()
    ];

    let (master_edition_key, _master_edition_seed) =
        Pubkey::find_program_address(master_edition_seed, metadata_program.key);
    
    // assert_eq!(master_edition_key, ctx.accounts.nft_mint.key());
          
    let nft_metadata_account = nft_metadata_account;
    let nft_mint_account_pubkey =   nft_mint.key();

    let metadata_seed = &[
        "metadata".as_bytes(),
        metadata_program.key.as_ref(),
        nft_mint_account_pubkey.as_ref(),
    ];


    let (metadata_derived_key, _bump_seed) =
    Pubkey::find_program_address(
        metadata_seed,
        metadata_program.key
    );
//check that derived key is the current metadata account key
assert_eq!(metadata_derived_key, nft_metadata_account.key());

if nft_metadata_account.data_is_empty() {
    return false;
};


//Get the metadata account struct so we can access its values
let metadata_full_account =
    &mut mpl_token_metadata::state::Metadata::from_account_info(&nft_metadata_account);

let full_metadata_clone = metadata_full_account.clone();

let expected_creator =
    Pubkey::from_str("BWxYFcNv1TacJTkVo39eimrJHWiBkNYn2KRebAbEr6ZV").unwrap();
    
    let mut compiled_without_error: bool = false;
    if full_metadata_clone.as_ref().unwrap().data.creators.as_ref().unwrap()[0].address == expected_creator {

    msg!("NFT MATCHES CONGRATS");   
    return true ;  
    compiled_without_error = true;  
  } else {
        return false;
        compiled_without_error = false;
       msg!("Token Cannot Be Staked");
  }

    assert_eq!(
        full_metadata_clone.as_ref().unwrap().data.creators.as_ref().unwrap()[0].address,
        expected_creator
    );


}


// Account Functions 

