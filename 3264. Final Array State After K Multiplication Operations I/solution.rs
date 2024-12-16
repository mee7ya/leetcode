use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = BinaryHeap::from_iter(
            nums.iter().enumerate().map(|(idx, &x)| (Reverse(x), Reverse(idx)))
        );
        for i in (0..k) {
            let (val, idx): (Reverse<i32>, Reverse<usize>) = heap.pop().unwrap();
            heap.push((Reverse(val.0 * multiplier), idx));
        }
        while let Some((val, idx)) = heap.pop() {
            nums[idx.0] = val.0;
        }
        nums
    }
}
