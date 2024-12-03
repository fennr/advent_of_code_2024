use std;
mod days;
mod utils;

fn main() -> Result<(), std::io::Error>{
    println!("day 1: {}", days::day01::total_distance("files/01.txt")?);
    println!("day 2: {}", days::day02::save_order_counter("files/02.txt")?);
    println!("day 3: {}", days::day03::parse_mul("files/03.txt")?);
    Ok(())
}
