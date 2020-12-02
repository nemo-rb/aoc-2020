use aoc::*;

fn main() -> Result<()> {
    let result = run();
    println!("Day 2 part 2 result is {}", result);
    Ok(())
}

fn run() -> usize {
    input("2.txt")
        .unwrap()
        .lines()
        .map(|s| parse_line(s))
        .filter(|r| valid_password(*r))
        .count()
}

fn parse_line(p: &str) -> (usize, usize, char, &str) {
    let x: Vec<&str> = p.split(|c| c == '-' || c == ' ' || c == ':').collect();
    (
        x[0].parse().unwrap(),
        x[1].parse().unwrap(),
        x[2].chars().next().unwrap(),
        x[4],
    )
}

fn valid_password((min, max, letter, password): (usize, usize, char, &str)) -> bool {
    let chars: Vec<char> = password.chars().collect();
    (chars[min - 1] == letter) ^ (chars[max - 1] == letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert!(run() == 530);
    }
}
