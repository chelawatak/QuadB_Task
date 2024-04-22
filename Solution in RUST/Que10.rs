// Check if a number is prime in Rust


fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    let sqrt_num = (number as f64).sqrt() as u32;
    for i in 2..=sqrt_num {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 29;

    if is_prime(number) {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}
