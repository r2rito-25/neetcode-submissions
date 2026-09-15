impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut check = HashSet::new();
        for x in nums{
            if check.contains(&x){
                return true;
            }
            else{
                check.insert(x);
            }
        }
        return false
    }
}
