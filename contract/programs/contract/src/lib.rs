#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use constant::*;
use instructions::*;
use math::*;
use state::*;
mod constant;
mod error;
mod instructions;
mod math;
mod state;
declare_id!("5cpMfyieK8CnhDMZcvgdZiad7dKfhkSoe7TtaEzEH7zh");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize_pool(ctx: Context<PoolAccount>, fee_bps: u16) -> Result<()> {
        process_init_pool(ctx, fee_bps)
    }

    
    pub fn initialize_liquidity_provider(ctx: Context<LiquidityProviderAccount>) -> Result<()> {
        process_init_liquidity_provider(ctx)
    }

    pub fn initialize_lp_token_account(ctx: Context<LpTokenAccount>) -> Result<()> {
        process_init_lp_token_account(ctx)
    }

     pub fn initialize_reserve_a(ctx: Context<InitReserveA>) -> Result<()> {
        process_init_reserve_a(ctx)
    }

      pub fn initialize_reserve_b(ctx: Context<InitReserveB>) -> Result<()> {
        process_init_reserve_b(ctx)
    }

    pub fn update_pool(ctx: Context<UpdatePoolAccount>, fee_bps: u16) -> Result<()> {
        process_update(ctx, fee_bps)
    }

    pub fn deposit(
        ctx: Context<LiquidateDepositAccount>,
        amount_a_token: u64,
        amount_b_token: u64,
    ) -> Result<()> {
        process_deposit(ctx, amount_a_token, amount_b_token)
    }

    pub fn mint_token(ctx: Context<LiquidateMintTokenAccount>, amount: u64) -> Result<()> {
        process_mint_tokens(ctx, amount)
    }

    pub fn withdraw(ctx: Context<DeLiquidateWithdrawAccount>, amount: u64) -> Result<()> {
        process_withdraw(ctx, amount)
    }

    pub fn burn_token(ctx: Context<DeLiquidateBurnAccount>, amount: u64) -> Result<()> {
        process_burn(ctx, amount)
    }


    pub fn swap(ctx: Context<SwapTokens>, source_token_amount: u64) -> Result<()> {
        process_swap(ctx, source_token_amount)
    }
}
