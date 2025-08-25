use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Tokens Unit Not In Correct Ratio")]
    TokensAmountNotInExpectedRatio,

    #[msg("Math overflow: pool reserves too large")]
    NumericalOverflow,

    #[msg("Invariant decreased too much (k dropped).")]
    InvariantKDecreasedTooMuch,
    
    #[msg("Token mint do not much")]
    MintDoNotMatch,

    #[msg("No history of providing liquidity to pool")]
    FoundReserveTokensNull,

    #[msg("Lp token burn amount is greater")]
    LpTokenBurnAmountNotSatisfied,

    #[msg("Both tokens must be in poll expected ratio")]
    TokensRatioNotSatisfied,

    #[msg("Math overflow, try with less token amount")]
    OverflowInTokenRatioCheck,

    #[msg("Math overflow, try with less token amount ")]
    OverflowInLiquidityAmountCalculation,

    #[msg("Math overflow, try with less token amount ")]
    OverflowInTokenAmountCalculationAfterBurn,

    #[msg("Math overflow, try with less token amount ")]
    OverflowInTokenAmountCalculationAfterSwap,

    #[msg("Math overflow, try with less token amount ")]
    OverflowInAfterLiquidityAmountCalculation,

    #[msg("An account's data contents was invalid")]
    InvalidAccountData

}
