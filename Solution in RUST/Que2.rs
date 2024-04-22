// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn index_finder(arr: &[i32], num: i32) -> Option<usize> {
    arr.iter().position(|&x| x == num)
}

fn main() {
    let array = vec![1, 2, 2, 3, 4, 5];
    let find_occurrence_of = 2;

    if let Some(index) = index_finder(&array, find_occurrence_of) {
        // let index starting from 1.
        println!("The first occurrence of {} is at index: {}", find_occurrence_of, index + 1);
    } else {
        println!("The number {} is not present in the array.", find_occurrence_of);
    }
}
