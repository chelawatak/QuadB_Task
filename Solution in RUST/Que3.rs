// Given a string of words, implement a function that returns the shortest word in the string.


use std::cmp;

fn solver(s: &str) -> String {
    let mut shortest_word = String::new();
    let mut min_len = usize::MAX;

    for word in s.split_whitespace() {
        let len = word.chars().count();
        if len < min_len {
            shortest_word = word.to_string();
            min_len = len;
        }
    }

    shortest_word
}

fn main() {
    let s = "  Aniket       Chelawat is      solving the assignment   given     by     QuadB company  ";
    let shortest_word = solver(s);
    println!("Shortest word present in this string = {}", shortest_word);
}
