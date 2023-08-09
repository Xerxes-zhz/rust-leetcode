use std::cmp::min;

pub struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return  0i32;
        }
        if nums[0] >= (n - 1) as i32 {
            return 1i32;
        }

        let mut min_jump_step = vec![-1; n];
        min_jump_step[0] = 0i32;

        let mut far_move = 0;
        for (i, &val) in nums.iter().enumerate() {
            if i > far_move {
                panic!("cant reach");
            } else {
                let val = val as usize;
                if i + val > far_move {
                    let step = min_jump_step[i] + 1;
                    let max_i = min(n-1, i+val);
                    if max_i >=n-1 {
                        min_jump_step[n - 1] = step;
                        break;
                    }
                    for x in min_jump_step[far_move+1..=max_i].iter_mut() {
                        *x = step;
                    }
                    far_move = max_i;
                } else {
                    continue;
                }
            }
        }
        min_jump_step[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {}
}
