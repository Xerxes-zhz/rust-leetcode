pub struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut mo = 1;
        let mut  result = digits.clone();
        for digit in result.iter_mut().rev(){
            (mo, *digit) = {
                let sum: i32 = mo + *digit;
                (sum/10,sum%10)
            }
        }
        if mo==1{
            result.insert(0, 1);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
    }
}