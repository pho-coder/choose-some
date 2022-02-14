pub struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut new_vec: Vec<i32> = vec![];
        new_vec.push(nums[0] * nums[0]);
        for (index, &value) in nums[1..].iter().enumerate() {
            let v_square = value * value;
            for (index1, &value1) in new_vec.iter().enumerate() {
                if v_square <= value1 {
                    let new_vec1 = &new_vec[..index1];
                    let new_vec2 = &new_vec[index1..];
                    // let new_vec = new_vec1.push(v_square);
                }
            }
        }
        return new_vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            [0, 1, 9, 16, 100]
        );
    }
    #[test]
    fn test_search_insert_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            [4, 9, 9, 49, 121]
        );
    }
}
