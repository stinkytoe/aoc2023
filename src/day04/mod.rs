mod util;

use crate::day04::util::parse_scratchcards;

pub fn part1(input: &str) -> u32 {
    parse_scratchcards(input)
        .expect("failed to parse input!")
        .iter()
        .map(|card| card.score())
        .sum()
}
