use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}};
use crate::{constant::{SEED_LIQUIDITY_PROVIDER, SEED_LPTOKEN_ACCOUNT, SEED_LP_MINT}, error::CustomError, state::{LiquidityProvider, LpToken}, Pool, SEED_POOL_ACCOUNT, SEED_POOL_RESERVE_A, SEED_POOL_RESERVE_B};

#[derive(Accounts)]
pub struct PoolAccount<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(mut)]
  pub reserve_a_mint: InterfaceAccount<'info, Mint>,

  #[account(mut)]
  pub reserve_b_mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    payer = payer,
    space = 8 + Pool::INIT_SPACE,
    seeds = [SEED_POOL_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
    bump
  )]
  pub pool_account: Box<Account<'info, Pool>> ,

  pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct InitReserveA<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

  
  pub reserve_a_mint: InterfaceAccount<'info, Mint>,


  pub reserve_b_mint: InterfaceAccount<'info, Mint>,

    // Existing pool (already initialized earlier)
    #[account(
        mut,
        seeds = [SEED_POOL_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
        bump = pool_account.bump,
    )]
    pub pool_account: Box<Account<'info, Pool>>,

    // You do need the mint typed here for Token-2022 CPI

    #[account(
        init,
        payer = payer,
        token::mint = reserve_a_mint,
        token::authority = pool_account,
        seeds = [SEED_POOL_RESERVE_A, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_a_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitReserveB<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_POOL_ACCOUNT, pool_account.reserve_a_mint.as_ref(), pool_account.reserve_b_mint.as_ref()],
        bump = pool_account.bump,
    )]
    pub pool_account: Box<Account<'info, Pool>>,

    
    pub reserve_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = reserve_b_mint,
        token::authority = pool_account,
        seeds = [SEED_POOL_RESERVE_B, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_b_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}






#[derive(Accounts)]
pub struct LiquidityProviderAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,


    #[account(
        init,
        payer = payer,
        space = 8 + LiquidityProvider::INIT_SPACE,
        //changed [SEED_LIQUIDITY_PROVIDER, depositor.key().as_ref()] to below seed because lp can liquidate more than one pool
        seeds = [SEED_LIQUIDITY_PROVIDER, payer.key().as_ref()], 
        bump
    )]
    pub liquidity_provider_account: Account<'info, LiquidityProvider>,

    pub system_program: Program<'info, System>,

}



#[derive(Accounts)]
pub struct LpTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

  #[account(
    init,
    payer = payer,
    seeds = [SEED_LP_MINT, ],
    bump,
    mint::decimals = 9,
    mint::authority = lp_token_mint,
    mint::freeze_authority = lp_token_mint,
    mint::token_program = token_program
   )]
   pub lp_token_mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    payer = payer,
    space = 8 + LpToken::INIT_SPACE,
    seeds = [SEED_LPTOKEN_ACCOUNT, payer.key().as_ref()],
    bump
   )]
  pub lp_token_account: Box<Account<'info, LpToken>>,
  pub token_program: Program<'info, Token2022>,
  pub system_program: Program<'info, System>,
}

pub fn process_init_pool(ctx: Context<PoolAccount>, fee_bps: u16) -> Result<()> {

    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.authority = ctx.accounts.payer.key();
    pool_account.bump = ctx.bumps.pool_account;
    pool_account.fee_bps = fee_bps;
    pool_account.last_update_ts = Clock::get()?.unix_timestamp;
    pool_account.reserve_a_mint = ctx.accounts.reserve_a_mint.key();
    pool_account.reserve_b_mint = ctx.accounts.reserve_b_mint.key();
    pool_account.reserve_a_amount = 0;
    pool_account.reserve_b_amount = 0;



    Ok(())
}



pub fn process_init_reserve_a(ctx: Context<InitReserveA>) -> Result<()> {
    // optional safety: ensure stored mint matches provided mint
    require_keys_eq!(ctx.accounts.pool_account.reserve_a_mint, ctx.accounts.reserve_a_mint.key(), CustomError::InvalidAccountData);
    ctx.accounts.pool_account.reserve_a_token = ctx.accounts.reserve_a_account.key();
    Ok(())
}

pub fn process_init_reserve_b(ctx: Context<InitReserveB>) -> Result<()> {
    require_keys_eq!(ctx.accounts.pool_account.reserve_b_mint, ctx.accounts.reserve_b_mint.key(), CustomError::InvalidAccountData);
    ctx.accounts.pool_account.reserve_b_token = ctx.accounts.reserve_b_account.key();
    Ok(())
}


pub fn process_init_liquidity_provider(ctx: Context<LiquidityProviderAccount>) -> Result<()> {

   if !ctx.accounts.liquidity_provider_account.is_initialized {
        *ctx.accounts.liquidity_provider_account = LiquidityProvider {
            liquidator: ctx.accounts.payer.key(),
            reserve_a_amount: 0,
            reserve_b_amount: 0,
            token_amount: 0,
            liquidator_acc_bump: ctx.bumps.liquidity_provider_account,
            is_initialized: true,
        };
    }

  Ok(())
}

pub fn process_init_lp_token_account(ctx: Context<LpTokenAccount>) -> Result<()> {

    let lp_token_account = &mut ctx.accounts.lp_token_account;
    lp_token_account.total_fees = 0;
    lp_token_account.total_liquidity = 0;
    lp_token_account.lp_mint = ctx.accounts.lp_token_mint.key();
    lp_token_account.bump = ctx.bumps.lp_token_mint;

  Ok(())
}