use bigdecimal::BigDecimal;

pub type BigDecimalMathResult = Result<BigDecimal, BigDecimalMathError>;

#[derive(Debug, PartialEq)]
pub enum BigDecimalMathError {
    // A math error, i.e. divide by 0
    ArithmeticError(String)
}
