//! Big Decimal Math
//!
//! A collection of mathematical functions [originally implemented in Java by Richard J. Mathar](https://arxiv.org/abs/0908.3030v3) for [bigdecimal].

use bigdecimal::{BigDecimal, FromPrimitive, One, ToPrimitive, Zero};
mod error;
use error::{BigDecimalMathError, BigDecimalMathResult};

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
}
