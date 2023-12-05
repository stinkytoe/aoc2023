mod util;

use crate::day04::util::parse_scratchcards;

pub fn part1(input: &str) -> u32 {
    parse_scratchcards(input)
        .expect("failed to parse input!")
        .iter()
        .map(|card| card.score())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let scores: Vec<usize> = parse_scratchcards(input)
        .expect("failed to parse input!")
        .iter()
        .map(|card| card.how_many_matches())
        .collect();

    let mut card_counts = vec![1; scores.len()];

    for i in 0..(scores.len()) {
        let current_score = scores[i];
        let current_count = card_counts[i];
        for j in (i + 1)..usize::min(i + 1 + current_score, card_counts.len()) {
            card_counts[j] += current_count;
        }
    }

    card_counts.iter().sum::<usize>() as u32
}
