use std::io::{self, BufRead};

// Specifically, to find the fuel required for a module, take its mass, divide
// by three, round down, and subtract 2.
pub fn mass(w: i64) -> i64 {
    (w / 3) - 2
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
            Ok(Ok(num)) => sum += mass(num),
            _ => eprintln!("Something went wrong: {:?}", line),
        }
    }

    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, mass(14));
        assert_eq!(654, mass(1969));
        assert_eq!(33583, mass(100756));
    }
}
