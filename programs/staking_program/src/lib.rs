
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_program;
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
use std::str::FromStr;

pub mod account;
pub mod constants;
pub mod errors;

use account::*; 
use errors::*;

declare_id!("9AdA14dP96xHcB4DMpJMYt1aRN46PuF74JXov7z3KCHU");


// Functions   


#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn create_staking_account(ctx: Context<NewAccount>) -> Result<()> {
    

        msg!(
            "Opening Account"
        );

        Ok(())
    }


    pub fn stake(ctx: Context<Stake>) -> Result<()> {       
        let clock = Clock::get().unwrap();



        let expected_creator =
        Pubkey::from_str("BWxYFcNv1TacJTkVo39eimrJHWiBkNYn2KRebAbEr6ZV").unwrap();

    // // if nft_metadata_account.data_is_empty() {
    // //     return false;
    // // };
    
    
    // //Get the metadata account struct so we can access its values
    msg!("{:?}",ctx.accounts.nft_metadata_account.data_len());
    assert_ne!(ctx.accounts.nft_metadata_account.data_len(), 0);   

    let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
    msg!("{:?}", metadata.update_authority);

    require!(metadata.update_authority == expected_creator, StakeError::TokenNotEligble);
        // let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.nft_metadata_account.to_account_info())?;

































        if ctx.accounts.stake_account_state.stake_start_time == 0 {
            ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
        } 
        // let index = ctx.accounts.stake_list.load_mut()?.staked_nfts as usize;
        // ctx.accounts.stake_list.load_mut()?.staked_list[5] = ctx.accounts.nft_mint.key();

        // msg!("{:?}", ctx.accounts.stake_list.load_mut()?.staked_list);
        // VERIFICATION
        // Verify that NFTs is part of the collection 
        
        // Commenting Out For Now

        let nft_eligble = check_nft(&ctx.accounts.user, &ctx.accounts.nft_mint, &ctx.accounts.nft_token_account,&mut ctx.accounts.metadata_program, &ctx.accounts.nft_metadata_account);


        
        msg!("Approving delegate");
        let mut walletlist = ctx.accounts.stake_account_list.load_mut()?;
        walletlist.new_user[1] = 1;
        let result = calc_rate(ctx.accounts.stake_account_state.staked_amount as i8 ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
        ctx.accounts.stake_account_state.tokens_owed = result;
        ctx.accounts.stake_account_state.staked_amount += 1;
        new_stake(walletlist, ctx.accounts.nft_mint.key(),&mut  ctx.accounts.stake_account_state);
        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nft_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;

        msg!("Freezing token account");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap(); 
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_token_account.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_token_account.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.metadata_program.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;

        // let tokensowed = ctx.accounts. // calc_rate(ctx.accounts.stake_account_state.staked_amount, clock.unix_timestamp, ctx.accounts.stake_account_state.tokens_owed);
        // ctx.accounts.stake_account_state.tokens_owed = tokensowed;
        // ctx.accounts.stake_account_state.stakedtokens.push(ctx.accounts.nft_mint.key());
      

        ctx.accounts.stake_account_state.user_pubkey = ctx.accounts.user.key();
        ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
        ctx.accounts.stake_account_state.is_initialized = true;
        // msg!("{:?}", ctx.accounts.stake_account_state.stakedtokens);
        Ok(())
    }



    pub fn increase_stake_capacity(ctx: Context<IncreaseSpace>) -> Result<()> {
        Ok(())
    }




    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        require!(
            ctx.accounts.stake_account_state.is_initialized,
            errors::StakeError::UninitializedAccount
        );




        let clock = Clock::get()?;

        
        msg!(
            "Stake last redeem: {:?}",
            ctx.accounts.stake_account_state.stake_start_time 
        );

        msg!("Current time: {:?}", clock.unix_timestamp);
        let unix_time = clock.unix_timestamp - ctx.accounts.stake_account_state.stake_start_time ;
        msg!("Seconds since last redeem: {}", unix_time);
        let mut redeem_amount = calc_rate(ctx.accounts.stake_account_state.staked_amount, ctx.accounts.stake_account_state.stake_start_time, ctx.accounts.stake_account_state.tokens_owed);
        msg!("Elligible redeem amount: {}", redeem_amount);
        if redeem_amount < 0 {
            redeem_amount = 0 
        }
        msg!("Minting staking rewards");
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.stake_mint.to_account_info(),
                    to: ctx.accounts.user_stake_ata.to_account_info(),
                    authority: ctx.accounts.stake_authority.to_account_info(),
                },
                &[&[
                    b"mint".as_ref(),
                    &[*ctx.bumps.get("stake_authority").unwrap()],
                ]],
            ),
            redeem_amount.try_into().unwrap(),
        )?;
        ctx.accounts.stake_account_state.tokens_owed = 0;
        ctx.accounts.stake_account_state.stake_start_time  = clock.unix_timestamp;
        msg!(
            "Updated last stake redeem time: {:?}",
            ctx.accounts.stake_account_state.stake_start_time 
        );

        Ok(())
    }


