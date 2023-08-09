

pub struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let shortest: Option<&String> = strs.iter().min_by_key(|s| s.len());
        let mut min = 0;
        match shortest{
            Some(s) => min =  s.len(),
            None => println!("The vector is empty!")
        }
        if min==0{
            return "".to_string();
        }
        let mut res: i32 = min as i32- 1 ;
        for i in 0 .. min{
            let mut keys = strs.iter().map(|s| s.chars().nth(i));
            let first = keys.next().flatten();
            if keys.all(|c|c==first){
                continue;
            }else {
                res = i as i32 - 1;
                break;
            }
        }
        if res==-1{
            return "".to_string();
        }
        
        match strs.first(){
            Some(s) => return s[0..=res as usize].to_string(),
            None => return "".to_string() ,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".to_string(), "cat".to_string()]),
            "".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flow".to_string(),
                "fly".to_string(),
                "flent".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "a".to_string(),
            ]),
            "a".to_string()
        );
    }
}
