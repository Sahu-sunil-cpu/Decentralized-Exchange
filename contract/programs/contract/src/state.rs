use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub reserve_a_mint: Pubkey,
    pub reserve_b_mint: Pubkey,
    pub reserve_a_token: Pubkey,
    pub reserve_b_token: Pubkey,
    pub fee_bps: u16,
    pub last_update_ts: i64,
    pub bump: u8,
    pub first_liquidity_deposit: bool,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct LpToken {
    pub total_liquidity: u64,
    pub lp_mint: Pubkey,
    pub bump: u8,
    pub total_fees: u64,
    pub last_update_ts: i64,
}

#[account]
#[derive(InitSpace)]
pub struct LiquidityProvider {
    //can also add liquidator_reserve_a_ata and liquidator_reserve_b_ata
    pub liquidator: Pubkey, //liquidator wallet address
    //pub liquidator_account: Pubkey,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
   // pub lp_token_account: Pubkey, // lp tokens will be minted in this ata
    pub token_amount: u64,
    pub liquidator_acc_bump: u8,
    pub is_initialized: bool,
}
