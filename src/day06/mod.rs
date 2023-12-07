use std::iter::zip;

use anyhow::{anyhow, bail, Result};

pub fn part1(input: &str) -> u64 {
    let races = parse_input(input).expect("couldn't parse input!");

    races.iter().map(|race| race.calc_winners()).product()
}

pub fn part2(input: &str) -> u64 {
    parse_input_bad_kerning(input)
        .expect("couldn't parse input!")
        .calc_winners()
}

fn parse_input(input: &str) -> Result<Vec<Race>> {
    let mut lines = input.lines();

    let times = parse_line_of_header_and_numbers(lines.next().ok_or(anyhow!("no first line?"))?, "Time:")?;

    let distances = parse_line_of_header_and_numbers(lines.next().ok_or(anyhow!("no second line?"))?, "Distance:")?;

    Ok(zip(times, distances).map(|pair| pair.into()).collect())
}

fn parse_line_of_header_and_numbers(input: &str, header: &str) -> Result<Vec<u64>> {
    let mut tokens = input.split(' ').filter(|token| !token.is_empty());

    if tokens.next().ok_or(anyhow!("expected content in line"))? != header {
        bail!("Line should start with \"{header}\"!");
    };

    tokens
        .map(|time| Ok(time.parse::<u64>()?))
        .collect::<Result<Vec<u64>>>()
}

fn parse_input_bad_kerning(input: &str) -> Result<Race> {
    let mut lines = input.lines();

    let time = parse_line_bad_kerning(lines.next().ok_or(anyhow!("no first line?"))?, "Time:")?;

    let distance = parse_line_bad_kerning(lines.next().ok_or(anyhow!("no first line?"))?, "Distance:")?;

    Ok(Race { time, distance })
}

fn parse_line_bad_kerning(input: &str, header: &str) -> Result<u64> {
    let mut tokens = input.split(' ').filter(|token| !token.is_empty());

    if tokens.next().ok_or(anyhow!("expected content in line"))? != header {
        bail!("Line should start with \"{header}\"!");
    };

    Ok(tokens
        .flat_map(|token| token.chars())
        .collect::<String>()
        .parse::<u64>()?)
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn calc_winners(&self) -> u64 {
        let Self { time, distance } = self;

        (1..=(*time - 1)).filter(|i| ((*time - i) * i) > *distance).count() as u64
    }
}

impl From<(u64, u64)> for Race {
    fn from((time, distance): (u64, u64)) -> Self {
        Self { time, distance }
    }
}
