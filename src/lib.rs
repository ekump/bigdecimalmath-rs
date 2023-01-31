//! Big Decimal Math
//!
//! A collection of mathematical functions [originally implemented in Java by Richard J. Mathar](https://arxiv.org/abs/0908.3030v3) for [bigdecimal].

use bigdecimal::{BigDecimal, FromPrimitive, One, ToPrimitive, Zero};
mod error;
use error::{BigDecimalMathError, BigDecimalMathResult};
use std::convert::TryFrom;
use num_bigint::{BigInt, ParseBigIntError, Sign, ToBigInt};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Calculates the n-th root of a BigDecimal rounded to the precision implied by x, x^(1/n).
///
/// # Arguments
///  * `n` - the positive root
///  * `x` - the non-negative number we are calculating the n-th root of
///
///  # Example
///
///  ```
///  use bigdecimal::BigDecimal;
///  use std::str::FromStr;
///  use bigdecimalmath::root;
///
///  let n = 4;
///  let x = BigDecimal::from_str("14.75").unwrap();
///  assert_eq!(Ok(BigDecimal::from_str("1.9597").unwrap()), root(n,x));
///  ```
pub fn root(n: isize, x: BigDecimal) -> BigDecimalMathResult {
    if x < BigDecimal::zero() {
        let msg = format!("negative argument {:?} of root", x);
        return Err(BigDecimalMathError::ArithmeticError(msg));
    }
    if n <= 0 {
        let msg = format!("negative power {:?} of root", x);
        return Err(BigDecimalMathError::ArithmeticError(msg));
    }

    if n == 1 {
        return Ok(x);
    }
    // TODO: HELP
    // start the computation from a double precision estimate
    let x_as_f64 = f64::powf(x.to_f64().unwrap(), 1.0 / (n as f64));
    let mut s: BigDecimal = BigDecimal::from_f64(x_as_f64).unwrap();

    // this creates nth with nominal precision of 1 digit
    let nth = BigDecimal::from_isize(n).unwrap();

    // Specify an internal accuracy within the loop which is slightly larger than what is demanded by 'eps' below.
    let xhighpr: BigDecimal = scale_prec(&x, 2);
    let mc_precision_only = 2 + get_prec(&x);

    // Relative accuracy of the result is eps.
    let eps_numerator: f64 = ulp(&x).to_f64().unwrap();
    let eps_denominator: f64 = 2.0 * n as f64 * x.to_f64().unwrap();
    let eps = eps_numerator / eps_denominator;
    loop {
        let mut c = &xhighpr / pow(&s, (n - 1) as i32)?;
        c = c.with_prec(mc_precision_only as u64);
        c = &s - &c;
        let locmc = get_prec(&c);
        c = &c / &nth;
        c = c.with_prec(locmc as u64);
        s = &s - &c;

        if (c.to_f64().unwrap() / s.to_f64().unwrap()) < eps {
            break;
        }
    }

    Ok(s.round(err2prec(eps) as i64))
}

fn err2prec(xerr: f64) -> i32 {
    1 + ((0.5 / xerr).abs().log10()) as i32
}

fn scale_prec(x: &BigDecimal, d: i64) -> BigDecimal {
    let (_, scale) = x.as_bigint_and_exponent();

    x.with_scale(d + scale)
}

// TODO: Is there a faster way to calculate the precision?
fn get_prec(x: &BigDecimal) -> usize {
    let (bigint, _scale) = x.as_bigint_and_exponent();
    bigint.to_string().chars().count()
}

fn ulp(x: &BigDecimal) -> BigDecimal {
    let (_, scale) = x.as_bigint_and_exponent();

    BigDecimal::new(One::one(), scale)
}

fn pow(x: &BigDecimal, n: i32) -> BigDecimalMathResult {
    if !(0..=999999999).contains(&n) {
        return Err(BigDecimalMathError::ArithmeticError(
            "Invalid power operation".to_owned()
        ));
    }

    let (bigint, scale) = x.as_bigint_and_exponent();
    let new_scale = scale * n as i64;

    Ok(BigDecimal::new(bigint.pow(n as u32), new_scale))
}

