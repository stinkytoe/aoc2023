use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
pub struct ScratchCard {
    _card_number: u32,
    winning_numbers: Vec<u32>,
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

            let winning_numbers = parse_space_separated_ints(winning_numbers)?;
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

pub fn get_two_parts<'a>(
    input: &'a str,
    pattern: &'static str,
    field_name: &'static str,
) -> Result<(&'a str, &'a str)> {
    let mut split = input.split(pattern);

    let first_part = split
        .find(|s| !s.is_empty())
        .ok_or(anyhow!("Malformed {field_name}, no first part: {input}!"))?;

    let second_part = split
        .find(|s| !s.is_empty())
        .ok_or(anyhow!("Malformed {field_name}, no second part: {input}!"))?;

    if split.next().is_some() {
        bail!("Malformed {field_name}, more than two parts: {input}!");
    }

    Ok((first_part, second_part))
}
