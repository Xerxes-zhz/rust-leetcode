pub struct Solution;
struct CS {
    count: i32,
    num: i32
}

impl Solution{
    pub fn count_and_say(n: i32)->String{
        match n {
            1=>"1".to_string(),
            2=>"11".to_string(),
            3=>"21".to_string(),
            4=>"1211".to_string(),
            5=>"111221".to_string(),
            6=>"312211".to_string(),
            7=>"13112221".to_string(),
            8=>"1113213211".to_string(),
            9=>"31131211131221".to_string(),
            10=>"13211311123113112211".to_string(),
            _=>{
                let s = Self::count_and_say(n-1);
                let mut last= CS{count:0, num:0};
                let mut cs_list= Vec::<CS>::new();
                for str in s.chars(){
                    let num = str.to_digit(10).unwrap() as i32;
                    if last.num ==0{
                        last.num = num;
                    }
                    if last.num == num{
                        last.count += 1;
                    }else{
                        cs_list.push(last);
                        last = CS{count:1, num};
                    }
                }
                cs_list.push(last);
                let mut ans = String::new();
                for cs in cs_list.iter(){
                    ans.push_str(&cs.count.to_string());
                    ans.push_str(&cs.num.to_string());
                }
                ans
            }

        }
    }
}
