use std::cmp;

use crate::error::CustomError;
use anchor_lang::prelude::*;

fn int_sqrt(n: u128) -> u128 {
    let mut x0 = n / 2;
    if x0 == 0 {
        return n;
    }
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }    
    x0
}

pub fn check_token_ratio(
    token_a_amount: u128,
    token_b_amount: u128,
    reserve_a_amount: u128,
    reserve_b_amount: u128,
) -> Result<()> {
    // tokne_ratio = reserve_a_amount/reserve_b_amount
    // amount = token_b_amount*ratio
    // and val == token_a_amount

    let ratio = reserve_a_amount.saturating_div(reserve_b_amount);
    if let Some(val) = token_b_amount.checked_mul(ratio) {
        if val != token_a_amount {
            //return error
            return Err(CustomError::TokensAmountNotInExpectedRatio.into());
        }
    } else {
        return Err(CustomError::OverflowInTokenRatioCheck.into());
    }

    Ok(())
}

pub fn first_provider_liquidity(amount_a: u128, amount_b: u128) -> Result<u64> {
    let amount = amount_a
        .checked_mul(amount_b)
        .map(|prod| int_sqrt(prod))
        .ok_or(CustomError::OverflowInLiquidityAmountCalculation)?;

    let final_amount: u64 = amount
        .try_into()
        .map_err(|_| CustomError::OverflowInLiquidityAmountCalculation)?;

    Ok(final_amount)
}

pub fn further_provider_liquidity(
    amount_a: u128,
    amount_b: u128,
    reserve_a: u128,
    reserve_b: u128,
    total_liquidity: u128, //total lp tokens
) -> Result<u64> {
    // liquidity = min( amount_x * total_liquidity / reserve_x, amount_y * total_liquidity / reserve_y )
    let part1 = amount_a
        .checked_mul(total_liquidity)
        .ok_or(CustomError::OverflowInAfterLiquidityAmountCalculation)?
        .checked_div(reserve_a)
        .ok_or(CustomError::OverflowInAfterLiquidityAmountCalculation)?;

    let part2 = amount_b
        .checked_mul(total_liquidity)
        .ok_or(CustomError::OverflowInAfterLiquidityAmountCalculation)?
        .checked_div(reserve_b)
        .ok_or(CustomError::OverflowInAfterLiquidityAmountCalculation)?;

    let amount: u64 = cmp::min(part1, part2)
        .try_into()
        .map_err(|_| CustomError::NumericalOverflow)?;

    Ok(amount)
}

pub fn calculate_reserves_amount(
    reserve_a_amount: u128,
    reserve_b_amount: u128,
    token_burn_amount: u64,
    total_liquidity: u128,
) -> Result<(u64, u64)> {
    // reserve_a_amount = token_burn_amount*reserve_a_amount/total_liquidity
    // reserve_b_amount = token_burn_amount*reserve_b_amount/total_liquidity

    let reserve_a_amount = (token_burn_amount as u128)
        .checked_mul(reserve_a_amount)
        .and_then(|v| v.checked_div(total_liquidity))
        .ok_or(CustomError::OverflowInTokenAmountCalculationAfterBurn)?;

    let reserve_b_amount = (token_burn_amount as u128)
        .checked_mul(reserve_b_amount)
        .and_then(|v| v.checked_div(total_liquidity))
        .ok_or(CustomError::OverflowInTokenAmountCalculationAfterBurn)?;

    let final_reserve_a_amount: u64 = reserve_a_amount
        .try_into()
        .map_err(|_| CustomError::NumericalOverflow)?;

    let final_reserve_b_amount: u64 = reserve_b_amount
        .try_into()
        .map_err(|_| CustomError::NumericalOverflow)?;
    Ok((final_reserve_a_amount, final_reserve_b_amount))
}

pub fn swap_calculation(
    amount_in: u64,
    destination_reserve_amount: &u64,
    source_reserve_amount: &u64,
    fee_bps: u16,
) -> Result<u64> {
    // dy = destination_token, dx = source_token, x = reserve_a_amount, y = reserve_b_amount

    // x*y = k
    let k = (*destination_reserve_amount as u128)
        .checked_mul(*source_reserve_amount as u128)
        .ok_or(CustomError::OverflowInTokenAmountCalculationAfterSwap)?;

    // amount*(10000-fee_bps)/10000
    let fee_numerator = 10_000u128 - fee_bps as u128;
    let amount_in_with_fee = (amount_in as u128)
        .checked_mul(fee_numerator as u128)
        .and_then(|v| v.checked_div(10_000))
        .ok_or(CustomError::OverflowInTokenAmountCalculationAfterSwap)?;
    let new_source_amount = *source_reserve_amount as u128 + amount_in_with_fee;

    let destination_new_amount = *destination_reserve_amount as u128
        - k.checked_mul(new_source_amount)
            .ok_or(CustomError::OverflowInTokenAmountCalculationAfterSwap)?;

    //  let fee_amount = amount_in - amount_in_with_fee;

    let k_new = new_source_amount
        .checked_mul(destination_new_amount)
        .ok_or(CustomError::OverflowInTokenAmountCalculationAfterSwap)?;

    if k_new < k {
        let difference = k.checked_sub(k_new).unwrap();
        // Set a threshold for acceptable rounding error, e.g., 1
        if difference > 1 {
            return Err(CustomError::InvariantKDecreasedTooMuch.into());
        }
    }

    let amount: u64 = destination_new_amount
        .try_into()
        .map_err(|_| CustomError::NumericalOverflow)?;

    Ok(amount)
}

// pub fn check_k_drop_rel(k_before: u128, k_after: u128, ppm: u128) -> Result<()> {
//     // allow ppm parts per million drop, e.g. ppm = 10 (0.001%)
//     if k_after < k_before {
//         let diff = k_before
//             .checked_sub(k_after)
//             .ok_or(CustomError::NumericalOverflow)?;
//         let allowed = k_before
//             .checked_mul(ppm)
//             .ok_or(CustomError::NumericalOverflow)?
//             .checked_div(1_000_000)
//             .ok_or(CustomError::NumericalOverflow)?;
//         if diff > allowed {
//             return err!(CustomError::NumericalOverflow);
//         }
//     }
//     Ok(())
// }
