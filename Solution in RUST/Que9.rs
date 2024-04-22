// Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars().rev() {
        result.push(ch);
    }
    result
}

fn main() {
    let s = "Aniket";
    let ans = reverse_string(s);
    println!("Reverse = {}", ans);
}
