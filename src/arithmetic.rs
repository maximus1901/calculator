pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Power,
}

impl Operator {
    pub fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operator::Addition => Ok(a + b),
            Operator::Subtraction => Ok(a - b),
            Operator::Multiplication => Ok(a * b),
            Operator::Division => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
            Operator::Modulus => Ok(a % b),
            Operator::Power => Ok(a.powf(b)),
        }
    }
}