pub fn prepunstake(ctx: Context<PrepUnstake>) -> Result<()> {
  
    let mut index = find_stake(ctx.accounts.stake_account_list.load_mut()?,ctx.accounts.nft_mint.key());

    ctx.accounts.stake_account_state.prev_key_claimed = false;
     msg!("Prepairing Unstake");


    let clock = Clock::get()?;
    let  stakeamount = ctx.accounts.stake_account_list.load_mut()?.amountstaked;
    msg!("{:?}", ctx.accounts.stake_account_list.load_mut()?.mintlist[300]);
    ctx.accounts.stake_account_state.tokens_owed = calc_rate(ctx.accounts.stake_account_state.staked_amount ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
    ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
    remove_stake(ctx.accounts.stake_account_list.load_mut()?, ctx.accounts.nft_mint.key(), ctx.accounts.system_program.key(), &mut ctx.accounts.stake_account_state);
    ctx.accounts.stake_account_state.prev_key = true;   
    ctx.accounts.stake_account_state.staked_amount -= 1;
    Ok(())
}


    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        require!(
            ctx.accounts.stake_account_state.is_initialized,
            StakeError::UninitializedAccount
        );
       
        require!(
        ctx.accounts.stake_account_state.prevunstake.token_mint == ctx.accounts.nft_mint.key(),
        StakeError::NoPrepForUnstake
        );
        ctx.accounts.stake_account_state.prev_key_claimed =true;
        ctx.accounts.stake_account_state.prev_key = false;
        msg!("Thawing token account");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        invoke_signed(
            &thaw_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_token_account.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_token_account.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.metadata_program.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;

        msg!("Revoking delegate");

        let cpi_revoke_program = ctx.accounts.token_program.to_account_info();
        let cpi_revoke_accounts = Revoke {
            source: ctx.accounts.nft_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_revoke_ctx = CpiContext::new(cpi_revoke_program, cpi_revoke_accounts);
        token::revoke(cpi_revoke_ctx)?;

        let clock = Clock::get()?;
      
        msg!(
            "Stake last redeem: {:?}",
            ctx.accounts.stake_account_state.stake_start_time 
        );

        msg!("Current time: {:?}", clock.unix_timestamp);
        let unix_time = clock.unix_timestamp - ctx.accounts.stake_account_state.stake_start_time ;
        msg!("Seconds since last redeem: {}", unix_time);
        let redeem_amount = calc_rate(ctx.accounts.stake_account_state.staked_amount as i8 ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
       

        msg!("Minting staking rewards");
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.stake_mint.to_account_info(),
                    to: ctx.accounts.user_stake_ata.to_account_info(),
                    authority: ctx.accounts.stake_authority.to_account_info(),
                },
                &[&[
                    b"mint".as_ref(),
                    &[*ctx.bumps.get("stake_authority").unwrap()],
                ]],
            ),
            redeem_amount.try_into().unwrap(),
        )?;

        ctx.accounts.stake_account_state.stake_start_time  = clock.unix_timestamp;
        msg!(
            "Updated last stake redeem time: {:?}",
            ctx.accounts.stake_account_state.stake_start_time 
        );
        ctx.accounts.stake_account_state.tokens_owed = 0;   


        Ok(())
    }
}



// Account Section 

#[derive(Accounts)]
pub struct NewAccount<'info> {
    #[account(
        init_if_needed,
        space = 2 * 1024,
        payer = user, 
        seeds = [user.key().as_ref(), b"infamousstakingnewtestY".as_ref()],
        bump
    )]
    pub stake_account_list: AccountLoader<'info, WalletList>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

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
    /// CHECK: No Operation
    pub nft_metadata_account: AccountInfo<'info>,
    /// CHECK: Manual validation
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<UserStakeInfo>() + 12,
        seeds = [user.key().as_ref(), b"stake_global2".as_ref()],
        bump
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    #[account(mut
        )]
    pub stake_account_list: AccountLoader<'info, WalletList>,
    // #[account(zero)]
    // pub stake_list: AccountLoader<'info, StakedTokenINfo>,
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
    pub metadata_program: Program<'info, MetadataTest>,
}


#[derive(Accounts)]
pub struct IncreaseSpace<'info> {
    #[account(
        mut,
        realloc = 1024 * 10 as usize,
        realloc::zero = true, 
        realloc::payer=user
    )]
    pub stake_account_list: AccountLoader<'info, WalletList>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [user.key().as_ref(), b"stake_global2".as_ref()],
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
pub struct PrepUnstake<'info> {
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
        seeds = [user.key().as_ref(), b"stake_global2".as_ref()],
        bump
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    #[account(
    mut    
    )]
    pub stake_account_list: AccountLoader<'info, WalletList>,
    // #[account(zero)]
    // pub stake_list: AccountLoader<'info, StakedTokenINfo>,
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
    pub metadata_program: Program<'info, MetadataTest>,
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
        seeds = [user.key().as_ref(), b"stake_global2".as_ref()],
        bump
    )]
    pub stake_account_state: Account<'info, UserStakeInfo>,
    // #[account(zero)]
    // pub stake_list: AccountLoader<'info, StakedTokenINfo>

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
    pub metadata_program: Program<'info, MetadataTest>,
}

