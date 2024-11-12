use std::collections::VecDeque;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits: [i32; 32] = [0; 32];
        let mut window: VecDeque<i32> = VecDeque::new();
        let mut i: usize = 0;
        let mut expand: bool = true;
        let mut min_count: i32 = i32::MAX;
        loop {
            if expand {
                if i >= nums.len() { break }
                let mut num = nums[i];
                window.push_back(nums[i]);
                let mut bit_pos = 0;
                while num != 0 {
                    let bit = num & 1;
                    num >>= 1;
                    bits[bit_pos] += bit;
                    bit_pos += 1;
                }

                let mut or: i32 = 0;
                for bit in bits.iter().rev() {
                    or <<= 1;
                    or |= if *bit > 0 { 1 } else { 0 }
                }

                if or >= k {
                    expand = false;
                    min_count = min_count.min(window.len() as i32);
                    if window.len() == 1 { break }
                }

                i += 1;
            } else {
                let mut num = window.pop_front().unwrap();
                
                let mut bit_pos = 0;
                while num != 0 {
                    let bit = num & 1;
                    num >>= 1;
                    bits[bit_pos] -= bit;
                    bit_pos += 1;
                }

                let mut or: i32 = 0;
                for bit in bits.iter().rev() {
                    or <<= 1;
                    or |= if *bit > 0 { 1 } else { 0 }
                }

                if or < k {
                    expand = true;
                } else {
                    min_count = min_count.min(window.len() as i32);
                    if window.len() == 1 { break }
                }
            }
        }
        if min_count != i32::MAX { min_count } else { -1 }
    }
}
