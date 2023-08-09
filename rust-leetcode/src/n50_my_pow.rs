pub struct Solution {}
use std::i32;

impl Solution {

    pub fn my_pow(x: f64, n: i32) -> f64 {
        return x.powi(n);
    } 
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::max_power(2 as f64, 2 as i32), 25 as f64);
    }
}