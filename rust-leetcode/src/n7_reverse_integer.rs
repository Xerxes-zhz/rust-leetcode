pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x >= 0 {
            Solution::reverse_positive(x)
        } else {
            -Solution::reverse_positive(-x)
        }
    }
    fn reverse_positive(x: i32) -> i32 {
        match x
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
        {
            Ok(ans) => ans,
            Err(_) => 0i32,
        }
    }
}

