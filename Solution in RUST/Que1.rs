// Implement a function that checks whether a given string is a palindrome or not.


fn check_palindrome(s: &str) -> bool {
    s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
}

fn main() {
    let s1 = "nayan";

    let output1 = check_palindrome(s1);

    if output1 {
        println!("{} is a palindrome", s1);
    } else {
        println!("{} is not a palindrome", s1);
    }
}