
use anchor_lang::accounts::cpi_account::CpiAccount;
use anchor_lang::prelude::*;

use anchor_lang::solana_program::system_program;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{
    
    ID as MetadataTokenId,
};



use mpl_token_metadata::state::{ TokenMetadataAccount};
use mpl_token_metadata::state::{ Metadata};
use std::str::FromStr;

pub mod account;
pub mod constants;
pub mod errors;

use account::*; 
use errors::*;

declare_id!("4VZBMgYfak2krLwYTw9LErqtHsvS3QL3m3kZr6jzWUic");


// Functions   


#[program]
pub mod anchor_nft_staking {
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

    use super::*;

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        // let uawallet = Pubkey::from_str("5CxVLBRdfpC5gn6vX1d3MWcPxdfHYuK459dcSjYk9DrVs").unwrap();
        let deploymentwallet = Pubkey::from_str("BWxYFcNv1TacJTkVo39eimrJHWiBkNYn2KRebAbEr6ZV").unwrap();
        let expected_creator =
        Pubkey::from_str("ABbCYvy3FqN7RVp9aXNvT3cPotzFNMrQ74pX2VFXpHDa").unwrap();
        // ctx.accounts.user.key() == uawallet ||
        let personalwallet = Pubkey::from_str("3SgV1dMLaFtRDF2cvxqdZFtdk8h6asE8cMmwgY58XJyb").unwrap();
        require!(ctx.accounts.user.key().key() == personalwallet || ctx.accounts.user.key().key() == deploymentwallet || ctx.accounts.user.key().key() == expected_creator, StakeError::NoPrem);
        let balance: u64 = ctx.accounts.global_state.to_account_info().lamports() - (LAMPORTS_PER_SOL / 500);
        msg!(
            "Withdrawing Fees."
        );
        msg!("Withdrawing: {:?}", balance);
        **ctx.accounts.global_state.to_account_info().try_borrow_mut_lamports()? -= balance;
        **ctx.accounts.user.try_borrow_mut_lamports()? += balance;
 

        Ok(())
    }


    pub fn create_staking_account(ctx: Context<NewAccount>) -> Result<()> {
        msg!("Create Account");
        Ok(())
    }


    pub fn stake(ctx: Context<Stake>) -> Result<()> {       
        let clock = Clock::get().unwrap();



        let expected_creator =
        Pubkey::from_str("ABbCYvy3FqN7RVp9aXNvT3cPotzFNMrQ74pX2VFXpHDa").unwrap();

    // // if nft_metadata_account.data_is_empty() {
    // //     return false;
    // // };
    
    
    // //Get the metadata account struct so we can access its values
    
    assert_ne!(ctx.accounts.nft_metadata_account.data_len(), 0);   

    let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
    



    require!(metadata.update_authority == expected_creator, StakeError::TokenNotEligble);
        // let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.nft_metadata_account.to_account_info())?;





   
        let cpi_accounts = Transfer {
            from: ctx.accounts.nft_token_account.to_account_info().clone(),
            to: ctx.accounts.nft_holding_wallet.to_account_info().clone(),
            authority: ctx.accounts.user.to_account_info().clone(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.clone().to_account_info(), cpi_accounts),
            1,
        )?;








        if ctx.accounts.stake_account_state.stake_start_time == 0 {
            ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
        } 
        // let index = ctx.accounts.stake_list.load_mut()?.staked_nfts as usize;
        // ctx.accounts.stake_list.load_mut()?.staked_list[5] = ctx.accounts.nft_mint.key();

        // 
        // VERIFICATION
        // Verify that NFTs is part of the collection 
        
        // Commenting Out For Now

        // Transfer Fee

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.global_state.key(),
            constants::STAKE_FEE,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.global_state.to_account_info(),
            ],
        );

        
        
        let mut walletlist = ctx.accounts.stake_account_list.load_mut()?;
        walletlist.new_user[1] = 1;
        let result = calc_rate(ctx.accounts.stake_account_state.staked_amount as i8 ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
        ctx.accounts.stake_account_state.tokens_owed = result;
        ctx.accounts.stake_account_state.staked_amount += 1;
        new_stake(walletlist, ctx.accounts.nft_mint.key(),&mut  ctx.accounts.stake_account_state);



        


        // ctx.accounts.stake_account_state.stakedtokens.push(ctx.accounts.nft_mint.key());
      

        ctx.accounts.stake_account_state.user_pubkey = ctx.accounts.user.key();
        ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
        ctx.accounts.stake_account_state.is_initialized = true;
        ctx.accounts.global_state.global_nft_count += 1;
        // 
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

        


        
        let mut redeem_amount = calc_rate(ctx.accounts.stake_account_state.staked_amount, ctx.accounts.stake_account_state.stake_start_time, ctx.accounts.stake_account_state.tokens_owed);
        
        if redeem_amount < 0 {
            redeem_amount = 0 
        }
        
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

        Ok(())
    }


