pub mod day01;
pub mod day02;

fn main() {
    println!("Hello, aoc2023!");
    println!();

    // day01
    println!("day01");
    println!("day01 part1: {}", day01::part1(include_str!("day01/puzzle_input.txt")));
    println!("day01 part2: {}", day01::part2(include_str!("day01/puzzle_input.txt")));

    // day 02
    println!("day02");
    println!("day02 part1: {}", day02::part1(include_str!("day02/puzzle_input.txt")));
    println!("day02 part2: {}", day02::part2(include_str!("day02/puzzle_input.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn day01_test() {
        use crate::day01;

        assert_eq!(day01::part1(include_str!("day01/part1_sample_input.txt")), 142);
        assert_eq!(day01::part2(include_str!("day01/part2_sample_input.txt")), 281);
    }

    #[test]
    fn day02_test() {
        use crate::day02;

        assert_eq!(day02::part1(include_str!("day02/sample_input.txt")), 8);
        assert_eq!(day02::part2(include_str!("day02/sample_input.txt")), 2286);
    }
}
