use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{self, Mint, MintTo, TokenAccount, TransferChecked},
};

use crate::constant::SEED_LP_MINT;

pub fn deposit_tokens_internal<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        mint: mint_account.to_account_info(),
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: from.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token_interface::transfer_checked(cpi_context, amount, decimals)?;
    Ok(())
}

pub fn mint_tokens_internal<'info>(
    bump: &u8,
    mint_account: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_LP_MINT, &[*bump]]];

    let cpi_accounts = MintTo {
        mint: mint_account.to_account_info(),
        to: to.to_account_info(),
        authority: mint_account.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    token_interface::mint_to(cpi_context, amount)?;

    Ok(())
}
