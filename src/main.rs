// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?
use std::f32::consts::PI;

fn count_to_5(mut number: i32) -> i32 {
    println!("Initial Number {}", number);

    loop {
        if number > PI as i32 && number >= 5 {
            break;
        }

        number += 1;
        println!(
            "Now equals to {}, need more {} to reach 5",
            number,
            5 - number
        );
    }

    number
}

fn main() {
    println!("I can count to {}", count_to_5(1));
    println!("I can count to {}", count_to_5(4));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5(1) == 5, true);
    }
}
