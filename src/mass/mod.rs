use std::io::{self, BufRead};

// Specifically, to find the fuel required for a module, take its mass, divide
// by three, round down, and subtract 2.
pub fn fuel(w: i64) -> i64 {
    (w / 3) - 2
}

// Fuel itself requires fuel just like a module - take its mass, divide by
// three, round down, and subtract 2.
pub fn with_fuel(w: i64) -> i64 {
    let mut total = 0;
    let mut extra = fuel(w);

    while extra > 0 {
        total += extra;
        extra = fuel(extra);
    }

    total
}

// The Fuel Counter-Upper needs to know the total fuel requirement. To find it,
// individually calculate the fuel needed for the mass of each module (your
// puzzle input), then add together all the fuel values.
//
// What is the sum of the fuel requirements for all of the modules on your
// spacecraft?
pub fn main() {
    let mut sum = 0;
    let stdin = io::stdin();

    for ref line in stdin.lock().lines() {
        match line.as_ref().map(|l| l.parse::<i64>()) {
            Ok(Ok(num)) => sum += with_fuel(num),
            _ => eprintln!("Something went wrong: {:?}", line),
        }
    }

    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(2, fuel(14));
        assert_eq!(654, fuel(1969));
        assert_eq!(33583, fuel(100756));
    }

    #[test]
    fn recursive() {
        assert_eq!(2, with_fuel(14));
        // 654 + 216 + 70 + 21 + 5 = 966
        assert_eq!(966, with_fuel(1969));
        assert_eq!(50346, with_fuel(100756));
    }
}
