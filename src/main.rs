pub mod day01;

fn main() {
    use crate::day01::part1;
    use crate::day01::part2;

    println!("Hello, aoc2023!");
    println!(
        "day01 part1: {}",
        part1(include_str!("day01/puzzle_input.txt"))
    );
    println!(
        "day01 part2: {}",
        part2(include_str!("day01/puzzle_input.txt"))
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn day01_test() {
        use crate::day01::part1;

        assert_eq!(part1(include_str!("day01/part1_sample_input.txt")), 142);
    }

    #[test]
    fn day02_test() {
        use crate::day01::part2;

        assert_eq!(part2(include_str!("day01/part2_sample_input.txt")), 281);
    }
}
