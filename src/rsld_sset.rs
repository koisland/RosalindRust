use std::{error::Error, fs};

pub fn count_subsets(fname: &str) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let n: u32 = contents.trim().parse()?;
    let base: u32 = 2;
    let mut subsets: u32 = 1;

    // https://byjus.com/maths/subsets/
    // 2 ^ n
    for _ in 0..n {
        subsets = (subsets * base) % 1_000_000;
    }

    // Both proper and improper subsets
    Ok(subsets)
}
