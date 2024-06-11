use num_bigfloat::BigFloat;
use num_bigint::BigInt;
use std::{
    ops::{Div, Mul},
    str::FromStr,
};

fn main() {
    let n = BigFloat::from_i32(2000 * 1000 * 20 * 15);
    let uint256_max = BigFloat::from_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
    )
    .unwrap();
    println!("uint256_max {}", uint256_max);

    let nth = BigFloat::from_i32(1).div(&n);
    let target_per = BigFloat::from_str("0.00001").unwrap();
    println!("target_per {}", target_per);
    let nth_root = target_per.pow(&nth);
    println!("n fail chance {}", nth_root);

    let success_chance = BigFloat::from_i32(1).sub(&nth_root);
    println!("success chance {:.100}", success_chance);

    let result = success_chance.mul(uint256_max);
    result.to_string();
    println!("result {:.100}", result);
    //   let uint256_max = BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap();
    //  let new:BigInt = uint256_max.pow(n).mul(0);
}
