fn fizz(i: i32) -> bool {
    if i % 3 == 0 {
        return true;
    } else {
        return false;
    }
}

fn buzz(i: i32) -> bool {
    if i % 5 == 0 {
        return true;
    } else {
        return false;
    }
}

fn fizzbuzz(i: i32) -> bool {
    if i % 5 == 0 && i % 3 == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    for i in 1..101 {
        if fizzbuzz(i) {
            println!("fizzbuzz!")
        } else if fizz(i) {
            println!("fizz")
        } else if buzz(i) {
            println!("buzz")
        } else {
            println!("{}", i)
        }
    }
}
