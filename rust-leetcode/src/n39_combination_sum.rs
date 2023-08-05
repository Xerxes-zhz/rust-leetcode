pub struct Solution;
impl Solution{
    pub fn combination_sum(candidates:Vec<i32>, target:i32)->Vec<Vec<i32>>{
        let mut candidates = candidates; 
        candidates.sort_by(|a, b| a.cmp(b));
        Self::combination_sum_of_sorted(candidates,target)
    }
    fn combination_sum_of_sorted(candidates:Vec<i32>, target:i32)-> Vec<Vec<i32>>{
        let first_num = candidates[0];
        let mut ans = Vec::<Vec<i32>>::new();
        if candidates.len()==0 || first_num>target{
            return ans;
        }
        let mut i = 0i32;
        while first_num*i<=target{
            let first = vec![first_num; i as usize];
            let rest_target  = target-first_num*i;
            if rest_target==0{
                ans.push(first);
            }else if candidates.len()>1{
                let rest = Self::combination_sum(candidates[1..].to_vec(), rest_target);
                    if rest.len()!=0{
                        for item in rest.iter(){
                            let mut temp = first.clone();
                            temp.extend_from_slice(item);
                            ans.push(temp);
                        }
                    }
            }
            i+=1; 
        }
        ans

    }

}
#[cfg(test)]
mod test{
    use super::Solution;
    #[test]
    pub fn test_1(){
        assert_eq!(Solution::combination_sum(vec![8i32,7i32,4i32,3i32], 11i32), vec![vec![4i32,7i32], vec![3i32,8i32], vec![3i32, 4i32, 4i32]]);
    }
}
