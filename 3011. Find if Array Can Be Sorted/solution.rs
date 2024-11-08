impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        for i in (0..nums.len() - 1) {
            if nums[i+1] < nums[i] {
                if nums[i+1].count_ones() != nums[i].count_ones() { return false }
                nums.swap(i, i+1);
            }
        }

        for i in (1..nums.len()).rev() {
            if nums[i-1] > nums[i] {
                if nums[i-1].count_ones() != nums[i].count_ones() { return false }
                nums.swap(i, i-1);
            }
        }

        true
    }
}
