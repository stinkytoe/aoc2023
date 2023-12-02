pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find_map(|ch| ch.to_digit(10))
                .unwrap_or_else(|| panic!("No first number found in line: \"{line}\""));

            let last = line
                .chars()
                .rev()
                .find_map(|ch| ch.to_digit(10))
                .unwrap_or_else(|| panic!("No last number found in line: \"{line}\""));

            first * 10 + last
        })
        .sum()
}
