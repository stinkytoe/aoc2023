use anyhow::{bail, Error, Result};

use crate::day02::cube_set::CubeSet;
use crate::day02::util::get_two_parts;

pub struct GameRun {
    game_id: u32,
    cube_sets: Vec<CubeSet>,
}

impl GameRun {
    pub fn could_contain(&self, rhs: &CubeSet) -> bool {
        !self.cube_sets.iter().any(|cube_set| !cube_set.could_contain(rhs))
    }

    pub fn get_game_id(&self) -> u32 {
        self.game_id
    }

    pub fn calc_min_set(&self) -> CubeSet {
        let mut min_set = CubeSet::default();

        for cube_set in &self.cube_sets {
            min_set.red = std::cmp::max(min_set.red, cube_set.red);
            min_set.green = std::cmp::max(min_set.green, cube_set.green);
            min_set.blue = std::cmp::max(min_set.blue, cube_set.blue);
        }

        min_set
    }
}

impl TryFrom<&str> for GameRun {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (game_header, cube_sets) = get_two_parts(input, ": ", "game run")?;

        let game_id = {
            let (magic_str, game_id) = get_two_parts(game_header, " ", "game header")?;

            if !magic_str.eq("Game") {
                bail!("Malformed game header, must start with \"Game\", given {input}!");
            }

            game_id.parse::<u32>()?
        };

        let cube_sets: Vec<CubeSet> = cube_sets.split("; ").map(CubeSet::try_from).collect::<Result<_>>()?;

        Ok(GameRun { game_id, cube_sets })
    }
}
