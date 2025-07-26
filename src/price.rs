use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Price {
    cents: i64,
}

impl Price {
    pub fn from_cents(cents: i64) -> Self {
        Self { cents }
    }

    pub fn from_dollars(dollars: f64) -> Self {
        // eg Price::from_dollars(12.34) -> 1234
        Self { cents: (dollars * 100.0).round() as i64 }
    }

    pub fn as_cents(self) -> i64 {
        self.cents
    }

    pub fn as_dollars_float(self) -> f64 {
        self.cents as f64 / 100.0
    }

    pub fn dollars_part(self) -> i64 {
        self.cents / 100
    }

    pub fn cents_part(self) -> u8 {
        (self.cents.abs() % 100) as u8
    }

    /// Returns eg "12.00" or "12.34"
    pub fn to_decimal_string(self) -> String {
        format!("{}.{}", self.dollars_part(), format!("{:02}", self.cents_part()))
    }

    /// Returns eg "$12.34" (US format)
    pub fn to_usd_string(self) -> String {
        format!("${}", self.to_decimal_string())
    }

    /// For logging eg "1234 cents ($12.34)"
    pub fn debug_string(self) -> String {
        format!("{} cents (${:.2})", self.cents, self.as_dollars_float())
    }
}

// Arithmetic with another Price
impl Add<Price> for Price {
    type Output = Price;
    fn add(self, rhs: Price) -> Self::Output {
        Price::from_cents(self.cents + rhs.cents)
    }
}
impl Sub<Price> for Price {
    type Output = Price;
    fn sub(self, rhs: Price) -> Self::Output {
        Price::from_cents(self.cents - rhs.cents)
    }
}

// Multiplying a Price by int/f64 (returns rounded)
impl Mul<i64> for Price {
    type Output = Price;
    fn mul(self, rhs: i64) -> Self::Output {
        Price::from_cents(self.cents * rhs)
    }
}
impl Mul<f64> for Price {
    type Output = Price;
    fn mul(self, rhs: f64) -> Self::Output {
        Price::from_cents((self.as_dollars_float() * rhs * 100.0).round() as i64)
    }
}
impl Mul<Price> for i64 {
    type Output = Price;
    fn mul(self, rhs: Price) -> Self::Output {
        rhs * self
    }
}

// Division by int/f64
impl Div<i64> for Price {
    type Output = Price;
    fn div(self, rhs: i64) -> Self::Output {
        Price::from_cents(self.cents / rhs)
    }
}
impl Div<f64> for Price {
    type Output = Price;
    fn div(self, rhs: f64) -> Self::Output {
        Price::from_dollars(self.as_dollars_float() / rhs)
    }
}


impl Price {
    pub fn abs(self) -> Self {
        Price::from_cents(self.cents.abs())
    }
}

impl fmt::Display for Price {
    /// prints eg "12.34"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // minus for negatives
        if self.cents < 0 {
            write!(f, "-{}", self.abs().to_decimal_string())
        } else {
            write!(f, "{}", self.to_decimal_string())
        }
    }
}


// Construction from cents, dollars, primitive types
impl From<i64> for Price {
    fn from(cents: i64) -> Price {
        Price::from_cents(cents)
    }
}
impl From<u32> for Price {
    fn from(cents: u32) -> Price {
        Price::from_cents(cents as i64)
    }
}
impl From<f64> for Price {
    fn from(dollars: f64) -> Price {
        Price::from_dollars(dollars)
    }
}



#[cfg(test)]
mod tests {
    use super::Price;

    #[test]
    fn test_price_from_cents() {
        let p = Price::from_cents(1234);
        assert_eq!(p.as_cents(), 1234);
        assert_eq!(p.dollars_part(), 12);
        assert_eq!(p.cents_part(), 34);
    }

    #[test]
    fn test_price_from_dollars() {
        let p = Price::from_dollars(12.34);
        assert_eq!(p.as_cents(), 1234);
        assert_eq!(p.to_decimal_string(), "12.34");
        assert_eq!(p.to_usd_string(), "$12.34");
    }

    #[test]
    fn test_price_rounding() {
        let p = Price::from_dollars(12.999);
        assert_eq!(p.as_cents(), 1300); // .999 rounds up to 13.00
    }

    #[test]
    fn test_price_display_and_log_string() {
        let p = Price::from_cents(9900);
        assert_eq!(p.to_usd_string(), "$99.00");
        assert_eq!(format!("{}", p), "$99.00");
        assert_eq!(p.debug_string(), "9900 cents ($99.00)");
    }

    #[test]
    fn test_price_add_sub() {
        let a = Price::from_cents(500);
        let b = Price::from_cents(125);
        let c = a + b;
        assert_eq!(c.as_cents(), 625);
        let d = c - Price::from_cents(25);
        assert_eq!(d.as_cents(), 600);
    }

    #[test]
    fn test_price_mul_int() {
        let a = Price::from_cents(123);
        assert_eq!((a * 3).as_cents(), 369);
        assert_eq!((3 * a).as_cents(), 369);
    }

    #[test]
    fn test_price_mul_f64() {
        let p = Price::from_cents(2000);
        let discounted = p * 0.5;
        assert_eq!(discounted.as_cents(), 1000);
        assert_eq!(discounted.to_usd_string(), "$10.00");
    }

    #[test]
    fn test_price_div_int() {
        let p = Price::from_cents(600);
        let per_item = p / 3;
        assert_eq!(per_item.as_cents(), 200);
    }

    #[test]
    fn test_price_div_f64() {
        let p = Price::from_cents(1200); // $12.00
        let result = p / 4.0;
        assert_eq!(result.as_cents(), 300); // $3.00
    }

    #[test]
    fn test_price_negative_prices() {
        let p = Price::from_cents(-1234);
        assert_eq!(format!("{}", p), "-$12.34");
        assert_eq!(p.debug_string(), "-1234 cents ($-12.34)");
        let abs = p.abs();
        assert_eq!(abs.as_cents(), 1234);
    }

    #[test]
    fn test_price_edge_cent_values() {
        let p = Price::from_cents(250);
        assert_eq!(p.dollars_part(), 2);
        assert_eq!(p.cents_part(), 50);

        let zero = Price::from_cents(0);
        assert_eq!(zero.dollars_part(), 0);
        assert_eq!(zero.cents_part(), 0);

        // Negative edge case: -9906c = -99.06
        let n = Price::from_cents(-9906);
        assert_eq!(n.dollars_part(), -99);
        assert_eq!(n.cents_part(), 6);
        assert_eq!(n.to_decimal_string(), "-99.06");
        assert_eq!(format!("{}", n), "-$99.06");
    }

    #[test]
    fn test_price_large() {
        let p = Price::from_cents(1_000_000_000); // $10,000,000.00
        assert_eq!(p.dollars_part(), 10_000_000);
        assert_eq!(p.cents_part(), 0);
        assert_eq!(p.to_usd_string(), "$10000000.00");
    }
}