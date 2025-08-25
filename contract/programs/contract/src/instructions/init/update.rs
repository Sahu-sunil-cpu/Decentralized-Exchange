use anchor_lang::prelude::*;

use crate::{constant::SEED_POOL_ACCOUNT, state::Pool};

#[derive(Accounts)]
pub struct UpdatePoolAccount<'info> {
    #[account(
        mut,
        seeds = [SEED_POOL_ACCOUNT, pool_account.reserve_a_mint.key().as_ref(), pool_account.reserve_a_mint.key().as_ref()],
        bump = pool_account.bump,
    )]
    pub pool_account: Account<'info, Pool>,

}

pub fn process_update(ctx: Context<UpdatePoolAccount>, fee_bps: u16) -> Result<()> {
    let pool_account  = &mut ctx.accounts.pool_account;
    pool_account.fee_bps = fee_bps;
    Ok(())
}
