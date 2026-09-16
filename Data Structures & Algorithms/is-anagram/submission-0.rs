impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut check = HashMap::new();
        for c in s.chars(){
            let mut count = check.get(&c).copied().unwrap_or(0);
            count = count + 1;
            check.insert(c, count);
        }
        for c in t.chars(){
            let mut count = check.get(&c).copied().unwrap_or(0);
            if count <= 0{ // if no chars left from that type or none where there to begin with -> it can't be an anagram
                return false;
            }
            // otherwise decrease the count for that particular char or remove it
            count = count - 1;
            if count == 0 {
                check.remove(&c);
            }
            else{
                check.insert(c, count);
            }
        }
        if check.is_empty(){
            return true;
        }
        else{
            return false;
        }
    }
}
