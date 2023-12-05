use anyhow::{anyhow, bail, Result};
use std::collections::BTreeSet;

use crate::util::get_two_parts;

#[derive(Debug)]
pub struct ScratchCard {
    _card_number: u32,
    winning_numbers: BTreeSet<u32>,
    given_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn how_many_matches(&self) -> usize {
        self.given_numbers
            .iter()
            .filter(|given_number| self.winning_numbers.contains(given_number))
            .count()
    }

    pub fn score(&self) -> u32 {
        let how_many_matches: u32 = self.how_many_matches() as u32;

        if how_many_matches == 0 {
            0
        } else {
            1 << (how_many_matches - 1)
        }
    }
}

pub fn parse_scratchcards(input: &str) -> Result<Vec<ScratchCard>> {
    input
        .lines()
        .map(|line| {
            let (header, _card) = get_two_parts(line, ": ", "Card")?;

            let (card_magic, card_number) = get_two_parts(header, " ", "card header")?;

            if card_magic != "Card" {
                bail!("Must start with \"Card\"!");
            }

            let card_number = card_number.parse()?;

            let (winning_numbers, given_numbers) = get_two_parts(_card, " | ", "Card Numbers")?;

            let winning_numbers = BTreeSet::from_iter(parse_space_separated_ints(winning_numbers)?.iter().copied());
            let given_numbers = parse_space_separated_ints(given_numbers)?;

            Ok(ScratchCard {
                _card_number: card_number,
                winning_numbers,
                given_numbers,
            })
        })
        .collect::<Result<_>>()
}

fn parse_space_separated_ints(input: &str) -> Result<Vec<u32>> {
    input
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| Ok(n.parse::<u32>()?))
        .collect::<Result<Vec<u32>>>()
}
