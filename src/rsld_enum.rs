use std::fs;
use itertools::Itertools;

pub fn signed_perm(fname: &str) -> String {
    let contents: i8 = fs::read_to_string(fname)
        .expect("Unable to read file.")
        .trim()
        .parse()
        .expect("Unable to parse value to usize.");

    let possible_perms: Vec<Vec<i8>> = (1..=contents)
        // Add signed values
        .chain(-contents..=-1)
        // Permutations length same as contents
        .permutations(contents as usize)
        // Remove permutations that have duplicate values.
        .filter(|x| x.iter().map(|x| x.abs()).all_unique())
        .collect_vec();

    let string_possible_perms: String = possible_perms
        .iter()
        .map(|perm| perm.iter().join(" "))
        .join("\n");

    let total_perms = format!("{}\n", &possible_perms.len().to_string());
    let mut res = String::new();
    res.push_str(&total_perms);
    res.push_str(&string_possible_perms);

    res
}