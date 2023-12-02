use std::collections::HashMap;

pub fn part2(input: &str) -> u32 {
    let number_str_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| {
                    number_str_map
                        .keys()
                        .find(|key| line[i..].starts_with(*key))
                })
                .unwrap_or_else(|| panic!("No first number found in line: \"{line}\""));

            let last = (0..line.len())
                .find_map(|i| {
                    number_str_map
                        .keys()
                        .find(|key| line[..(line.len() - i)].ends_with(*key))
                })
                .unwrap_or_else(|| panic!("No last number found in line: \"{line}\""));

            number_str_map[first] * 10 + number_str_map[last]
        })
        .sum()
}
