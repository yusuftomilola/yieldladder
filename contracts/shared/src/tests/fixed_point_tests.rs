#![cfg(test)]
use crate::fixed_point::{mul_fp, div_fp, SCALING_FACTOR};

#[test]
fn test_fixed_point_basic_and_boundaries() {
    // Test basic precision scaling multiplication matching 7 decimal boundaries
    let a = 10 * SCALING_FACTOR; // 10.0000000
    let b = 2 * SCALING_FACTOR;  // 2.0000000
    assert_eq!(mul_fp(a, b), 20 * SCALING_FACTOR);

    // Test Audit Finding M-02 condition: ensure sub-cent asset parameters (~5 USDC fees) calculate accurately without truncation
    let micro_deposit = 4_500_000; // 0.45 USDC
    let fee_basis = 100_000;       // 1% fee rate factor
    let calculated_fee = mul_fp(micro_deposit, fee_basis);
    assert_eq!(calculated_fee, 45_000); // 0.0045 USDC fee preserved cleanly!

    // Verify division scaling limits
    assert_eq!(div_fp(20 * SCALING_FACTOR, b), 10 * SCALING_FACTOR);
}