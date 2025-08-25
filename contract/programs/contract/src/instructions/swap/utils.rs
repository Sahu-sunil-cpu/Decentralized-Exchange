use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{Token2022}, token_interface::{self, Mint, TokenAccount, TransferChecked}
};


pub fn transfer_tokens_from<'info>(
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

pub fn transfer_tokens_to<'info>(
    signer_seeds: &[&[&[u8]]],
    amount: u64,
    mint_decimals: &u8,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    token_program: &Program<'info, Token2022>
) -> Result<()> {

    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: from.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts)
    .with_signer(signer_seeds);

    token_interface::transfer_checked(cpi_context, amount, *mint_decimals)?;
    Ok(())
}
