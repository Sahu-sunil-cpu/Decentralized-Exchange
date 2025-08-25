use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{burn, Burn, Token2022}, token_interface::{self, Mint, TokenAccount, TransferChecked}
};

pub fn withdraw_tokens_internal<'info>(
    signer_seeds: &[&[&[u8]]],
    amount: &u64,
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


    //TODO: handle type casting
    token_interface::transfer_checked(cpi_context, *amount, *mint_decimals)

}

pub fn burn_tokens_internal<'info>(
 mint: &InterfaceAccount<'info, Mint>,
 from: &InterfaceAccount<'info, TokenAccount>,
 token_program: &Program<'info, Token2022>,
 authority: &Signer<'info>,
 amount: u64,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint.to_account_info(),
                from: from.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}