use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn slices_from_txt(filename: &str) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut out = vec![];

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        out.push(numbers);
    }
    Ok(out)
}

pub fn str_from_txt(filename: &str) -> Result<String, std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
