use std::{collections::HashMap, hash::{Hash,Hasher}};

pub struct Solution;
#[derive(Debug, Clone)]
struct LetterVec {
    length: usize,
    letter_count: HashMap<char, u32>,
    str: String,
}
impl LetterVec {
    fn new(str: String) -> Self {
        let mut letter_count: HashMap<char, u32> = HashMap::new();
        for char in str.chars() {
            let counter = letter_count.entry(char).or_insert(0);
            *counter += 1;
        }

        LetterVec {
            length: str.len(),
            letter_count,
            str,
        }
    }
}
impl PartialEq for LetterVec{
    fn eq(&self, other: &Self)->bool{
        if self.length!=other.length {
            false
        }else{
            self.letter_count==other.letter_count
        }
    }
}
impl Eq for LetterVec{}
impl Hash for LetterVec{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.length.hash(state);
        for (key ,value) in &self.letter_count{
            key.hash(state);
            value.hash(state);
        }
    }

}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let letter_vec:Vec<LetterVec> = strs.into_iter().map(|str|LetterVec::new(str)).collect();
        let mut ans_map: HashMap<LetterVec, Vec<String>> = HashMap::new();
        for lv in letter_vec.iter(){
            let setter = ans_map.entry(lv.clone()).or_insert(vec![]);
            setter.push(lv.str.clone());
        }
        println!("{:?}", ans_map);
        ans_map.into_values().collect()

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
