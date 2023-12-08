mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

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

    // day 03
    println!("day03");
    println!("day03 part1: {}", day03::part1(include_str!("day03/puzzle_input.txt")));
    println!("day03 part2: {}", day03::part2(include_str!("day03/puzzle_input.txt")));

    // day 04
    println!("day04");
    println!("day04 part1: {}", day04::part1(include_str!("day04/puzzle_input.txt")));
    println!("day04 part2: {}", day04::part2(include_str!("day04/puzzle_input.txt")));

    // day 05
    println!("day05");
    println!("day05 part1: {}", day05::part1(include_str!("day05/puzzle_input.txt")));
    // println!("day05 part2: {}", day05::part2(include_str!("day05/puzzle_input.txt")));
    println!("day05 part2: (not run because it's waaayyy to slow. answer was: 6472060)");

    // day 06
    println!("day06");
    println!("day06 part1: {}", day06::part1(include_str!("day06/puzzle_input.txt")));
    println!("day06 part2: {}", day06::part2(include_str!("day06/puzzle_input.txt")));

    // day 06
    println!("day07");
    println!("day07 part1: {}", day07::part1(include_str!("day07/puzzle_input.txt")));
    // println!("day07 part2: {}", day07::part2(include_str!("day07/puzzle_input.txt")));
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

    #[test]
    fn day03_test() {
        use crate::day03;

        assert_eq!(day03::part1(include_str!("day03/sample_input.txt")), 4361);
        assert_eq!(day03::part2(include_str!("day03/sample_input.txt")), 467835);
    }

    #[test]
    fn day04_test() {
        use crate::day04;

        assert_eq!(day04::part1(include_str!("day04/sample_input.txt")), 13);
        assert_eq!(day04::part2(include_str!("day04/sample_input.txt")), 30);
    }

    #[test]
    fn day05_test() {
        use crate::day05;

        assert_eq!(day05::part1(include_str!("day05/sample_input.txt")), 35);
        assert_eq!(day05::part2(include_str!("day05/sample_input.txt")), 46);
    }

    #[test]
    fn day06_test() {
        use crate::day06;

        assert_eq!(day06::part1(include_str!("day06/sample_input.txt")), 288);
        assert_eq!(day06::part2(include_str!("day06/sample_input.txt")), 71503);
    }

    #[test]
    fn day07_test() {
        use crate::day07;

        assert_eq!(day07::part1(include_str!("day07/sample_input.txt")), 6440);
        // assert_eq!(day07::part2(include_str!("day07/sample_input.txt")), 71503);
    }
}