fn do_round(d: &BigDecimal, precision: isize) -> BigDecimalMathResult {
    // TODO: Impl DivideAndRoundByTenPow
    // TODO: Impl DivdeAndRound
    // TODO: Impl bigTenToThe
    unimplemented!()
}

// TODO: Not 100% sure this is necessary in Rust. Cleanup error message
fn check_scale_non_zero(val: i64) -> Result<i32, BigDecimalMathError> {
    if let Ok(as_int) = i32::try_from(val) {
        return Ok(as_int);
    } else {
       Err(BigDecimalMathError::ArithmeticError("TODO".to_owned())) 
    }
}

// TODO: Follow up on casting to usize
fn divide_and_round_by_ten_pow(int_val: BigInt, ten_pow: i32) -> BigInt {
    let mut new_int_val;

    if (ten_pow as usize) < LONG_TEN_POWERS_TABLE.len() {
        new_int_val = divide_and_round_i64(int_val, LONG_TEN_POWERS_TABLE[ten_pow as usize]);
    } else {
        new_int_val = divide_and_round_bigint(int_val, big_ten_to_the(ten_pow));
    }
    new_int_val
}

fn divide_and_round_i64(int_val: BigInt, pow: i64) -> BigInt {
    todo!()
}

fn divide_and_round_bigint(dividend: BigInt, divisor: BigInt) -> BigInt {
    todo!()
}

// TODO: Can this BigInt be a ref instead?
// TODO: Should probably clean this up
fn big_ten_to_the(n: i32) -> BigInt {
    if n < 0 {
        return BigInt::zero();
    }

    { 
        let mut pows = BIG_TEN_POWERS_TABLE.lock().unwrap();
        let current_len = pows.len() as i32;

        if current_len <= n {
            let mut new_len = current_len << 1;
            while new_len <= n {
                new_len <<= 1;
            }

            let mut i = current_len;
            let big_ten = &BigInt::from_u8(10).expect("TODO");
            while i < new_len {
                let val = pows.get((i as usize) -1).expect("TODO") * big_ten;
                pows.insert(i as usize, val);
                i += 1;
            }
        }

        return pows[n as usize].clone()
    }
}

// TODO: NOT SURE IF THIS SHOULD GO IN A SEP MODULE FOR BIGINT
// TODO: CLEANUP - this func is a mess
fn compare_half(a: &BigInt, b: &BigInt) -> i32 {
    let mut a_val = a.to_u32_digits().1;
    a_val.reverse();
    let mut b_val = b.to_u32_digits().1;
    b_val.reverse();

    if a_val.len() <= 0 {
        if b_val.len() <= 0 {
            return 0;
        } else {
            return -1;
        }
    }

    if a_val.len() > b_val.len() {
        return 1;
    }

    if a_val.len() < b_val.len() -1 {
        return -1;
    }

    let mut b_start = 0;
    let mut carry = 0;

    if a_val.len() != b_val.len() {
        if b_val[b_start] == 1 {
            b_start += 1;
            // carry = 0x80000000;
            carry = -2147483648;
        } else {
            return -1;
        }
    }

    let val: Vec<u32> = a_val.clone(); // TODO: FIX THIS CLONE
    let mut i = 0; // TODO: Is this correct? this is "offset"
    let mut j = b_start;
    let LONG_MASK: i64 = 0xffffffff;
    while i < (a_val.len() + 0) {
        let bv = b_val[j];
        j += 1;
        let hb: i64 = ((bv as i64 >> 1) + carry) & LONG_MASK;
        let v = a_val[i] as i64 & LONG_MASK;
        i += 1;

        if v != hb {
            if v < hb {
                return -1;
            } else {
                return 1;
            }
        }

        carry = ((bv & 1) << 31) as i64;

    }
    if carry == 0 {
        return 0;
    } else {
        return -1;
    }
}


