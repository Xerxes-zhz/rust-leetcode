pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in 0..nums.len(){
            if is_sorted_reverse(&nums[i..]){
                
                return nums[..i]+min()+nums[i+1..]
            }
        }
    }
    fn is_sorted_reverse(nums: &[i32]) -> bool{
        let mut is_sorted = true;
        let mut prev = nums[0];
        if nums.len() == 1{
            return false
        }
        for &num in nums.iter().skip(1) {
            if num<=prev {
                is_sorted= false;
                break;
            }
            prev = num;
        }
        is_sorted
    }
}
