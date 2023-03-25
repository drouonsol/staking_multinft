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

declare_id!("9DgBFkB3cXa4gAFJjkbggw8TmVLjmkAuDMXtqWGe6R9M");

pub mod constants;
pub mod errors;
pub mod accounts;

#[program]
pub mod anchor_nft_staking {
    use super::*;



    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        let mut fixed_pool = ctx.accounts.stake_list.load_mut()?;
        fixed_pool.staked_list[1] = ctx.accounts.nft_mint.key();
        msg!("{:?}", fixed_pool.staked_list);
        let clock = Clock::get().unwrap();
        msg!("Approving delegate");
        
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
        ctx.accounts.stake_account_state.tokens_owed = tokensowed;
        // ctx.accounts.stake_account_state.stakedtokens.push(ctx.accounts.nft_mint.key());
        ctx.accounts.stake_account_state.user_pubkey = ctx.accounts.user.key();
        ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
        ctx.accounts.stake_account_state.is_initialized = true;
        // msg!("{:?}", ctx.accounts.stake_account_state.stakedtokens);
        Ok(())
    }

    pub fn redeem(ctx: Context<accounts::__cpi_client_accounts_redeem::Redeem>) -> Result<()> {
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
        let redeem_amount = (10 * i64::pow(10, 2) * unix_time) / (24 * 60 * 60);
        msg!("Elligible redeem amount: {}", redeem_amount);

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

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        require!(
            ctx.accounts.stake_account_state.is_initialized,
            StakeError::UninitializedAccount
        );



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
        let redeem_amount = (10 * i64::pow(10, 2) * unix_time) / (24 * 60 * 60);
        msg!("Elligible redeem amount: {}", redeem_amount);

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



        Ok(())
    }
}
