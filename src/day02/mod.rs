mod cube_set;
mod game_run;
mod util;

use crate::day02::cube_set::CubeSet;

pub fn part1(input: &str, bag_contents: &str) -> u32 {
    use anyhow::Result;

    use crate::day02::game_run::GameRun;

    let game_runs = input
        .lines()
        .map(GameRun::try_from)
        .collect::<Result<Vec<GameRun>>>()
        .expect("We failed to parse the input! {e}");

    let bag_contents =
        CubeSet::try_from(bag_contents).expect("We failed to parse the supplied bag contents! {e}");

    game_runs
        .iter()
        .filter(|game_run| game_run.could_contain(&bag_contents))
        .map(|game_run| game_run.get_game_id())
        .sum()
}