pub fn prepunstake(ctx: Context<PrepUnstake>) -> Result<()> {
  
   
    ctx.accounts.global_state.global_nft_count -= 1;
    ctx.accounts.stake_account_state.prev_key_claimed = false;
     


    let clock = Clock::get()?;

    ctx.accounts.stake_account_state.tokens_owed = calc_rate(ctx.accounts.stake_account_state.staked_amount ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
    ctx.accounts.stake_account_state.stake_start_time = clock.unix_timestamp;
    remove_stake(ctx.accounts.stake_account_list.load_mut()?, ctx.accounts.nft_mint.key(), ctx.accounts.system_program.key(), &mut ctx.accounts.stake_account_state);
    ctx.accounts.stake_account_state.prev_key = true;   

        
    ctx.accounts.stake_account_state.staked_amount -= 1;
    Ok(())
}


    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        // require!(
        //     ctx.accounts.stake_account_state.is_initialized,
        //     StakeError::UninitializedAccount
        // );
       
        // require!(
        // ctx.accounts.stake_account_state.prevunstake.token_mint == ctx.accounts.nft_mint.key(),
        // StakeError::NoPrepForUnstake
        // );
        ctx.accounts.stake_account_state.prev_key_claimed =true;
        ctx.accounts.stake_account_state.prev_key = false;
        


        let seeds =  [b"account_global".as_ref(),   &[*ctx.bumps.get("global_state").unwrap()]];
        
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.nft_holding_wallet.to_account_info().clone(),
            to: ctx.accounts.nft_token_account.to_account_info().clone(),
            authority: ctx.accounts.global_state.to_account_info().clone(),
        };
        
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.clone().to_account_info(), 
                cpi_accounts,
                signer
            ),
            1,
        )?;

        
        



        let clock = Clock::get()?;


        
   
        
        let redeem_amount = calc_rate(ctx.accounts.stake_account_state.staked_amount as i8 ,ctx.accounts.stake_account_state.stake_start_time ,ctx.accounts.stake_account_state.tokens_owed);
       

        
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
        ctx.accounts.stake_account_state.tokens_owed = 0;   


        Ok(())
    }
}



// Account Section 

#[derive(Accounts)]
pub struct WithdrawFees<'info> {

    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<GlobalStake>() + 12,
        seeds = [b"account_global".as_ref()],
        bump
    )]
    pub global_state: Account<'info, GlobalStake>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewAccount<'info> {
    #[account(
        init_if_needed,
        space = 2 * 1024,
        payer = user, 
        seeds = [user.key().as_ref(), b"infamousstakingnew".as_ref()],
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
        associated_token::authority=user,
        constraint = nft_token_account.mint == *nft_mint.to_account_info().key,
    )]
    
    pub nft_token_account: CpiAccount<'info, TokenAccount>,
    
    pub nft_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        seeds=[b"stakewallet".as_ref(), user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        token::mint=nft_mint,
        token::authority=global_state,
    )]
    pub nft_holding_wallet: Account<'info, TokenAccount>,
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
    pub stake_account_state: Box<Account<'info, UserStakeInfo>>,
    #[account(
        init_if_needed,
        payer=user,
        space = std::mem::size_of::<GlobalStake>() + 12,
        seeds = [b"account_global".as_ref()],
        bump
    )]
    pub global_state: Account<'info, GlobalStake>,
    // #[account(zero)]
    // pub stake_list: AccountLoader<'info, StakedTokenINfo>
    #[account(
        mut,
        seeds=[b"stakewallet".as_ref(), user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        token::mint=nft_mint,
        token::authority=global_state,
    )]
    pub nft_holding_wallet: Box<Account<'info, TokenAccount>>,
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