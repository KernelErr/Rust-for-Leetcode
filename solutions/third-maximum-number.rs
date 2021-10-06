use std::collections::{BinaryHeap, HashSet};
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let set: HashSet<_> = nums.drain(..).collect();
        nums.extend(set.into_iter());
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();
        for num in nums {
            heap.push(-(num as i64));
            while heap.len() > 3 {
                heap.pop();
            }
        }
        while let Some(ret) = heap.peek() {
            if heap.len() == 1 || heap.len() == 3 {
                return -(*ret) as i32;
            }
            heap.pop();
        }
        return 0;
    }
}