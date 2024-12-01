use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn total_distance(filename: &str) -> Result<i32, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let (mut lst1, mut lst2) = (vec![], vec![]);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            lst1.push(numbers[0]);
            lst2.push(numbers[1]);
        }
    }

    lst1.sort_unstable();
    lst2.sort_unstable();

    let total: i32 = lst1.iter().zip(lst2.iter()).map(|(a, b)| (a - b).abs()).sum();
    Ok(total)
}
