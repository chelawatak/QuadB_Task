// Implement a function that finds the longest common prefix of a given set of strings.


fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new(); // If the vector is empty, return an empty string
    }

    for (i, &ch) in strs[0].as_bytes().iter().enumerate() {
        for other_str in &strs[1..] {
            if i >= other_str.len() || other_str.as_bytes()[i] != ch {
                return strs[0][..i].to_string();
            }
        }
    }
    // If no mismatch is found, return the first string
    strs[0].clone()
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
