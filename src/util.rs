use anyhow::{anyhow, bail, Result};

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
