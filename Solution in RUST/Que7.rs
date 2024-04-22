// Implement a function that returns the kth smallest element in a given array.


use std::collections::HashMap;

fn finder(v: Vec<i32>, k: usize) -> i32 {
    let mut mp: HashMap<i32, usize> = HashMap::new();
    for &num in &v {
        *mp.entry(num).or_insert(0) += 1;
    }

    let mut current_k = 0;
    for i in 0..=10000 {
        if let Some(&count) = mp.get(&(i as i32)) {
            current_k += count;
            if current_k >= k {
                return i as i32;
            }
        }
    }

    panic!("Invalid input: k exceeds the number of unique elements");
}

fn main() {
    let array = vec![7, 5, 3, 1, 1, 2, 6, 8, 11, 2, 6, 7];
    let k = 12; // find the second smallest element

    let ans = finder(array, k);
    println!("{}th smallest element in the given array is: {}", k, ans);
}
