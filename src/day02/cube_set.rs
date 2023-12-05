use anyhow::{bail, Error, Result};

use crate::util::get_two_parts;

#[derive(Default, Debug)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub fn could_contain(&self, rhs: &CubeSet) -> bool {
        self.red <= rhs.red && self.green <= rhs.green && self.blue <= rhs.blue
    }

    pub fn pow(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl TryFrom<&str> for CubeSet {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut new_cube_set = CubeSet::default();

        let color_specs: Vec<(u32, &str)> = input
            .split(", ")
            .map(|color_spec| {
                let (count, color) = get_two_parts(color_spec, " ", "color spec")?;
                Ok((count.parse::<u32>()?, color))
            })
            .collect::<Result<_>>()?;

        for (count, color) in color_specs {
            match color {
                "red" => new_cube_set.red = count,
                "green" => new_cube_set.green = count,
                "blue" => new_cube_set.blue = count,
                _ => bail!("Malformed color spec, unknown color key: {color}!"),
            }
        }

        Ok(new_cube_set)
    }
}
