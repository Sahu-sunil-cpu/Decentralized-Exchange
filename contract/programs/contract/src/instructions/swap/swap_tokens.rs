use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}};

use crate::{
     error::CustomError, instructions::{transfer_tokens_from, transfer_tokens_to}, math::swap_calculation, Pool, SEED_POOL_ACCOUNT, SEED_POOL_RESERVE_A, SEED_POOL_RESERVE_B
};

/*
both reserve_a_mint and reserve_b_mint, one which is being traded and to be deposited and another tooken of pair
payer_reserve_a_token_account, payer_reserve_b_token_account -> init_if_needed
pool_account,
pool_reserve_a_token_account
pool_reserve_b_token_account
*/
#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub payer_source_token_mint: InterfaceAccount<'info, Mint>,

    pub payer_destination_token_mint: InterfaceAccount<'info, Mint>,

    pub reserve_a_mint: InterfaceAccount<'info, Mint>,

    pub reserve_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [SEED_POOL_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
        bump  = pool_account.bump,
        has_one = reserve_a_mint,
        has_one = reserve_b_mint
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [SEED_POOL_RESERVE_A, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_a_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_POOL_RESERVE_B, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_b_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub payer_source_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub payer_destination_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>
}

/*
CHECK: the user passed mints to the pool reserve mints
CALCULATE: deduct fee and x.y = k, according to this formula, calculate
other token amount
ACTION: deposit the source token into the respective reserve after matching the keys
UPDATE: source token amount in pool account
ACTION: tranfer destination token to the user destination_token_account
UPDATE: destination token amount in pool account
UPDATE: fee and liquidity provider part
*/
pub fn process_swap(
    ctx: Context<SwapTokens>,
    source_token_amount: u64,
) -> Result<()> {
    require!(
        (ctx.accounts.reserve_a_mint.key() == ctx.accounts.payer_source_token_mint.key()
            && ctx.accounts.reserve_b_mint.key()
                == ctx.accounts.payer_destination_token_mint.key()),
        CustomError::MintDoNotMatch
    );

    let source_token_account;
    let destination_token_account;
    let source_reserve_amount;
    let destination_reserve_amount;
    let reserve_pool_seed;

    if ctx.accounts.pool_account.reserve_a_mint.key() == ctx.accounts.payer_source_token_mint.key() {
        source_reserve_amount = &ctx.accounts.pool_account.reserve_a_amount;
        destination_reserve_amount = &ctx.accounts.pool_account.reserve_b_amount;
        source_token_account = &ctx.accounts.reserve_a_token_account;
        destination_token_account = &ctx.accounts.reserve_b_token_account;
        reserve_pool_seed = SEED_POOL_RESERVE_B;
    }else {
        source_reserve_amount = &ctx.accounts.pool_account.reserve_b_amount;
        destination_reserve_amount = &ctx.accounts.pool_account.reserve_a_amount;
        source_token_account = &ctx.accounts.reserve_b_token_account;
        destination_token_account = &ctx.accounts.reserve_a_token_account;
        reserve_pool_seed = SEED_POOL_RESERVE_A;
    }


    transfer_tokens_from(
        &ctx.accounts.payer_source_token_mint,
        &ctx.accounts.payer_source_token_account,
        &source_token_account,
        &ctx.accounts.token_program,
        source_token_amount,
        ctx.accounts.payer_source_token_mint.decimals
      )?;

    let destination_amount = swap_calculation(
            source_token_amount,
            destination_reserve_amount,
            source_reserve_amount,
            ctx.accounts.pool_account.fee_bps,
        )?;


        let key = ctx.accounts.pool_account.key();
        let signer_seeds: &[&[&[u8]]] = &[&[reserve_pool_seed, key.as_ref(), &[ctx.bumps.reserve_a_token_account]]];


    transfer_tokens_to(
        signer_seeds,
        destination_amount, 
        &ctx.accounts.payer_destination_token_mint.decimals,
        &destination_token_account,
        &ctx.accounts.payer_destination_token_account,
        &ctx.accounts.payer_destination_token_mint,
        &ctx.accounts.token_program
    )?;
    Ok(())
}
