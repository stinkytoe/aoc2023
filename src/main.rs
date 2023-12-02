pub mod day01;

fn main() {
    use crate::day01::part01;
    use crate::day01::part02;

    println!("Hello, aoc2023!");
    println!("part01: {}", part01(include_str!("day01/puzzle_input.txt")));
    println!("part02: {}", part02(include_str!("day01/puzzle_input.txt")));
}

#[cfg(test)]
mod tests {

    #[test]
    fn day01_test() {
        use crate::day01::part01;
        use crate::day01::part02;

        assert_eq!(part01(include_str!("day01/part01_sample_input.txt")), 142);
        assert_eq!(part02(include_str!("day01/part02_sample_input.txt")), 281);
    }

    #[test]
    fn day02_test() {}
}
