use std::iter::zip;

use anyhow::{anyhow, bail, Result};

pub fn part1(input: &str) -> u32 {
    let races = parse_input(input).expect("couldn't parse input!");

    races.iter().map(|race| race.calc_winners()).product()
}

fn parse_input(input: &str) -> Result<Vec<Race>> {
    let mut lines = input.lines();

    let times = parse_line_of_header_and_numbers(lines.next().ok_or(anyhow!("no first line?"))?, "Time:")?;

    let distances = parse_line_of_header_and_numbers(lines.next().ok_or(anyhow!("no second line?"))?, "Distance:")?;

    Ok(zip(times, distances).map(|pair| pair.into()).collect())
}

fn parse_line_of_header_and_numbers(input: &str, header: &str) -> Result<Vec<u32>> {
    // let mut times = lines
    //     .next()
    //     .ok_or(anyhow!("no first line?"))?
    let mut times = input.split(' ').filter(|token| !token.is_empty());

    if times.next().ok_or(anyhow!("expected content in line"))? != header {
        bail!("First line should start with \"{header}\"!");
    };

    times.map(|time| Ok(time.parse::<u32>()?)).collect::<Result<Vec<u32>>>()
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn calc_winners(&self) -> u32 {
        let Self { time, distance } = self;

        (1..=(*time - 1)).filter(|i| ((*time - i) * i) > *distance).count() as u32
    }
}

impl From<(u32, u32)> for Race {
    fn from((time, distance): (u32, u32)) -> Self {
        Self { time, distance }
    }
}
