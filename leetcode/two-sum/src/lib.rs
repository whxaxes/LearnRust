// https://leetcode.cn/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        let c = target - nums[i];
        for j in 0..i {
            if nums[j] == c {
                return vec![ j as i32, i as i32 ];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![ 3, 2, 4 ], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![ 3, 3 ], 6), vec![0, 1]);
    }
}
