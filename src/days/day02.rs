use crate::utils;

pub fn save_order_counter(filename: &str) -> Result<i32, std::io::Error> {
    utils::slices_from_txt(filename)?
        .into_iter()
        .filter(|line| ascending_order(line) || descending_order(line))
        .count()
        .try_into()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Count overflow"))
}


fn descending_order(numbers: &[i32]) -> bool {
    numbers.windows(2).all(|pair| pair[0] > pair[1] && ((pair[0] - pair[1]) >= 1 && (pair[0] - pair[1]) <= 3))
}

fn ascending_order(numbers: &[i32]) -> bool {
    numbers.windows(2).all(|pair| pair[0] < pair[1] && ((pair[1] - pair[0]) >= 1 && (pair[1] - pair[0]) <= 3))
}