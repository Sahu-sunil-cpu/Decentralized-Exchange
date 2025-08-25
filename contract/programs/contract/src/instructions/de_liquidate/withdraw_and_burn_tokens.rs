use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}};

use crate::{SEED_LIQUIDITY_PROVIDER, SEED_LPTOKEN_ACCOUNT, SEED_LP_MINT, SEED_POOL_ACCOUNT, SEED_POOL_RESERVE_A, SEED_POOL_RESERVE_B, burn_tokens_internal, withdraw_tokens_internal, math::calculate_reserves_amount, LiquidityProvider, LpToken, Pool, error::CustomError};

/* 
liquidity_provider_ata_a, liquidity_provider_ata_b, init_if_needed, withdrawer_lp_mint_ata
reserve_a_mint, reserve_b_mint, lp_token_mint,
reserve_a_token, reserve_b_token  
pool_account
liquidator_account 
*/

#[derive(Accounts)]
pub struct DeLiquidateWithdrawAccount<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    
    #[account(mut)]
    pub withdrawer_reserve_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub withdrawer_reserve_b_ata: InterfaceAccount<'info, TokenAccount>,
    
     #[account(mut)]
    pub reserve_a_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub reserve_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_POOL_RESERVE_A, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_a_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

      #[account(
        mut,
        seeds = [SEED_POOL_RESERVE_B, pool_account.key().as_ref()],
        bump
    )]
    pub reserve_b_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

   #[account(
        seeds = [SEED_POOL_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
        bump  = pool_account.bump,
        has_one = reserve_a_mint,
        has_one = reserve_b_mint
    )]
    pub pool_account: Box<Account<'info, Pool>>,

    #[account(
        seeds = [SEED_LIQUIDITY_PROVIDER, withdrawer.key().as_ref(), reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()], 
        bump = liquidity_provider_account.liquidator_acc_bump,
    )]
    pub liquidity_provider_account: Account<'info, LiquidityProvider>,
   
   
    #[account(
      seeds = [SEED_LPTOKEN_ACCOUNT, reserve_a_mint.key().as_ref(), reserve_b_mint.key().as_ref()],
      bump = lptoken_account.bump
    )]
    pub lptoken_account: Account<'info, LpToken>,
    pub token_program: Program<'info, Token2022>,
}


#[derive(Accounts)]
pub struct DeLiquidateBurnAccount<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_LP_MINT],
        bump
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    
    #[account(mut)]
    pub withrawer_lp_token_account: InterfaceAccount<'info, TokenAccount>,

    //TODO: this is needed to be change because there is only one lptoken account is possible due to such seeds
    // needed to be make it same user -> different pool -> different lptoken accounts
    #[account(
      seeds = [SEED_LPTOKEN_ACCOUNT, withdrawer.key.as_ref()],
      bump = lptoken_account.bump
    )]
    pub lptoken_account: Account<'info, LpToken>,

    pub token_program: Program<'info, Token2022>,

}
/*
CHECK: if liquidity provider exists
CHECK: the reserve_tokens amount in the liquidity provider if exists
QUESTION: what if reserve_a_token & reserve_b_token are less than
 what liquidity provider deposited
CHECK: if liquidity provider does exists and the amount to
 withdraw is eqal or less than what liquidated
CHECK: if lp_token_burn_amount is less or equal to lp_token_mint_amount
CALCULATE: what amount of reserve_a_token and reserve_b_token need to transfer
 using lp_token amount given by liquidity provider 
ACTION: transfer both the tokens to liquidity provider ata from token_account of reserves
 maintained in contract
UPDATE: update the reserve token amount in LiquidityProvider account of liquidity
 provider
ACTION: burn LP tokens from lp_token_account provided by liquidity provider
UPDATE: update lp_token_mint_amount in liquidity provider
TODO: later somehow remove the LP accounts if all the amount has exhausted
*/
pub fn process_withdraw(ctx: Context<DeLiquidateWithdrawAccount>, amount: u64) -> Result<()> {
    let liquidity_provider_account = &mut ctx.accounts.liquidity_provider_account;

    //handle error
    require!((liquidity_provider_account.reserve_a_amount > 0 && liquidity_provider_account.reserve_b_amount > 0), CustomError::FoundReserveTokensNull);
    require!((amount >= liquidity_provider_account.token_amount), CustomError::LpTokenBurnAmountNotSatisfied);
    
    let reserve_a_amount = ctx.accounts.pool_account.reserve_a_amount as u128;
    let reserve_b_amount = ctx.accounts.pool_account.reserve_b_amount as u128;
    let total_liquidity = ctx.accounts.lptoken_account.total_liquidity as u128;

    let (amount_a, amount_b) = calculate_reserves_amount(reserve_a_amount, reserve_b_amount, amount, total_liquidity)?;
    
    //withdraw amount_a
    let key =  ctx.accounts.pool_account.key();
    let signer_seeds_a: &[&[&[u8]]] = &[&[
        SEED_POOL_RESERVE_A,
        key.as_ref(),
        &[ctx.bumps.reserve_a_token_account]
        ]];
    
     withdraw_tokens_internal(
        signer_seeds_a, 
        &amount_a, 
        &ctx.accounts.reserve_a_mint.decimals,
        &ctx.accounts.reserve_a_token_account,
        &ctx.accounts.withdrawer_reserve_a_ata, 
        &ctx.accounts.reserve_a_mint, 
        &ctx.accounts.token_program
        )?;

    //withdraw amount_b
    let signer_seeds_b: &[&[&[u8]]] = &[&[
        SEED_POOL_RESERVE_B,
        key.as_ref(),
        &[ctx.bumps.reserve_b_token_account]
        ]];
    
     withdraw_tokens_internal(
        signer_seeds_b, 
        &amount_b, 
        &ctx.accounts.reserve_b_mint.decimals,
        &ctx.accounts.reserve_b_token_account,
        &ctx.accounts.withdrawer_reserve_b_ata, 
        &ctx.accounts.reserve_b_mint, 
        &ctx.accounts.token_program
        )?;

    ctx.accounts.liquidity_provider_account.reserve_a_amount -= amount_a;
    ctx.accounts.liquidity_provider_account.reserve_b_amount -= amount_b;
    ctx.accounts.pool_account.reserve_a_amount -= amount_a;
    ctx.accounts.pool_account.reserve_b_amount -= amount_b;

    Ok(())
}


pub fn process_burn(ctx: Context<DeLiquidateBurnAccount>, amount: u64) -> Result<()> {
        burn_tokens_internal(
            &ctx.accounts.lp_token_mint,
            &ctx.accounts.withrawer_lp_token_account,
            &ctx.accounts.token_program,
            &ctx.accounts.withdrawer,
            amount
         )?;

        ctx.accounts.lptoken_account.total_liquidity -= amount;
    Ok(())
}