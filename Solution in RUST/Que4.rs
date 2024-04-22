// Implement a function that checks whether a given number is prime or not.


fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn main() {
    let n = 11;
    if is_prime(n) {
        println!("Prime Number");
    } else {
        println!("Non Prime Number");
    }
}
