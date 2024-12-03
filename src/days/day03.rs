use regex::Regex;
use crate::utils;


pub fn parse_mul(filename: &str) -> Result<i32, std::io::Error> {
    let lines = utils::str_from_txt(filename)?;
    let mods = find_all_muls(&lines);
    let out = mods.iter().map(|(a, b)| a * b).sum();
    Ok(out)
}

fn find_all_muls(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let first = cap[1].parse::<i32>().ok()?;
            let second = cap[2].parse::<i32>().ok()?;
            Some((first, second))
        })
        .collect()
}
