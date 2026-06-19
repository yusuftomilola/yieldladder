use soroban_sdk::panic_with_error;

pub const SCALING_FACTOR: i128 = 10_000_000; // 7 decimal places (stroop precision)

#[derive(Copy, Clone, Debug)]
pub enum MathError {
    Overflow = 1,
    DivisionByZero = 2,
}

pub fn mul_fp(a: i128, b: i128) -> i128 {
    let Some(prod) = a.checked_mul(b) else {
        panic!("FixedPoint Math: Multiplication overflow");
    };
    prod / SCALING_FACTOR
}

pub fn div_fp(a: i128, b: i128) -> i128 {
    if b == 0 {
        panic!("FixedPoint Math: Division by zero");
    }
    let Some(scaled_a) = a.checked_mul(SCALING_FACTOR) else {
        panic!("FixedPoint Math: Scaling overflow during division");
    };
    scaled_a / b
}