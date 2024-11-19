impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);
        
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }

        let mut count = right;
        while left <= right && (left == 0 || arr[left - 1] <= arr[left]) {
            while right < arr.len() && arr[left] > arr[right] {
                right += 1
            }
            count = count.min(right - left - 1);
            left += 1
        }
        
        return count as i32
    }
}
