// Create functions for fizz, buzz and fizzbuzz that return bools for whether the number is divisible by 3, 5 or both respectively.
fn fizz(i: i32) -> bool {
    if i % 3 == 0 {
        // Return true if i is divisible by 3
        return true;
    } else {
        return false;
    }
}

fn buzz(i: i32) -> bool {
    if i % 5 == 0 {
        // Return true if i is divisible by 5
        return true;
    } else {
        return false;
    }
}

fn fizzbuzz(i: i32) -> bool {
    if i % 5 == 0 && i % 3 == 0 {
        // Return true if i is divisible by 3 and 5
        return true;
    } else {
        return false;
    }
}

// Main function for execution
fn main() {
    // For loop to iterate through numbers from 1 to 100
    for i in 1..=100 {
        // If i is divisible by 3 and 5, print fizzbuzz
        if fizzbuzz(i) {
            println!("fizzbuzz!")
        // If i is divisible by 3, print fizz
        } else if fizz(i) {
            println!("fizz")
        // If i is divisible by 5, print buzz
        } else if buzz(i) {
            println!("buzz")
        // If i is not divisible by 3 or 5, print i
        } else {
            println!("{}", i)
        }
    }
}
