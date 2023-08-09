pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        let mut num_vec: Vec<i32> = nums.clone();
        num_vec.sort();
        while left < right {
            let sum = num_vec[left] + num_vec[right];
            if sum == target {
                break;
            } else if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            }
        }
        let mut i: i32 = -1;
        let mut j: i32 = -1;
        for (id, num) in nums.iter().enumerate() {
            if *num == num_vec[left] && i == -1 {
                i = id as i32;
            } else if *num == num_vec[right] && j == -1 {
                j = id as i32;
            }
        }
        return vec![i, j];
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
