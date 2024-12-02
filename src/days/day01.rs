use crate::utils;

pub fn total_distance(filename: &str) -> Result<i32, std::io::Error> {
    let lines = utils::slices_from_txt(filename)?;
    let (mut lst1, mut lst2) = (vec![], vec![]);

    for line in lines.iter() {

        if line.len() == 2 {
            lst1.push(line[0]);
            lst2.push(line[1]);
        }
    }

    lst1.sort_unstable();
    lst2.sort_unstable();

    let total: i32 = lst1.iter().zip(lst2.iter()).map(|(a, b)| (a - b).abs()).sum();
    Ok(total)
}
