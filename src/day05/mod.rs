use std::str::Lines;

use anyhow::{anyhow, bail, Error, Result};

use crate::util::get_two_parts;

pub fn part1(input: &str) -> u32 {
    let almanac: Almanac = input.try_into().expect("failed to parse input!");

    todo!()
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<MapSet>,
}

impl TryFrom<&str> for Almanac {
    type Error = Error;

    fn try_from(input: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut lines = input.lines();

        let seeds = {
            let (seed_magic, seeds) = get_two_parts(
                lines.next().ok_or(anyhow!("failed to get seed line!"))?,
                ": ",
                "seed line",
            )?;

            lines.next();

            if seed_magic != "seeds" {
                bail!("bad magic header, expected \"seed\"!")
            }

            seeds
                .split(' ')
                .map(|seed| Ok(seed.parse::<u32>()?))
                .collect::<Result<_>>()?
        };

        let maps = {};

        Ok(Almanac {
            seeds,
            maps: Vec::default(),
        })
    }
}

struct MapSet {
    set_name: String,
    ranges: Vec<MapSetRange>,
}

impl TryFrom<&'_ Lines<'_>> for MapSet {
    type Error = Error;

    fn try_from(value: &Lines) -> std::prelude::v1::Result<Self, Self::Error> {
        todo!()
    }
}

struct MapSetRange {
    destination_range_start: u32,
    source_range_start: u32,
    length: u32,
}
