mod cube_set;
mod game_run;
pub(crate) mod util;

use anyhow::Result;

use crate::day02::game_run::GameRun;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(GameRun::try_from)
        .collect::<Result<Vec<GameRun>>>()
        .expect("We failed to parse the input! {e}")
        .iter()
        .filter(|game_run| game_run.could_contain(&util::BAG_CONTENTS))
        .map(|game_run| game_run.get_game_id())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(GameRun::try_from)
        .collect::<Result<Vec<GameRun>>>()
        .expect("We failed to parse the input! {e}")
        .iter()
        .map(|game_run| game_run.calc_min_set().pow())
        .sum()
}
