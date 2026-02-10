pub enum Operator {
    addition,
    subtraction,
    multiplication,
    division,
    modulus,
    power,
}

impl Operator {
    pub fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operator::addition => Ok(a + b),
            Operator::subtraction => Ok(a - b),
            Operator::multiplication => Ok(a * b),
            Operator::division => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
            Operator::modulus => Ok(a % b),
            Operator::power => Ok(a.powf(b)),
        }
    }
}
