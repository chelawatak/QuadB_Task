// Find the maximum subarray sum in Rust
fn main() {
    let array = vec![1, 4, 5, 3, -8, 1, 5, 7, -1, -2, 8];

    let mut max_subarray_sum = i32::MIN;
    let mut max_sum = 0;
    for &num in &array {
        max_sum += num;
        if max_sum > max_subarray_sum {
            max_subarray_sum = max_sum;
        }

        if max_sum <= 0 {
            max_sum = 0;
        }
    }

    println!("Answer = {}", max_subarray_sum);
}
