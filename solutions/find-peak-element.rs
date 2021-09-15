impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut l: usize = 0;
        let mut r: usize = n - 1;

        if n == 1 {
            return 0;
        }

        fn check_ans(nums: &Vec<i32>, pos: usize) -> i32 {
            if pos == 0 {
                if nums[pos] > nums[pos + 1] {
                    0
                } else {
                    1
                }
            } else if pos == nums.len() - 1 {
                if nums[pos] > nums[pos - 1] {
                    0
                } else {
                    -1
                }
            } else {
                if nums[pos] > nums[pos - 1] && nums[pos] > nums[pos + 1] {
                    0
                } else if nums[pos] < nums[pos + 1] {
                    1
                } else {
                    -1
                }
            }
        }

        while l + 1 < r {
            let mid: usize = (r - l) / 2 + l;
            match check_ans(&nums, mid) {
                0 => return mid as i32,
                1 => {
                    l = mid;
                },
                _ => {
                    r = mid;
                }
            }
        }

        if check_ans(&nums, l) == 0 {
            return l as i32;
        }

        if check_ans(&nums, r) == 0 {
            return r as i32;
        }

        0
    }
}