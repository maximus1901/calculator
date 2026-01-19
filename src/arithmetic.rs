pub fn addition(a: f64, b: f64) -> f64 {
    a + b
}
pub fn substraction(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn division(a: f64, b: f64) -> f64 {
    if a == f64::from(0)|| b == f64::from(0) {
        return f64::from(0);
    }
    f64::from(a) / f64::from(b)
}

pub fn modulo(a: f64, b: f64) -> f64 { a % b }
pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
