pub struct Solution;
impl Solution {
    pub fn divide(divident: i32, divisor: i32) -> i32{
        
        let mut ans = 0i32;
        let mut divident = divident;

        let divident = {
            if divident>0{-divident } else { divident }
        };
        let divisor = {
            if divisor>0 {-divisor } else { divisor }
        };
        let power_of_divisor = Self::power_of_two_mul_list(divisor, divident);
        let times = power_of_divisor.len();
        

        let power_of_two = Self::power_of_two(times as i32);

        for i in (0..power_of_divisor.len()).rev(){
            let val = power_of_divisor[i];
            if divident>= val{
                divident -= val;
                ans += power_of_two[i];
            }
        }
        if pos {
            ans
        }else{
            -ans 
        }

    }
    fn power_of_two_mul_list(n: i32  ,  min: i32) -> Vec<i32> {
        let mut list = Vec::<i32>::new();
        let mut prev = n;
        loop {
            list.push(prev);
            if prev < i32::MIN - prev{break}
            prev = prev+prev; 
            if prev < min {break}
        }
        list

    }
    fn power_of_two( times: i32) -> Vec<i32> {
        let mut list = Vec::<i32>::new();
        let mut prev = -1i32;
        let mut times = times;
        loop {
            times -= 1;
            list.push(prev);
            prev = prev+prev; 
            if times<=0 {break}
        }
        list
    }
}
