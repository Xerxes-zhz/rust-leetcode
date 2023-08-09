pub struct Solution {}
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut start = 0;
        let mut result: i32 = 1;
        while start < s.len() {
            let mut len = 1;
            while start + len < s.len() {
                if s.chars().nth(start) == s.chars().nth(start + len) {
                    result = *[result, (len + 1) as i32].iter().max().unwrap_or(&0);
                    len += 1;
                } else {
                    start = start + len;
                    break;
                }
            }
            if start+len>=s.len(){
                start+=1;
            }
            
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::max_power("eeeee".to_string()), 5);

        assert_eq!(Solution::max_power("aaaeeeeecc".to_string()), 5);
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    }
}
