pub struct Solution {}

/*
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
*/
use std::collections::HashMap;
impl Solution {
    pub fn insert_n_minus(map: &mut HashMap<String, i32>, minus: &str, half: &str, full: &str) {
        map.insert(
            format!("{}{}", minus, half),
            map.get(&half.to_string()).unwrap() - map.get(&minus.to_string()).unwrap(),
        );
        map.insert(
            format!("{}{}", minus, full),
            map.get(&full.to_string()).unwrap() - map.get(&minus.to_string()).unwrap(),
        );
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("I".to_string(), 1 as i32);
        map.insert("V".to_string(), 5 as i32);
        map.insert("X".to_string(), 10 as i32);
        map.insert("L".to_string(), 50 as i32);
        map.insert("C".to_string(), 100 as i32);
        map.insert("D".to_string(), 500 as i32);
        map.insert("M".to_string(), 1000 as i32);
        for s in "IXCM".chars() {
            map.insert(
                format!("{}{}", s, s),
                map.get(&s.to_string()).unwrap() * 2 as i32,
            );
            map.insert(
                format!("{}{}{}", s, s, s),
                map.get(&s.to_string()).unwrap() * 3 as i32,
            );
        }

        Solution::insert_n_minus(&mut map, "I", "V", "X");
        Solution::insert_n_minus(&mut map, "X", "L", "C");
        Solution::insert_n_minus(&mut map, "C", "D", "M");
        println!("{:?}", map);
        let mut result = 0;
        let mut left = 0;
        while left < s.len() {
            let mut right = std::cmp::min(2, s.len() - left - 1);
            loop {
                if map.get(&s[left..=left + right]).is_some() {
                    result += map[&s[left..=left + right]];
                    println!("{} {} {}", left, right, map[&s[left..=left + right]]);
                    left = left + right + 1;
                    break;
                }

                if right == 0 {
                    break;
                } else {
                    right -= 1;
                }
            }
        }

        return result;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
