// Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(nums: &[i32]) -> f64 {
    let n = nums.len();
    // If the number of elements is even
    if n % 2 == 0 {
        let mid_right = n / 2;
        let mid_left = mid_right - 1;
        return (nums[mid_left] as f64 + nums[mid_right] as f64) / 2.0;
    } else {
        let mid = n / 2;
        return nums[mid] as f64;
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6];
    println!("Median: {}", find_median(&sorted_array)); // Output: 3 (since 3 is in the middle)
}
