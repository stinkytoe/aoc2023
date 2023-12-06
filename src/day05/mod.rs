use std::str::Lines;

use anyhow::{anyhow, bail, Error, Result};

use crate::util::get_two_parts;

pub fn part1(input: &str) -> u64 {
    let Almanac { seeds, maps } = input.try_into().expect("failed to parse input!");

    seeds
        .iter()
        .map(|seed| {
            let mut n = *seed;

            for map in &maps {
                n = map.do_map(n);
            }

            n
        })
        .min()
        .expect("no seeds?")
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<MapSet>,
}

impl TryFrom<&str> for Almanac {
    type Error = Error;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut lines = value.lines();

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
                .map(|seed| Ok(seed.parse::<u64>()?))
                .collect::<Result<_>>()?
        };

        let maps = get_paragraphs(&mut lines)
            .iter()
            .map(|paragraph| paragraph.try_into())
            .collect::<Result<_>>()?;

        Ok(Almanac { seeds, maps })
    }
}

#[derive(Debug)]
struct MapSet {
    _set_name: String,
    ranges: Vec<MapSetRange>,
}

impl MapSet {
    fn do_map(&self, input: u64) -> u64 {
        if let Some(matched_set_range) = self.ranges.iter().find(|&set_range| {
            (set_range.source_range_start..=(set_range.source_range_start + set_range.length - 1)).contains(&input)
        }) {
            input - matched_set_range.source_range_start + matched_set_range.destination_range_start
        } else {
            input
        }
    }
}

impl TryFrom<&Vec<String>> for MapSet {
    type Error = Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        let mut lines = value.iter();

        let (set_name, map_magic) = get_two_parts(lines.next().expect("bad first line in map!"), " ", "map header")?;

        if map_magic != "map:" {
            bail!("bad map magic!")
        }

        let ranges = lines.map(|line| line.as_str().try_into()).collect::<Result<_>>()?;

        Ok(Self {
            _set_name: set_name.to_owned(),
            ranges,
        })
    }
}

#[derive(Debug)]
struct MapSetRange {
    destination_range_start: u64,
    source_range_start: u64,
    length: u64,
}

impl TryFrom<&str> for MapSetRange {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let nums: Vec<u64> = value.split(' ').map(|n| Ok(n.parse::<u64>()?)).collect::<Result<_>>()?;

        if nums.len() != 3 {
            bail!("bad range line!")
        };

        Ok(Self {
            destination_range_start: nums[0],
            source_range_start: nums[1],
            length: nums[2],
        })
    }
}

fn get_paragraph(lines: &mut Lines) -> Vec<String> {
    lines
        .map_while(|line| match !line.is_empty() {
            true => Some(line.to_owned()),
            false => None,
        })
        .collect()
}

fn get_paragraphs(lines: &mut Lines) -> Vec<Vec<String>> {
    let mut accum = Vec::new();

    loop {
        let next = get_paragraph(lines);
        if next.is_empty() {
            return accum;
        } else {
            accum.push(next);
        }
    }
}
