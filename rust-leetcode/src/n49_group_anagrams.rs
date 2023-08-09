pub struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
         
        vec![vec![String::from("ant")]]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_1() {
        let strs: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat", "ant"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(
            Solution::group_anagrams(strs),
            vec![
                vec![String::from("bar")],
                vec![String::from("nat"), String::from("tan")],
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ]
            ]
        );
    }
}
