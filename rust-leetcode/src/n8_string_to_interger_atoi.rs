use std::string::ParseError;

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let mut positive = true;
        let mut num = String::new();
        let digits = "0123456789";
        for (i, c) in s.chars().enumerate() {
            match c {
                '+' if i == 0 => {}
                '-' if i == 0 => positive = false,
                _ if digits.contains(c) => num.push(c),
                _ => break,
            }
        }
        if num.len() > 0 {
            if positive {
                match num.parse::<i32>() {
                    Ok(ans) => ans,
                    Err(_) => i32::MAX,
                }
            } else {
                match num.parse::<i32>() {
                    Ok(ans) => -ans,
                    Err(_) => i32::MIN,
                }
            }
        } else {
            0i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42i32);
        assert_eq!(Solution::my_atoi("-42".to_string()), -42i32);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193i32);
        assert_eq!(
            Solution::my_atoi("419311111111111111 with words".to_string()),
            4193i32
        );
    }
}
