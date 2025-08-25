use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

use crate::{
    check_token_ratio, constant::SEED_LPTOKEN_ACCOUNT, deposit_tokens_internal, first_provider_liquidity, further_provider_liquidity, mint_tokens_internal, state::LpToken, LiquidityProvider, Pool, SEED_LIQUIDITY_PROVIDER, SEED_LP_MINT, SEED_POOL_ACCOUNT, SEED_POOL_RESERVE_A, SEED_POOL_RESERVE_B
};

#[derive(Accounts)]
pub struct LiquidateDepositAccount<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    //liquidity provider will pass ata to deposit token in pool
    #[account(mut)]
    pub depositor_reserve_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub depositor_reserve_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [SEED_POOL_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
        bump  = pool_account.bump,
        has_one = reserve_a_mint,
        has_one = reserve_b_mint
    )]
    pub pool_account: Account<'info, Pool>,

    pub reserve_a_mint: InterfaceAccount<'info, Mint>,

    pub reserve_b_mint: InterfaceAccount<'info, Mint>,   
    
    #[account(
      seeds = [SEED_LPTOKEN_ACCOUNT, depositor.key().as_ref()],
      bump = lptoken_account.bump
    )]
    pub lptoken_account: Account<'info, LpToken>,

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

     #[account(
        //changed [SEED_LIQUIDITY_PROVIDER, depositor.key().as_ref()] to below seed because lp can liquidate more than one pool
        seeds = [SEED_LIQUIDITY_PROVIDER, depositor.key().as_ref()], 
        bump = liquidity_provider_account.liquidator_acc_bump
    )]
    pub liquidity_provider_account: Account<'info, LiquidityProvider>,

    pub token_program: Program<'info, Token2022>,
}


#[derive(Accounts)]
pub struct LiquidateMintTokenAccount<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
    mut,
    seeds = [SEED_LP_MINT],
    bump
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    
    #[account(
      seeds = [SEED_LPTOKEN_ACCOUNT, depositor.key().as_ref()],
      bump = lptoken_account.bump
    )]
    pub lptoken_account: Account<'info, LpToken>,

    #[account(
      init_if_needed,
      payer = depositor,
      associated_token::authority = depositor,
      associated_token::token_program = token_program,
      associated_token::mint = lp_token_mint,
    )]
    pub depositor_lp_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/*
CHECK: if liquidity provider account is initialized
CALCULATE: accept a particular ratio of both the tokens in pair
CHECK : that ratio matches to user token_amount
ACTION: deposit reserve_a_token & reserve_b_token in pool
UPDATE: token amounts in liquidity provider account and pool account
CALCULATE: LP tokens amount to be minted for liquidity Provider
ACTION: mint lp_token to the LP token account
UPDATE: liquidity provider account and pool account states
*/
pub fn process_deposit(
    ctx: Context<LiquidateDepositAccount>,
    amount_a_token: u64,
    amount_b_token: u64,
) -> Result<()> {
   
    let reserve_a_amount = ctx.accounts.pool_account.reserve_a_amount;
    let reserve_b_amount = ctx.accounts.pool_account.reserve_b_amount;

    if !ctx.accounts.pool_account.first_liquidity_deposit {
        check_token_ratio(
            amount_a_token as u128,
            amount_b_token as u128,
            reserve_a_amount as u128,
            reserve_b_amount as u128,
        )?;
    }

    ctx.accounts.pool_account.reserve_a_amount += amount_a_token;
    ctx.accounts.pool_account.reserve_b_amount += amount_b_token;

    deposit_tokens_internal(
        &ctx.accounts.reserve_a_mint,
        &ctx.accounts.depositor_reserve_a_ata,
        &ctx.accounts.reserve_a_token_account,
        &ctx.accounts.token_program,
        amount_a_token,
        ctx.accounts.reserve_a_mint.decimals,
    )?;

    deposit_tokens_internal(
        &ctx.accounts.reserve_b_mint,
        &ctx.accounts.depositor_reserve_b_ata,
        &ctx.accounts.reserve_b_token_account,
        &ctx.accounts.token_program,
        amount_b_token,
        ctx.accounts.reserve_b_mint.decimals,
    )?;

    let token_amount;
    let total_liquidity = ctx.accounts.lptoken_account.total_liquidity;

    if ctx.accounts.pool_account.first_liquidity_deposit {
        token_amount =
            first_provider_liquidity(amount_a_token as u128, amount_b_token as u128)
            .expect("first lp error");
    } else {
        token_amount = further_provider_liquidity(
            amount_a_token as u128,
            amount_b_token as u128,
            reserve_a_amount as u128,
            reserve_b_amount as u128,
            total_liquidity as u128,
        )?;
        
    }

    ctx.accounts.liquidity_provider_account.token_amount += token_amount;
    ctx.accounts.liquidity_provider_account.reserve_a_amount += amount_a_token;
    ctx.accounts.liquidity_provider_account.reserve_b_amount += amount_b_token;
    Ok(())
}

pub fn process_mint_tokens(ctx: Context<LiquidateMintTokenAccount>, amount: u64) -> Result<()> {
    mint_tokens_internal(
        &ctx.bumps.lp_token_mint,
        &ctx.accounts.lp_token_mint,
        &ctx.accounts.depositor_lp_token_account,
        amount,
        &ctx.accounts.token_program,
    )?;

    ctx.accounts.lptoken_account.total_liquidity += amount;
    ctx.accounts.lptoken_account.total_liquidity += amount;
    ctx.accounts.lptoken_account.last_update_ts = Clock::get()?.unix_timestamp;
    Ok(())
}