use std::fs;
use std::io::Error;

fn main() -> Result<(), Box<Error>> {
    let input = fs::read_to_string("./01/input.txt")?;
    let solution = solve_part_one(&input);
    println!("{}", &solution);
    Ok(())
}

fn get_digits (input: &str) -> (u8, u8) {
    let digits: Vec<_> = input.matches(char::is_numeric).map(|x| x.parse::<u8>().unwrap()).collect();
    (digits[0], digits[digits.len() - 1])
}

fn solve_part_one(input: &str) -> u32 {
    input.lines()
        .map(get_digits)
        .fold(0, |acc, v| acc + u32::from(v.0) * 10 + u32::from(v.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_example() {
        let example = fs::read_to_string("./01/example.txt").unwrap();
        assert_eq!(solve_part_one(&example), 147);
    }
}
