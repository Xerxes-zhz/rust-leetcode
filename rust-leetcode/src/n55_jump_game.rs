use std::cmp::min;

pub struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jump_list: Vec<bool> = vec![false; nums.len()];
        jump_list[0] = true;
        let len: usize = nums.len();
        for (i,val) in nums.iter().enumerate(){
            if jump_list[i]{
                let max: usize = min(*val as usize, len-i-1);
                for step  in (1usize..=max).rev(){
                    if jump_list[i+step]{break;}
                    jump_list[i+step] = true;
                }
            }
        }
        jump_list[len - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
    }
}