// TODO: Move this, also const fn?
// TODO: RENAME?
const LONG_TEN_POWERS_TABLE: [i64;19] = [
        1,                     // 0 / 10^0
        10,                    // 1 / 10^1
        100,                   // 2 / 10^2
        1000,                  // 3 / 10^3
        10000,                 // 4 / 10^4
        100000,                // 5 / 10^5
        1000000,               // 6 / 10^6
        10000000,              // 7 / 10^7
        100000000,             // 8 / 10^8
        1000000000,            // 9 / 10^9
        10000000000,          // 10 / 10^10
        100000000000,         // 11 / 10^11
        1000000000000,        // 12 / 10^12
        10000000000000,       // 13 / 10^13
        100000000000000,      // 14 / 10^14
        1000000000000000,     // 15 / 10^15
        10000000000000000,    // 16 / 10^16
        10000000000000000,   // 17 / 10^17
        1000000000000000000   // 18 / 10^180
];

// TODO: Add comment how using lazy and/or a mutex may not be optimal, but we don't have any other option?
static BIG_TEN_POWERS_TABLE: Lazy<Mutex<Vec<BigInt>>> = Lazy::new(|| {
     Mutex::new(vec![
        BigInt::one(),
        10.to_bigint().unwrap(),
        100.to_bigint().unwrap(),
        1000.to_bigint().unwrap(),
        10000.to_bigint().unwrap(),
        100000.to_bigint().unwrap(),
        1000000.to_bigint().unwrap(),
        10000000.to_bigint().unwrap(),
        100000000.to_bigint().unwrap(),
        1000000000.to_bigint().unwrap(),
        10000000000_i64.to_bigint().unwrap(),
        100000000000_i64.to_bigint().unwrap(),
        1000000000000_i64.to_bigint().unwrap(),
        10000000000000_i64.to_bigint().unwrap(),
        100000000000000_i64.to_bigint().unwrap(),
        1000000000000000_i64.to_bigint().unwrap(),
        10000000000000000_i64.to_bigint().unwrap(),
        100000000000000000_i64.to_bigint().unwrap(),
        1000000000000000000_i64.to_bigint().unwrap()
    ])
});

const INFLATED: i64 = i64::MIN;

#[cfg(test)]
mod tests {
    use crate::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn root_from_str_test() {
        let vals: Vec<(&str, isize, &str)> = vec![
            ("1.79", 1, "1.79"),
            ("1.73803", 4, "9.125"),
            ("1.562880129", 5, "9.3245600"),
            ("1.453573513976", 13, "129.32456087"),
            ("1.09280916443673520", 135, "159765.989751345"),
        ];

        vals.iter().for_each(|(expected_result, n, x)| {
            assert_eq!(
                Ok(BigDecimal::from_str(expected_result).unwrap()),
                root(*n, BigDecimal::from_str(x).unwrap())
            );
        });
    }
    #[test]
    fn temp_test() {
        let a = BigInt::from_str("100000000000000976996261670137755572795867919921875").unwrap();
        let b = BigInt::from_str("1000000000000000000000000000000000000000000000000000").unwrap();
        let half = compare_half(&a,&b);
        println!("half answer: {:?}", half);
        assert_eq!(1,2);
       // let foo = big_ten_to_the(20);
       //  let bigint_test = BigInt::from_str("1000000000000000000000000000000000000000000000000000").unwrap();
       //
       //  let bar = bigint_test.to_u32_digits();
       //  println!("bar: {:?}", bar);
       // assert_eq!(BigInt::from_str("1000000000000000000000000000000").unwrap(), foo);
    }
    // #[test]
    // fn round_from_str_test() {
    //     let vals: Vec<(&str, isize, &str)> = vec![
    //         ("1.79", 2, "1.794"),
    //         // ("1.73803", 4, "9.125"),
    //         // ("1.562880129", 5, "9.3245600"),
    //         // ("1.453573513976", 13, "129.32456087"),
    //         // ("1.09280916443673520", 135, "159765.989751345"),
    //     ];
    //
    //     vals.iter().for_each(|(expected_result, prec, x)| {
    //         assert_eq!(
    //             Ok(BigDecimal::from_str(expected_result).unwrap()),
    //             do_round(&BigDecimal::from_str(x).unwrap(), *prec)
    //         );
    //     });
    // }
}
