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




#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Manual validation
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<UserStakeInfo>() + 12,
        seeds = [user.key().as_ref(), b"stake_global".as_ref()],
        bump
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    #[account(zero)]
    pub stake_list: AccountLoader<'info, StakedTokenINfo>,
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<GlobalStake>() + 12,
        seeds = [b"account_global".as_ref()],
        bump
    )]
    pub global_state: Account<'info, GlobalStake>,

    /// CHECK: Manual validation
    #[account(mut, seeds=["authority".as_bytes().as_ref()], bump)]
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [user.key().as_ref(), b"stake_global".as_ref()],
        bump,
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    #[account(mut)]
    pub stake_mint: Account<'info, Mint>,
    /// CHECK: manual check
    #[account(seeds = ["mint".as_bytes().as_ref()], bump)]
    pub stake_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=stake_mint,
        associated_token::authority=user
    )]
    pub user_stake_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Manual validation
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [user.key().as_ref(), b"stake_global".as_ref()],
        bump
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    /// CHECK: manual check
    #[account(mut, seeds=["authority".as_bytes().as_ref()], bump)]
    pub program_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub stake_mint: Account<'info, Mint>,
    /// CHECK: manual check
    #[account(seeds = ["mint".as_bytes().as_ref()], bump)]
    pub stake_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=stake_mint,
        associated_token::authority=user
    )]
    pub user_stake_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,
}

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
    pub staked_list: [Pubkey; NFT_MAX]
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
        self.staked_list[1] = item;
        self.staked_nfts += 1;
        
    }
} 